use bats_bdd_rust::parser::parse_feature;

#[test]
fn test_simple_parse() {
    // Note: Add trailing newline
    let content = "Feature: Simple feature\n  Scenario: Simple scenario\n    Given a step\n";
    let result = parse_feature(content);
    match &result {
        Ok(feature) => {
            println!("Feature name: {}", feature.name);
            println!("Scenarios: {}", feature.scenarios.len());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    assert!(result.is_ok(), "Parser should succeed");
}

#[test]
fn test_parse_feature() {
    // Note: Add trailing newline
    let feature_text = r#"Feature: Basic Calculator
  As a user
  I want to perform basic calculations
  So that I can solve simple math problems

  Scenario: Add two numbers
    Given I have a calculator
    When I add 2 and 3
    Then the result should be 5
"#;

    let result = parse_feature(feature_text);
    match &result {
        Ok(feature) => {
            println!("Feature name: {}", feature.name);
            println!("Description: {:?}", feature.description);
            println!("Scenarios: {}", feature.scenarios.len());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    assert!(result.is_ok(), "Parser should succeed");
    
    let feature = result.unwrap();
    assert_eq!(feature.name, "Basic Calculator");
    assert!(feature.description.is_some());
    assert_eq!(feature.scenarios.len(), 1);
    
    // Access the scenario properly through the enum
    match &feature.scenarios[0] {
        bats_bdd_rust::parser::ast::Scenario::Simple(simple) => {
            assert_eq!(simple.name, "Add two numbers");
            assert_eq!(simple.steps.len(), 3);
        }
        _ => panic!("Expected Simple scenario"),
    }
}