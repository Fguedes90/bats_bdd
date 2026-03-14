//! Gherkin parser implementation using pest

use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::sync::OnceLock;
use thiserror::Error;

pub mod ast;
pub mod i18n;

use i18n::LanguageRegistry;

#[derive(Parser)]
#[grammar = "parser/gherkin.pest"]
struct GherkinParser;

/// Global language registry initialized once
fn get_language_registry() -> &'static LanguageRegistry {
    static REGISTRY: OnceLock<LanguageRegistry> = OnceLock::new();
    REGISTRY.get_or_init(LanguageRegistry::new)
}

/// Detect language from content (e.g., "# language: pt")
fn detect_language(content: &str) -> Option<&str> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        // Support various formats: # language: pt, #language:pt, #  language:pt
        let lang = trimmed
            .strip_prefix("# language:")
            .or_else(|| trimmed.strip_prefix("#language:"))
            .map(|s| s.trim());
        if let Some(lang) = lang {
            if !lang.is_empty() {
                return Some(lang);
            }
        }
        // Stop at first non-comment, non-empty line
        if !trimmed.starts_with('#') {
            break;
        }
    }
    None
}

/// Parse tags from a pest pair
fn parse_tags(pair: pest::iterators::Pair<Rule>) -> Vec<String> {
    let mut tags = Vec::new();
    for tag_pair in pair.into_inner() {
        if tag_pair.as_rule() == Rule::tag {
            let tag = tag_pair.as_str().trim_start_matches('@').to_string();
            tags.push(tag);
        }
    }
    tags
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Pest(#[from] pest::error::Error<Rule>),
    #[error("Unknown step keyword: {0}")]
    UnknownStepKeyword(String),
}

/// Parse a Gherkin feature file from a string
pub fn parse_feature(content: &str) -> Result<ast::Feature, ParseError> {
    let language = detect_language(content);
    parse_feature_with_language(content, language)
}

/// Parse a Gherkin feature file from a string with explicit language
pub fn parse_feature_with_language(
    content: &str,
    language: Option<&str>,
) -> Result<ast::Feature, ParseError> {
    let pairs = GherkinParser::parse(Rule::feature, content)?;
    let mut feature = ast::Feature {
        language: language.map(|s| s.to_string()),
        ..Default::default()
    };

    // Get the feature rule and iterate over its children
    for feature_pair in pairs {
        for pair in feature_pair.into_inner() {
            match pair.as_rule() {
                Rule::tags => {
                    feature.tags = parse_tags(pair);
                }
                Rule::feature_name => {
                    feature.name = pair.as_str().trim().to_string();
                }
                Rule::description => {
                    feature.description = Some(pair.as_str().trim().to_string());
                }
                Rule::background => {
                    // Parse background steps
                    let background = parse_background(pair, language)?;
                    feature.background = Some(background);
                }
                Rule::scenario => {
                    let scenario = parse_scenario(pair, language)?;
                    feature.scenarios.push(scenario);
                }
                Rule::scenario_outline => {
                    let scenario_outline = parse_scenario_outline(pair, language)?;
                    feature
                        .scenarios
                        .push(ast::Scenario::Outline(scenario_outline));
                }
                Rule::rule => {
                    let rule = parse_rule(pair, language)?;
                    feature.rules.push(rule);
                }
                _ => {}
            }
        }
    }

    Ok(feature)
}

/// Parse a Gherkin feature file from disk
pub fn parse_feature_file(path: &str) -> Result<ast::Feature, ParseError> {
    let content = fs::read_to_string(path)?;
    parse_feature(&content)
}

fn parse_background(
    pair: pest::iterators::Pair<Rule>,
    language: Option<&str>,
) -> Result<ast::Background, ParseError> {
    let mut background = ast::Background::default();

    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::step {
            let step = parse_step(inner_pair, language)?;
            background.steps.push(step);
        }
    }

    Ok(background)
}

fn parse_scenario(
    pair: pest::iterators::Pair<Rule>,
    language: Option<&str>,
) -> Result<ast::Scenario, ParseError> {
    let mut scenario = ast::Scenario::Simple(ast::SimpleScenario::default());

    if let ast::Scenario::Simple(ref mut simple) = scenario {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::tags => {
                    simple.tags = parse_tags(inner_pair);
                }
                Rule::scenario_name => {
                    simple.name = inner_pair.as_str().trim().to_string();
                }
                Rule::step => {
                    let step = parse_step(inner_pair, language)?;
                    simple.steps.push(step);
                }
                _ => {}
            }
        }
    }

    Ok(scenario)
}

