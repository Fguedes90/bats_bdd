---
name: bdd-gherkin
description: Guide for writing good .feature files following BDD principles and using bats-bdd correctly
globs: ["*.feature"]
---

# BDD Gherkin Skill

This skill guides you in writing excellent `.feature` files following BDD principles and using bats-bdd correctly.

## Table of Contents

1. [What is BDD?](#what-is-bdd)
2. [Gherkin Syntax Reference](#gherkin-syntax-reference)
3. [Writing Good Scenarios](#writing-good-scenarios)
4. [Using bats-bdd](#using-bats-bdd)
5. [Step Definitions](#step-definitions)
6. [Common Pitfalls](#common-pitfalls)
7. [Templates](#templates)

---

## What is BDD?

**Behavior-Driven Development (BDD)** is a collaborative approach to software development that bridges the gap between:

- **Business stakeholders** (what they need)
- **Developers** (what to build)
- **Testers** (how to verify)

### The BDD Cycle

```
1. Discovery    → Discuss requirements, examples
2. Formulation  → Write scenarios in Gherkin
3. Automation   → Implement step definitions
4. Development  → Write code to make scenarios pass
5. Demo         → Show working software
```

### Key Principles

1. **Everyone participates** - Business, devs, testers collaborate
2. **Examples over specifications** - Concrete scenarios, not abstract requirements
3. **Living documentation** - Scenarios are executable specifications
4. **Outside-in** - Start from user behavior, not implementation

---

## Gherkin Syntax Reference

Gherkin is the language used to write BDD scenarios. It uses a set of special keywords to give structure and meaning to executable specifications.

### Keywords

| Keyword | Purpose | Aliases |
|---------|---------|---------|
| `Feature` | High-level description of a feature | - |
| `Rule` | Business rule grouping (Gherkin 6+) | - |
| `Scenario` | A concrete example | `Example` |
| `Scenario Outline` | Parametrized scenario | `Scenario Template` |
| `Background` | Steps repeated in all scenarios | - |
| `Examples` | Data for scenario outline | `Scenarios` |

### Step Keywords

| Keyword | Purpose |
|---------|---------|
| `Given` | Initial context (precondition) |
| `When` | Action or event |
| `Then` | Expected outcome (assertion) |
| `And` | Continuation of previous step |
| `But` | Negative continuation |
| `*` | Bulleted step (alternative) |

### Basic Structure

```gherkin
Feature: Feature Title
  As a [role]
  I want [capability]
  So that [benefit]

  Background:
    Given some common setup

  Rule: Business rule description
    Example: Scenario description
      Given initial context
      When some action happens
      Then expected result
```

### Indentation

- Use **2 spaces** for indentation (recommended)
- Keywords are followed by a colon (`:`)
- Steps start with keywords but without colon

---

## Writing Good Scenarios

### The Three-Part Pattern

Every scenario should follow this pattern:

```
GIVEN  → Context/Precondition (setup)
WHEN   → Action/Event (trigger)
THEN   → Expected Result (assertion)
```

### Principle 1: Describe Behavior, Not Implementation

**BAD** (imperative, describes how):
```gherkin
Scenario: User logs in
  Given I am on the login page
  When I enter "bob@example.com" in the email field
  And I enter "password123" in the password field
  And I click the "Login" button
  Then I should see the dashboard page
```

**GOOD** (declarative, describes what):
```gherkin
Scenario: User logs in with valid credentials
  Given a registered user "Bob" with password "password123"
  When Bob logs in with his credentials
  Then Bob should be on the dashboard
```

### Principle 2: Keep Scenarios Short

- **3-5 steps** per scenario recommended
- If you need more, consider splitting into multiple scenarios
- Long scenarios lose their expressive power

### Principle 3: Use Meaningful Names

- Scenario names should describe the expected behavior
- Use personas: "Free user", "Premium subscriber", "Admin"
- Include edge cases in names: "with invalid credentials", "with expired session"

### Principle 4: One Concept Per Scenario

Each scenario should test one specific behavior:
- ✓ "User can reset password"
- ✗ "User can reset password, change email, and update profile"

### Principle 5: Background Should Be Short

- Maximum 4 lines in Background
- Use high-level steps that add business value
- If Background scrolls off screen, it's too long

### Examples: Good vs Bad

**Good Example:**
```gherkin
Feature: Shopping cart
  Scenario: Adding item to cart increases total
    Given the store has a "Laptop" priced at $999
    And my cart is empty
    When I add the "Laptop" to my cart
    Then my cart should contain 1 item
    And the total should be $999
```

**Bad Example:**
```gherkin
Scenario: Adding item
  Given I navigate to "/products"
  When I click the first product
  And I wait for page load
  And I scroll down 500px
  And I click "Add to Cart" button
  And I wait for animation
  And I click "View Cart"
  Then I should see "1 item in cart"
```

---

## Using bats-bdd

### Installation

```bash
# Using the install script
curl -sSL https://raw.githubusercontent.com/Fguedes90/bats-bdd/main/install.sh | sh

# Or build from source
cargo install bats-bdd
```

### Basic Usage

```bash
# Parse and display AST
bats-bdd parse my.feature

# Run feature file (transpiles + executes)
bats-bdd run my.feature

# With verbose output (shows generated BATS code)
bats-bdd run my.feature --verbose

# Custom step definitions file
bats-bdd run my.feature --steps my-steps.bash
```

### Command Options

TV:| `-v, --verbose` | Show generated BATS code |
TH:| `-p, --parallel` | Run BATS tests in parallel |

YZ:### Running Directory of Features

QZ:bats-bdd can process multiple `.feature` files in a directory:

```bash
# Run all .feature files in a directory
bats-bdd run features/

# With parallel execution
bats-bdd run features/ --parallel

# With custom output directory
bats-bdd run features/ -o output/
```

RP:### Automatic File Generation

KB:**You only need to write `.feature` files!**

VB:bats-bdd automatically:
SB:1. Generates `.bats` test files from each `.feature`
QF:2. Creates `step_definitions.bash` with TODO stubs for missing steps
FM:3. Updates existing stubs (doesn't duplicate)

HV:**Workflow for agents:**

```bash
1. Write only the .feature file(s)
2. Run: bats-bdd run features/
3. bats-bdd generates:
   - feature.bats (test file)
   - step_definitions.bash (with TODO stubs)
4. Run the tests to verify they FAIL with TODO messages
5. Implement the TODO stubs in step_definitions.bash
6. Run tests again to verify they PASS
```

HQ:### Idempotency

KN:bats-bdd is **idempotent** - running it multiple times is safe:
SZ:- If no `.feature` files changed, no files are modified
XZ:- Step definitions are only added for NEW steps
QF:- Existing step implementations are preserved
QF:- Duplicate step definitions are not created

BW:### Installing the BDD Skill

To install this skill in your project for better agent guidance:

```bash
bats-bdd install-skill
```

This will create a `.omp/skills/bdd-gherkin/` directory with the skill files.

---

## Step Definitions

### Naming Convention

bats-bdd converts Gherkin steps to Bash functions using this pattern:

| Gherkin Step | Function Name |
|--------------|---------------|
| `Given I have a calculator` | `step_given_i_have_a_calculator` |
| `When I add 2 and 3` | `step_when_i_add_2_and_3` |
| `Then the result should be 5` | `step_then_the_result_should_be_5` |
| `And I have money` | `step_and_i_have_money` |

### Basic Step Definition Template

```bash
#!/usr/bin/env bash

# ============================================================================
# Step Definitions for My Feature
# ============================================================================

# GIVEN steps - Setup/Preconditions
step_given_i_have_a_calculator() {
  export CALCULATOR_RESULT=0
}

step_given_a_registered_user() {
  local user="$1"
  local password="$2"
  # Setup user in test database
}

# WHEN steps - Actions/Events
step_when_i_add_2_and_3() {
  CALCULATOR_RESULT=$((2 + 3))
}

step_when_user_logs_in() {
  local user="$1"
  local password="$2"
  # Perform login action
}

# THEN steps - Assertions (MUST return 0 for pass, 1 for fail)
step_then_the_result_should_be_5() {
  if [[ "$CALCULATOR_RESULT" -ne 5 ]]; then
    echo "Expected 5, got $CALCULATOR_RESULT" >&2
    return 1
  fi
}

step_then_user_should_see() {
  local expected="$1"
  local actual="$2"
  if [[ "$actual" != "$expected" ]]; then
    echo "Expected '$expected' but got '$actual'" >&2
    return 1
  fi
}
```

### Handling Parameters

Steps with parameters extract values from the step text:

```gherkin
Scenario: Add two numbers
  When I add 2 and 3
```

```bash
step_when_i_add_2_and_3() {
  # Values are passed as positional arguments
  local a="$1"  # "2"
  local b="$2"  # "3"
  RESULT=$((a + b))
}
```

### Data Tables in Steps

```gherkin
Scenario: Multiple users
  Given the following users:
    | name | email           |
    | Bob  | bob@example.com |
    | Alice| alice@test.com  |
```

```bash
step_given_the_following_users() {
  local table="$1"  # Entire table as string
  # Parse table and create users
}
```

### Scenario Outline Variables

```gherkin
Scenario Outline: Calculator operations
  When I <operation> <a> and <b>
  Then the result should be <result>

  Examples:
    | operation | a | b | result |
    | add       | 2 | 3 | 5      |
    | subtract  | 5 | 3 | 2      |
    | multiply  | 4 | 3 | 12     |
```

The `<variable>` placeholders become function arguments.

---

## Common Pitfalls

### 1. Imperative Over Declarative

**Problem:** Writing "how" instead of "what"

**Solution:** Focus on business behavior, not UI interactions

### 2. Duplicate Steps with Different Keywords

**Problem:**
```gherkin
Given there is money in my account
Then there is money in my account
```

**Solution:** Use different wording:
```gherkin
Given my account has a balance of $100
Then my account balance should be $100
```

### 3. Testing Implementation Details

**Problem:**
```gherkin
When I click the button with id "login-btn"
Then the form should be hidden
```

**Solution:**
```gherkin
When I submit the login form
Then I should see the dashboard
```

### 4. Missing Assertions

**Problem:** Only Given and When, no Then

**Solution:** Every scenario must have a Then with an assertion

### 5. Complex Background

**Problem:** 10+ lines in Background

**Solution:** Use higher-level steps, split into multiple features

### 6. Hard-coded Test Data in Scenarios

**Problem:**
```gherkin
Given user "testuser123" with email "test@test.com" exists
```

**Solution:** Use abstract data or factories:
```gherkin
Given a registered user exists
```

### 7. Not Using And/But Correctly

**Problem:** Starting new clause instead of continuing

**Solution:**
```gherkin
Given I am logged in    # First Given
And I have admin rights # Continuation - GOOD

Given I am logged in     # Fresh start - BAD if continuing
Given I have admin rights
```

---

## Templates

### Minimal Feature Template

```gherkin
Feature: [Feature Name]
  As a [role]
  I want [capability]
  So that [benefit]

  Scenario: [Scenario description]
    Given [initial context]
    When [action happens]
    Then [expected result]
```

### Complete Feature Template

```gherkin
Feature: [Feature Name]
  [Optional: Detailed description of the feature]

  [Optional: Business rules or acceptance criteria]

  Background:
    Given [common setup]

  Rule: [Business Rule 1]
    Example: [Happy path scenario]
      Given [context]
      When [action]
      Then [result]

    Example: [Edge case]
      Given [context]
      When [action]
      Then [result]

  Rule: [Business Rule 2]
    Example: [Another scenario]
      # ...
```

### Scenario Outline Template

```gherkin
Scenario Outline: [Description with <variables>]
  Given [some <input>]
  When [I perform <action>]
  Then [the result should be <expected>]

  Examples:
    | input | action | expected |
    | value1| doX    | result1  |
    | value2| doY    | result2  |
```

### Step Definitions Template

```bash
#!/usr/bin/env bash
# ============================================================================
# Step Definitions for [Feature Name]
# Generated by bats-bdd
# ============================================================================

# ---------------------------------------------------------------------------
# GIVEN - Setup/Preconditions
# ---------------------------------------------------------------------------

step_given_[description]() {
  # Setup code here
  return 0  # Success
}

# ---------------------------------------------------------------------------
# WHEN - Actions/Events
# ---------------------------------------------------------------------------

step_when_[description]() {
  # Action code here
  return 0  # Success
}

# ---------------------------------------------------------------------------
# THEN - Assertions
# ---------------------------------------------------------------------------

step_then_[description]() {
  # Assertion code here
  # MUST return 0 for pass, 1 for fail
  
  if [[ "$RESULT" -ne "$EXPECTED" ]]; then
    echo "Expected $EXPECTED but got $RESULT" >&2
    return 1
  fi
  return 0
}

# ---------------------------------------------------------------------------
# AND - Continuations (reuse appropriate section)
# ---------------------------------------------------------------------------

step_and_[description]() {
  # Continue from previous step type
  return 0
}
```

---

## Quick Reference

### bats-bdd Command Summary

```bash
# Parse feature file (show structure)
bats-bdd parse <feature-file>

# Run feature file
bats-bdd run <feature-file>

# Run with custom step definitions
bats-bdd run <feature-file> -s my-steps.bash

# Show generated BATS code
bats-bdd run <feature-file> --verbose

# Install skill for better agent guidance
bats-bdd install-skill
```

### Gherkin Checklist

Before committing a `.feature` file, verify:

- [ ] Feature has meaningful title and description
- [ ] Each scenario follows Given-When-Then pattern
- [ ] Scenarios are declarative, not imperative
- [ ] Scenario names describe expected behavior
- [ ] Steps use "And" for continuation, not new clauses
- [ ] Background is under 4 lines
- [ ] No implementation details (IDs, CSS selectors, etc.)
- [ ] Scenario Outline uses `<variable>` syntax correctly
- [ ] Examples table has header row
- [ ] All scenarios have assertions (Then steps)

### bats-bdd Checklist

- [ ] Step definitions follow naming convention
- [ ] Then steps return 1 on failure, 0 on success
- [ ] Use `export` for variables shared between steps
- [ ] Test runs successfully with `bats-bdd run`

---

## Additional Resources

- [Cucumber Gherkin Reference](https://cucumber.io/docs/gherkin/reference/)
- [Writing Better Gherkin](https://cucumber.io/docs/bdd/better-gherkin)
- [bats-core Documentation](https://bats-core.readthedocs.io/)
- [BDD with Cucumber](https://cucumber.io/docs/bdd/)

---

*This skill was created to help developers and AI agents write better BDD scenarios and use bats-bdd effectively.*
