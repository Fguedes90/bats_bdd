# BATS-BDD

A BDD (Behavior-Driven Development) library for Rust that integrates with BATS (Bash Automated Testing System).

BATS-BDD transpiles Gherkin feature files into native BATS bash scripts, enabling you to write tests in natural language while running them through BATS.

## Installation

### Homebrew (macOS / Linux)

```bash
# Add the tap
brew tap Fguedes90/bats-bdd

# Install
brew install bats-bdd
```

### Install Script

```bash
curl -sSL https://raw.githubusercontent.com/Fguedes90/bats-bdd/main/install.sh | sh

# Or with specific version
VERSION=0.1.0 curl -sSL https://raw.githubusercontent.com/Fguedes90/bats-bdd/main/install.sh | sh
```

### Build from Source

**Prerequisites:**

- [Rust](https://rustup.rs/) (1.75+)
- [BATS](https://bats-core.readthedocs.io/) (Bash Automated Testing System)

**Installing BATS:**

```bash
# macOS
brew install bats

# Linux (Debian/Ubuntu)
sudo apt install bats

# Linux (RHEL/Fedora)
sudo dnf install bats
```

**Building:**

```bash
# Clone the repository
git clone <repo-url>
cd bats-bdd-rust

# Build
cargo build --release

# The binary will be at target/release/bats-bdd
```

## Usage

### `parse` Command

Parses a `.feature` file and displays its AST structure:

```bash
bats-bdd parse examples/calculator.feature
```

### `run` Command

Transpiles a `.feature` file to BATS and executes it:

```bash
bats-bdd run examples/calculator.feature
```

Available options:

- `-s, --steps <FILE>` — Step definitions file (default: `step_definitions.bash`)
- `-o, --output <DIR>` — Output directory for the `.bats` file
- `-v, --verbose` — Shows generated BATS code

Example with verbose:

```bash
bats-bdd run examples/calculator.feature --verbose
```


### `install-skill` Command

Installs the BDD Gherkin skill in your project to help agents write better `.feature` files:

```bash
bats-bdd install-skill
```

This creates a `.omp/skills/bdd-gherkin/SKILL.md` file that provides:

- Gherkin syntax reference
- Writing good scenarios guide
- bats-bdd usage instructions
- Step definitions templates
- Common pitfalls to avoid
- Checklists for review

When used with Oh My Pi, agents will have access to this guidance when working with BDD tests.

You can also specify a target directory:

```bash
bats-bdd install-skill --directory /path/to/project
```

## How It Works

### 1. Write Your Feature File (Gherkin)

```gherkin
Feature: Calculator
  Basic arithmetic operations

  Background:
    Given I have a calculator

  Scenario: Add two numbers
    When I add 2 and 3
    Then the result should be 5
```

### 2. Define Your Steps

Create a `step_definitions.bash` file with the implementations:

```bash
step_given_i_have_a_calculator() {
  export CALCULATOR_RESULT=0
}

step_when_i_add_2_and_3() {
  CALCULATOR_RESULT=$((2 + 3))
}

step_then_the_result_should_be_5() {
  if [[ "$CALCULATOR_RESULT" -ne 5 ]]; then
    echo "Expected 5, got $CALCULATOR_RESULT" >&2
    return 1
  fi
}
```

### 3. Run

```bash
bats-bdd run my_test.feature
```

The transpiler converts each step to a bash function call based on the naming convention:

| Gherkin Step | Generated Function |
|--------------|-------------------|
| `Given I have a calculator` | `step_given_i_have_a_calculator` |
| `When I add 2 and 3` | `step_when_i_add_2_and_3` |
| `Then the result should be 5` | `step_then_the_result_should_be_5` |

## Supported Gherkin Syntax

### Features and Scenarios

```gherkin
Feature: Feature Name
  Optional description

  Scenario: Scenario name
    Given a step
    When another step
    Then expected result
```

### Background

```gherkin
Background:
  Given initial setup
  And more setup
```

### Scenario Outline (Tables)

```gherkin
Scenario Outline: Add numbers
  Given I have a calculator
  When I add <a> and <b>
  Then the result should be <result>

  Examples:
    | a | b | result |
    | 2 | 3 | 5       |
    | 4 | 5 | 9       |
```

### Supported Steps

- `Given` — Setup/Preparation
- `When` — Action
- `Then` — Assertion/Verification
- `And` — Continuation of previous step
- `But` — Negative continuation

## Project Structure

```
bats-bdd-rust/
├── src/
│   ├── parser/       # Gherkin parser
│   ├── compiler/     # BATS transpiler
│   ├── bats/         # BATS code generator
│   ├── lib.rs        # Public library
│   └── main.rs       # CLI
├── examples/
│   ├── calculator.feature
│   └── calculator.bats
├── step_definitions.bash  # Example step definitions
└── tests/            # Unit tests
```

## Using as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
bats-bdd-rust = "0.1.0"
```

Use the parser:

```rust
use bats_bdd_rust::parser::parse_feature;

let content = r#"
Feature: My Feature
  Scenario: My scenario
    Given a step
"#;

let feature = parse_feature(content).unwrap();
println!("Feature: {}", feature.name);
```

## Examples

Run the included example:

```bash
# After building
./target/release/bats-bdd run examples/calculator.feature
```

Or run directly with cargo:

```bash
cargo run --example run-calculator
```

## License

MIT