fn parse_scenario_outline(
    pair: pest::iterators::Pair<Rule>,
    language: Option<&str>,
) -> Result<ast::ScenarioOutline, ParseError> {
    let mut outline = ast::ScenarioOutline::default();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::tags => {
                outline.tags = parse_tags(inner_pair);
            }
            Rule::scenario_name => {
                outline.name = inner_pair.as_str().trim().to_string();
            }
            Rule::step => {
                let step = parse_step(inner_pair, language)?;
                outline.steps.push(step);
            }
            Rule::examples => {
                let examples = parse_examples(inner_pair)?;
                outline.examples = examples;
            }
            _ => {}
        }
    }

    Ok(outline)
}

fn parse_examples(pair: pest::iterators::Pair<Rule>) -> Result<ast::Examples, ParseError> {
    let mut examples = ast::Examples::default();

    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::table {
            let table = parse_table(inner_pair)?;
            examples.table = table;
        }
    }

    Ok(examples)
}

fn parse_table(pair: pest::iterators::Pair<Rule>) -> Result<ast::Table, ParseError> {
    let mut table = ast::Table::default();

    for row_pair in pair.into_inner() {
        if row_pair.as_rule() == Rule::table_row {
            let mut row = Vec::new();
            for cell_pair in row_pair.into_inner() {
                if cell_pair.as_rule() == Rule::table_cell {
                    row.push(cell_pair.as_str().trim().to_string());
                }
            }
            table.rows.push(row);
        }
    }

    Ok(table)
}

fn parse_step(
    pair: pest::iterators::Pair<Rule>,
    language: Option<&str>,
) -> Result<ast::Step, ParseError> {
    let mut keyword = String::new();
    let mut text = String::new();
    let mut doc_string: Option<ast::DocString> = None;
    let mut data_table: Option<ast::Table> = None;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::step_keyword => {
                keyword = inner_pair.as_str().trim().to_string();
            }
            Rule::step_text => {
                text = inner_pair.as_str().trim().to_string();
            }
            Rule::doc_string => {
                doc_string = Some(parse_doc_string(inner_pair)?);
            }
            Rule::data_table => {
                data_table = Some(parse_data_table(inner_pair)?);
            }
            _ => {}
        }
    }

    // Use LanguageRegistry to detect step type
    let registry = get_language_registry();
    let step_type = registry
        .detect_step_type(&keyword, language)
        .ok_or_else(|| ParseError::UnknownStepKeyword(keyword.clone()))?;

    let step = match step_type {
        i18n::StepType::Given => ast::Step::Given(text, doc_string, data_table),
        i18n::StepType::When => ast::Step::When(text, doc_string, data_table),
        i18n::StepType::Then => ast::Step::Then(text, doc_string, data_table),
        i18n::StepType::And => ast::Step::And(text, doc_string, data_table),
        i18n::StepType::But => ast::Step::But(text, doc_string, data_table),
    };

    Ok(step)
}

fn parse_rule(
    pair: pest::iterators::Pair<Rule>,
    language: Option<&str>,
) -> Result<ast::Rule, ParseError> {
    let mut rule = ast::Rule::default();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::tags => {
                rule.tags = parse_tags(inner_pair);
            }
            Rule::rule_name => {
                rule.name = inner_pair.as_str().trim().to_string();
            }
            Rule::description => {
                rule.description = Some(inner_pair.as_str().trim().to_string());
            }
            Rule::background => {
                let background = parse_background(inner_pair, language)?;
                rule.background = Some(background);
            }
            Rule::scenario => {
                let scenario = parse_scenario(inner_pair, language)?;
                rule.scenarios.push(scenario);
            }
            Rule::scenario_outline => {
                let scenario_outline = parse_scenario_outline(inner_pair, language)?;
                rule.scenarios
                    .push(ast::Scenario::Outline(scenario_outline));
            }
            _ => {}
        }
    }

    Ok(rule)
}

fn parse_doc_string(pair: pest::iterators::Pair<Rule>) -> Result<ast::DocString, ParseError> {
    let mut doc_string = ast::DocString::default();
    let mut content_lines: Vec<String> = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::doc_string_type => {
                doc_string.content_type = Some(inner_pair.as_str().trim().to_string());
            }
            Rule::doc_string_content_line => {
                content_lines.push(inner_pair.as_str().to_string());
            }
            Rule::doc_string_delimiter => {}
            _ => {}
        }
    }

    doc_string.content = content_lines.join("\n");
    Ok(doc_string)
}

