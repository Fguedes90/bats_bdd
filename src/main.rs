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

    /// Transpile a feature file to BATS and execute it
    Run {
        /// Path to the .feature file
        file: PathBuf,

        /// Path to step definitions bash file (default: step_definitions.bash)
        #[arg(short, long, default_value = "step_definitions.bash")]
        steps: PathBuf,

        /// Output directory for generated .bats file (default: same as .feature)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Show detailed output including generated BATS code
        #[arg(short, long)]
        verbose: bool,
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
            file,
            steps,
            output,
            verbose,
        } => {
            // 1. Parse Gherkin
            let content = std::fs::read_to_string(&file)?;
            let feature = parser::parse_feature(&content)?;

            // 2. Determine steps filename (will be copied to output dir)
            let steps_filename = steps
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("step_definitions.bash");

            // 3. Transpile to BATS (use filename only since steps will be in same dir)
            let bats_code = bats::transpile_to_bats(&feature, steps_filename).map_err(|e| {
                eprintln!("Error: {}", e);
                std::io::Error::new(std::io::ErrorKind::InvalidData, e)
            })?;

            // 4. Determine output path
            let output_dir = output
                .or_else(|| file.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("."));

            let bats_filename = format!(
                "{}.bats",
                file.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output")
            );
            let output_path = output_dir.join(&bats_filename);

            // 5. Create output directory if it doesn't exist
            if !output_dir.exists() {
                std::fs::create_dir_all(&output_dir)?;
            }

            // 6. Write generated BATS file
            std::fs::write(&output_path, &bats_code)?;

            // 7. Copy step definitions to output directory if needed
            let dest_steps_path = output_dir.join(steps_filename);

            // Copy if: output was specified OR steps has a parent directory
            let steps_dir = steps
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."));
            let needs_copy = output_dir != steps_dir;
            if needs_copy && steps.exists() {
                // Only copy if destination is different from source
                let src_canonical = steps.canonicalize().ok();
                let dst_canonical = dest_steps_path.canonicalize().ok();
                if src_canonical != dst_canonical {
                    std::fs::copy(&steps, &dest_steps_path)?;
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

            // 8. Check if BATS is installed
            let bats_check = std::process::Command::new("bats").arg("--version").output();

            if bats_check.is_err() {
                eprintln!("✗ Error: BATS CLI not found!");
                eprintln!();
                eprintln!(
                    "Install BATS: https://bats-core.readthedocs.io/en/stable/Installation.html"
                );
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

            // 9. Execute with official BATS CLI
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
        }
    }

    Ok(())
}
