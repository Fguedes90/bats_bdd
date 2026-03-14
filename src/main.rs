//! BATS-BDD CLI: Command-line interface for the BATS-BDD transpiler
//!
//! This binary transforms Gherkin feature files into native BATS bash scripts
//! and executes them using the official BATS CLI.

use bats_bdd_rust::compiler::bats;
use bats_bdd_rust::parser;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "bats-bdd")]
#[command(about = "Gherkin to BATS transpiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a Gherkin feature file and display its AST structure
    Parse {
        /// Path to the .feature file
        file: PathBuf,
    },

    /// Transpile a feature file (or directory) to BATS and execute it
    Run {
        /// Path to the .feature file or directory containing .feature files
        #[arg(value_parser = clap::value_parser!(std::path::PathBuf))]
        path: PathBuf,

        /// Path to step definitions bash file (default: step_definitions.bash)
        #[arg(short, long, default_value = "step_definitions.bash")]
        steps: PathBuf,

        /// Output directory for generated .bats file (default: same as .feature)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Show detailed output including generated BATS code
        #[arg(short, long)]
        verbose: bool,

        /// Run BATS tests in parallel
        #[arg(short, long)]
        parallel: bool,
    },

    /// Install the BDD Gherkin skill for better agent guidance
    InstallSkill {
        /// Target directory (default: current directory)
        #[arg(short, long)]
        directory: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { file } => {
            let content = std::fs::read_to_string(&file)?;
            match parser::parse_feature(&content) {
                Ok(feature) => {
                    println!("Feature: {}", feature.name);
                    if let Some(desc) = &feature.description {
                        println!("{}", desc);
                    }
                    println!();
                    println!("Scenarios: {}", feature.scenarios.len());
                    for scenario in &feature.scenarios {
                        match scenario {
                            parser::ast::Scenario::Simple(s) => {
                                println!("  - {}", s.name);
                                println!("    Steps: {}", s.steps.len());
                            }
                            parser::ast::Scenario::Outline(o) => {
                                println!("  - {} (outline)", o.name);
                                println!("    Steps: {}", o.steps.len());
                                println!(
                                    "    Examples: {} row(s)",
                                    o.examples.table.rows.len().saturating_sub(1)
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Parse error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Run {
            path,
            steps,
            output,
            verbose,
            parallel,
        } => {
            // Check if path is a file or directory
            let path = path.canonicalize()?;
            if path.is_dir() {
                run_directory(&path, &steps, output.as_deref(), verbose, parallel)?;
            } else {
                run_file(&path, &steps, output.as_deref(), verbose)?;
            }
        }

        Commands::InstallSkill { directory } => {
            install_skill(directory)?;
        }
    }

    Ok(())
}

/// Run a single .feature file
fn run_file(
    file: &std::path::Path,
    steps: &std::path::Path,
    output: Option<&std::path::Path>,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse Gherkin
    let content = std::fs::read_to_string(file)?;
    let feature = parser::parse_feature(&content)?;

    // 2. Determine steps filename
    let steps_filename = steps
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("step_definitions.bash");

    // 3. Transpile to BATS
    let bats_code = bats::transpile_to_bats(&feature, steps_filename).map_err(|e| {
        eprintln!("Error: {}", e);
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;

    // 4. Determine output path
    let output_dir = output
        .map(|p| p.to_path_buf())
        .or_else(|| file.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    let bats_filename = format!(
        "{}.bats",
        file.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output")
    );
    let output_path = output_dir.join(&bats_filename);

    // 5. Create output directory if needed
    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir)?;
    }

    // 6. Write generated BATS file
    std::fs::write(&output_path, &bats_code)?;

    // 7. Sync step definitions
    let dest_steps_path = output_dir.join(steps_filename);
    match bats::sync_step_definitions(&feature, &dest_steps_path) {
        Ok((new_steps, orphans)) => {
            for step in &new_steps {
                if verbose {
                    println!("✓ Added step definition: {}", step);
                } else {
                    println!("✓ Added step: {}", step);
                }
            }
            if !orphans.is_empty() {
                eprintln!();
                eprintln!("⚠ Warning: The following step definitions exist but are not used in the feature:");
                for orphan in &orphans {
                    eprintln!("  - {}", orphan);
                }
                eprintln!();
                eprintln!("These may be leftovers from a previous feature version.");
                eprintln!("Remove them from step_definitions.bash or update your .feature file.");
                eprintln!();
            }
        }
        Err(e) => {
            eprintln!("Error syncing step definitions: {}", e);
        }
    }

    // 8. Copy step definitions if needed
    let steps_dir = steps
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let needs_copy = output_dir != steps_dir;
    if needs_copy && steps.exists() {
        let src_canonical = steps.canonicalize().ok();
        let dst_canonical = dest_steps_path.canonicalize().ok();
        if src_canonical != dst_canonical {
            std::fs::copy(steps, &dest_steps_path)?;
            if verbose {
                println!(
                    "✓ Copied step definitions to: {}",
                    dest_steps_path.display()
                );
            }
        }
    }

    if verbose {
        println!("✓ Generated BATS file: {}", output_path.display());
        println!("\n--- Generated BATS code ---");
        println!("{}", bats_code);
        println!("--- End BATS code ---\n");
    } else {
        println!("✓ Generated: {}", output_path.display());
    }

    // 9. Check if BATS is installed
    let bats_check = std::process::Command::new("bats").arg("--version").output();
    if bats_check.is_err() {
        eprintln!("✗ Error: BATS CLI not found!");
        eprintln!();
        eprintln!("Install BATS: https://bats-core.readthedocs.io/en/stable/Installation.html");
        eprintln!();
        eprintln!("  macOS:");
        eprintln!("    brew install bats");
        eprintln!();
        eprintln!("  Linux (Debian/Ubuntu):");
        eprintln!("    sudo apt install bats");
        eprintln!();
        eprintln!("  Linux (RHEL/Fedora):");
        eprintln!("    sudo dnf install bats");
        eprintln!();
        std::process::exit(1);
    }

    // 10. Execute with BATS CLI
    if verbose {
        println!("Executing BATS tests...");
    }

    let status = std::process::Command::new("bats")
        .arg(&output_path)
        .status()?;

    if !status.success() {
        eprintln!("\n✗ BATS tests failed");
        std::process::exit(status.code().unwrap_or(1));
    }

    if verbose {
        println!("\n✓ All BATS tests passed!");
    }

    Ok(())
}

/// Run all .feature files in a directory
fn run_directory(
    dir: &std::path::Path,
    steps: &std::path::Path,
    output: Option<&std::path::Path>,
    verbose: bool,
    parallel: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Collect all .feature files
    let mut feature_files: Vec<std::path::PathBuf> = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "feature") {
            feature_files.push(path);
        }
    }

    if feature_files.is_empty() {
        eprintln!("No .feature files found in {}", dir.display());
        std::process::exit(1);
    }

    // Sort for consistent ordering
    feature_files.sort();

    // Check for duplicate filenames (stem conflicts)
    let mut stems: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut duplicates: Vec<String> = Vec::new();
    for f in &feature_files {
        let stem = f.file_stem().unwrap().to_string_lossy().to_string();
        if !stems.insert(stem.clone()) {
            duplicates.push(stem);
        }
    }
    if !duplicates.is_empty() {
        eprintln!("Error: Duplicate .feature filenames found:");
        for dup in &duplicates {
            eprintln!("  - {}.feature", dup);
        }
        eprintln!("\nRename files to have unique names before running.");
        std::process::exit(1);
    }

    println!(
        "Found {} .feature file(s) in {}",
        feature_files.len(),
        dir.display()
    );

    // 2. Determine output directory
    let output_dir = output
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| dir.to_path_buf());

    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir)?;
    }

    // 3. Parse all features first (to collect all steps)
    let mut all_features: Vec<(std::path::PathBuf, parser::ast::Feature)> = Vec::new();
    let mut parse_errors: Vec<(String, String)> = Vec::new();

    for feature_path in &feature_files {
        match std::fs::read_to_string(feature_path) {
            Ok(content) => match parser::parse_feature(&content) {
                Ok(feature) => {
                    all_features.push((feature_path.clone(), feature));
                }
                Err(e) => {
                    parse_errors.push((feature_path.to_string_lossy().to_string(), e.to_string()));
                }
            },
            Err(e) => {
                parse_errors.push((feature_path.to_string_lossy().to_string(), e.to_string()));
            }
        }
    }

    if !parse_errors.is_empty() {
        eprintln!("Failed to parse {} feature file(s):", parse_errors.len());
        for (path, err) in &parse_errors {
            eprintln!("  - {}: {}", path, err);
        }
        std::process::exit(1);
    }

    // 4. Generate .bats files (without running yet)
    let steps_filename = steps
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("step_definitions.bash");

    for (feature_path, feature) in &all_features {
        let bats_code = bats::transpile_to_bats(feature, steps_filename).map_err(|e| {
            eprintln!("Error transpiling {}: {}", feature_path.display(), e);
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?;

        let bats_filename = format!(
            "{}.bats",
            feature_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output")
        );
        let output_path = output_dir.join(&bats_filename);
        std::fs::write(&output_path, &bats_code)?;

        println!("✓ Generated: {}", output_path.display());
    }

    // 5. Aggregate all steps from all features for sync
    let mut aggregated_steps: Vec<String> = Vec::new();
    for (_, feature) in &all_features {
        let feature_steps = bats::extract_step_function_names(feature);
        for step in feature_steps {
            if !aggregated_steps.contains(&step) {
                aggregated_steps.push(step);
            }
        }
    }

    // 6. Sync step definitions with aggregated steps
    let dest_steps_path = output_dir.join(steps_filename);
    let mut all_new_steps: Vec<String> = Vec::new();
    let mut all_orphans: Vec<String> = Vec::new();

    if !dest_steps_path.exists() {
        // Create new step definitions file with all aggregated stubs
        let mut content = String::new();
        content.push_str("#!/usr/bin/env bash\n");
        content.push_str("# Auto-generated step definitions\n");
        content.push_str("# Add your step implementations below\n\n");

        for step_func_name in &aggregated_steps {
            let stub = bats::generate_step_stub(
                step_func_name,
                &format!(
                    "[Step matching '{}']",
                    step_func_name.trim_start_matches("step_")
                ),
            );
            content.push_str(&stub);
            all_new_steps.push(step_func_name.clone());
        }

        std::fs::write(&dest_steps_path, content)?;
    } else {
        // Merge with existing steps
        let existing_steps = bats::parse_existing_steps(&dest_steps_path)?;

        // Find new steps needed
        for step_func_name in &aggregated_steps {
            if !existing_steps.contains(step_func_name) {
                all_new_steps.push(step_func_name.clone());
            }
        }

        // Find orphans (exist in .bash but not in any .feature)
        for existing_step in &existing_steps {
            if !aggregated_steps.contains(existing_step) {
                all_orphans.push(existing_step.clone());
            }
        }

        // Append new stubs
        if !all_new_steps.is_empty() {
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(&dest_steps_path)?;
            use std::io::Write;
            for step_func_name in &all_new_steps {
                let stub = bats::generate_step_stub(
                    step_func_name,
                    &format!(
                        "[Step matching '{}']",
                        step_func_name.trim_start_matches("step_")
                    ),
                );
                file.write_all(stub.as_bytes())?;
            }
        }
    }

    // Print results
    if !all_new_steps.is_empty() {
        println!("\n✓ Added {} new step definition(s)", all_new_steps.len());
        if verbose {
            for step in &all_new_steps {
                println!("  + {}", step);
            }
        }
    }

    if !all_orphans.is_empty() {
        eprintln!();
        eprintln!(
            "⚠ Warning: {} orphan step definition(s) found",
            all_orphans.len()
        );
        eprintln!("These exist in step_definitions.bash but are not used in any .feature file:");
        for orphan in &all_orphans {
            eprintln!("  - {}", orphan);
        }
        eprintln!();
    }

    // 7. Copy step definitions if needed
    let steps_dir = steps
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let needs_copy = output_dir != steps_dir;
    if needs_copy && steps.exists() {
        let src_canonical = steps.canonicalize().ok();
        let dst_canonical = dest_steps_path.canonicalize().ok();
        if src_canonical != dst_canonical {
            std::fs::copy(steps, &dest_steps_path)?;
            if verbose {
                println!(
                    "✓ Copied step definitions to: {}",
                    dest_steps_path.display()
                );
            }
        }
    }

    // 8. Check if BATS is installed
    let bats_check = std::process::Command::new("bats").arg("--version").output();
    if bats_check.is_err() {
        eprintln!("✗ Error: BATS CLI not found!");
        eprintln!("Install BATS: https://bats-core.readthedocs.io/en/stable/Installation.html");
        std::process::exit(1);
    }

    // 9. Execute all generated .bats files
    println!();
    if parallel {
        println!("Running {} test file(s) in parallel...", all_features.len());
    } else {
        println!("Running {} test file(s)...", all_features.len());
    }

    let mut failed_files: Vec<String> = Vec::new();
    let mut total_tests = 0;
    let mut passed_tests = 0;

    // Collect all .bats files
    let mut bats_files: Vec<std::path::PathBuf> = Vec::new();
    for entry in std::fs::read_dir(&output_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "bats") {
            bats_files.push(path);
        }
    }
    bats_files.sort();

    if parallel {
        // Run in parallel using threads
        use std::process::Command;
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();
        let mut handles = Vec::new();

        for bats_file in &bats_files {
            let tx = tx.clone();
            let bats_file = bats_file.clone();
            let handle = thread::spawn(move || {
                let output = Command::new("bats").arg(&bats_file).output();
                tx.send((bats_file, output)).unwrap();
            });
            handles.push(handle);
        }

        drop(tx);

        for result in rx {
            let (bats_file, output) = result;
            total_tests += 1;
            match output {
                Ok(out) => {
                    if out.status.success() {
                        passed_tests += 1;
                        println!("✓ {}", bats_file.display());
                    } else {
                        failed_files.push(bats_file.to_string_lossy().to_string());
                        eprintln!("✗ {}", bats_file.display());
                    }
                }
                Err(e) => {
                    failed_files.push(bats_file.to_string_lossy().to_string());
                    eprintln!("✗ {}: {}", bats_file.display(), e);
                }
            }
        }

        for handle in handles {
            let _ = handle.join();
        }
    } else {
        // Sequential execution
        for bats_file in &bats_files {
            total_tests += 1;
            let output = std::process::Command::new("bats").arg(bats_file).output()?;
            if output.status.success() {
                passed_tests += 1;
                println!("✓ {}", bats_file.display());
            } else {
                failed_files.push(bats_file.to_string_lossy().to_string());
                eprintln!("✗ {}", bats_file.display());
                // Print stderr for debugging
                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
    }

    println!();
    println!(
        "Results: {}/{} test file(s) passed",
        passed_tests, total_tests
    );

    if !failed_files.is_empty() {
        eprintln!("\n✗ {} test file(s) failed:", failed_files.len());
        for f in &failed_files {
            eprintln!("  - {}", f);
        }
        std::process::exit(1);
    }

    println!("\n✓ All BATS tests passed!");
    Ok(())
}

fn install_skill(directory: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = directory.unwrap_or_else(|| PathBuf::from("."));
    let omp_skills_dir = target_dir.join(".omp/skills/bdd-gherkin");

    println!(
        "Installing BDD Gherkin skill to: {}",
        omp_skills_dir.display()
    );

    // Create .omp/skills directory structure
    std::fs::create_dir_all(&omp_skills_dir)?;

    // Get the skill content from the embedded resource
    let skill_content = include_str!("../skills/bdd-gherkin/SKILL.md");

    // Write the skill file
    let skill_path = omp_skills_dir.join("SKILL.md");
    std::fs::write(&skill_path, skill_content)?;

    println!("✓ Skill installed successfully!");
    println!();
    println!("The skill is now available at: {}", skill_path.display());
    println!();
    println!("To use with Oh My Pi:");
    println!("  1. Restart Oh My Pi or reload skills");
    println!("  2. The agent will now have access to BDD/Gherkin guidance");
    println!();
    println!("The skill provides:");
    println!("  - Gherkin syntax reference");
    println!("  - Writing good scenarios guide");
    println!("  - bats-bdd usage instructions");
    println!("  - Step definitions templates");
    println!("  - Common pitfalls to avoid");
    println!("  - Checklists for review");

    Ok(())
}