fn parse_data_table(pair: pest::iterators::Pair<Rule>) -> Result<ast::Table, ParseError> {
    let mut table = ast::Table::default();

    for row_pair in pair.into_inner() {
        if row_pair.as_rule() == Rule::table_row {
            let mut row = Vec::new();
            for cell_pair in row_pair.into_inner() {
                if cell_pair.as_rule() == Rule::table_cell {
                    row.push(cell_pair.as_str().trim().to_string());
                }
            }
            table.rows.push(row);
        }
    }

    Ok(table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parse() {
        let content = "Feature: Test\n  Scenario: Simple\n    Given a step\n";
        let result = parse_feature(content);
        assert!(result.is_ok());
        let feature = result.unwrap();
        assert_eq!(feature.name, "Test");
    }
}

#[test]
fn test_feature_with_tags() {
    // Feature-level tags are now supported
    let content = "@smoke\nFeature: Test\n  Scenario: Simple\n    Given a step\n";
    let result = parse_feature(content);
    println!("Result: {:?}", result);
    // Feature-level tags should now succeed
    assert!(result.is_ok());
    let feature = result.unwrap();
    assert_eq!(feature.tags, vec!["smoke"]);
    assert_eq!(feature.name, "Test");
}

#[test]
fn test_scenario_with_tags() {
    // This should work - tags in scenario
    let content = "Feature: Test\n  @smoke\n  Scenario: Simple\n    Given a step\n";
    let result = parse_feature(content);
    println!("Scenario tags result: {:?}", result.is_ok());
    assert!(result.is_ok());
}

#[test]
fn test_scenario_tags_inline() {
    // Test tags on same line as Scenario (might work based on grammar)
    let content = "Feature: Test\n  @smoke  Scenario: Simple\n    Given a step\n";
    let result = parse_feature(content);
    println!("Inline tags result: {:?}", result);
}

#[test]
fn test_feature_with_comment() {
    // Comments should be supported
    let content = "# language: en\nFeature: Test\n  Scenario: Simple\n    Given a step\n";
    let result = parse_feature(content);
    assert!(result.is_ok());
    let feature = result.unwrap();
    assert_eq!(feature.name, "Test");
}

#[test]
fn test_feature_comment_before_tags() {
    // Comment before tags
    let content =
        "# This is a comment\n@smoke\nFeature: Test\n  Scenario: Simple\n    Given a step\n";
    let result = parse_feature(content);
    assert!(result.is_ok());
    let feature = result.unwrap();
    assert_eq!(feature.tags, vec!["smoke"]);
    assert_eq!(feature.name, "Test");
}

#[test]
fn test_feature_no_trailing_newline() {
    // File without trailing newline should parse
    let content = "Feature: Test\n  Scenario: Simple\n    Given a step";
    let result = parse_feature(content);
    assert!(result.is_ok());
    let feature = result.unwrap();
    assert_eq!(feature.name, "Test");
}

#[test]
fn test_feature_comment_at_end() {
    // Comment at end of file
    let content = "Feature: Test\n  Scenario: Simple\n    Given a step\n# End comment";
    let result = parse_feature(content);
    assert!(result.is_ok());
}

#[test]
fn test_portuguese_step_keywords() {
    // Test Portuguese step keywords via language header
    // Note: Feature/Scenario keywords are still English-only in grammar
    let content = "# language: pt\nFeature: Test\n  Scenario: Simple\n    Dado um passo\n    Quando algo acontece\n    Então espero resultado\n    E outra coisa\n    Mas não aquilo\n";
    let result = parse_feature(content);
    assert!(result.is_ok(), "Parse failed: {:?}", result.err());
    let feature = result.unwrap();
    assert_eq!(feature.name, "Test");
    assert_eq!(feature.language, Some("pt".to_string()));

    let scenario = match &feature.scenarios[0] {
        ast::Scenario::Simple(s) => s,
        _ => panic!("Expected simple scenario"),
    };
    assert_eq!(scenario.steps.len(), 5);

    // Verify step types are correctly detected from Portuguese keywords
    assert_eq!(scenario.steps[0].keyword(), "Given");
    assert_eq!(scenario.steps[0].text(), "um passo");
    assert_eq!(scenario.steps[1].keyword(), "When");
    assert_eq!(scenario.steps[1].text(), "algo acontece");
    assert_eq!(scenario.steps[2].keyword(), "Then");
    assert_eq!(scenario.steps[2].text(), "espero resultado");
    assert_eq!(scenario.steps[3].keyword(), "And");
    assert_eq!(scenario.steps[3].text(), "outra coisa");
    assert_eq!(scenario.steps[4].keyword(), "But");
    assert_eq!(scenario.steps[4].text(), "não aquilo");
}

#[test]
fn test_portuguese_step_keywords_explicit() {
    // Test Portuguese step keywords with explicit language parameter
    let content =
        "Feature: Test\n  Scenario: Simple\n    Dado um passo\n    Quando algo acontece\n";
    let result = parse_feature_with_language(content, Some("pt"));
    assert!(result.is_ok(), "Parse failed: {:?}", result.err());
    let feature = result.unwrap();
    assert_eq!(feature.name, "Test");

    let scenario = match &feature.scenarios[0] {
        ast::Scenario::Simple(s) => s,
        _ => panic!("Expected simple scenario"),
    };
    assert_eq!(scenario.steps.len(), 2);
    assert_eq!(scenario.steps[0].keyword(), "Given");
    assert_eq!(scenario.steps[1].keyword(), "When");
}

#[test]
fn test_step_with_doc_string() {
    let content = "Feature: Test\n  Scenario: Simple\n    Given a step\n      \"\"\"\n      Some doc string content\n      Line 2 of content\n      \"\"\"";
    let result = parse_feature(content);
    assert!(result.is_ok(), "Parse failed: {:?}", result);
    let feature = result.unwrap();
    let scenario = match &feature.scenarios[0] {
        ast::Scenario::Simple(s) => s,
        _ => panic!("Expected simple scenario"),
    };
    let step = &scenario.steps[0];
    assert_eq!(step.text(), "a step");
    let doc_string = step.doc_string().expect("Expected doc_string");
    assert!(doc_string.content.contains("Some doc string content"));
    assert!(doc_string.content.contains("Line 2 of content"));
}

#[test]
fn test_step_with_data_table() {
    let content = r#"Feature: Test
  Scenario: Simple
    Given a step
      | col1 | col2 |
      | val1 | val2 |
      | val3 | val4 |"#;
    let result = parse_feature(content);
    assert!(result.is_ok(), "Parse failed: {:?}", result);
    let feature = result.unwrap();
    let scenario = match &feature.scenarios[0] {
        ast::Scenario::Simple(s) => s,
        _ => panic!("Expected simple scenario"),
    };
    let step = &scenario.steps[0];
    assert_eq!(step.text(), "a step");
    let table = step.data_table().expect("Expected data_table");
    assert_eq!(table.rows.len(), 3);
    assert_eq!(table.rows[0], vec!["col1", "col2"]);
    assert_eq!(table.rows[1], vec!["val1", "val2"]);
    assert_eq!(table.rows[2], vec!["val3", "val4"]);
}

#[test]
fn test_step_with_doc_string_and_type() {
    let content = "Feature: Test\n  Scenario: Simple\n    Given a step\n      \"\"\"json\n      {\"key\": \"value\"}\n      \"\"\"";
    let result = parse_feature(content);
    assert!(result.is_ok(), "Parse failed: {:?}", result);
    let feature = result.unwrap();
    let scenario = match &feature.scenarios[0] {
        ast::Scenario::Simple(s) => s,
        _ => panic!("Expected simple scenario"),
    };
    let step = &scenario.steps[0];
    let doc_string = step.doc_string().expect("Expected doc_string");
    assert_eq!(doc_string.content_type.as_deref(), Some("json"));
    assert!(doc_string.content.contains("\"key\": \"value\""));
}

#[test]
fn test_feature_with_rule() {
    let content = r#"Feature: Test Feature
  @business @critical
  Rule: A Business Rule
    Scenario: Simple
      Given a step
      When another step
      Then a result"#;
    let result = parse_feature(content);
    assert!(result.is_ok(), "Parse failed: {:?}", result);
    let feature = result.unwrap();
    assert_eq!(feature.name, "Test Feature");
    assert_eq!(feature.rules.len(), 1);
    let rule = &feature.rules[0];
    assert_eq!(rule.name, "A Business Rule");
    assert_eq!(rule.tags, vec!["business", "critical"]);
    assert_eq!(rule.scenarios.len(), 1);
}

#[test]
fn test_rule_with_background() {
    let content = r#"Feature: Test
  Rule: My Rule
    Background:
      Given setup is done
    Scenario: Simple
      When action
      Then result"#;
    let result = parse_feature(content);
    assert!(result.is_ok(), "Parse failed: {:?}", result);
    let feature = result.unwrap();
    assert_eq!(feature.rules.len(), 1);
    let rule = &feature.rules[0];
    assert_eq!(rule.name, "My Rule");
    assert_eq!(rule.scenarios.len(), 1);
}
