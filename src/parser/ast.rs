use serde::{Deserialize, Serialize};

/// Represents a Gherkin Feature
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Feature {
    pub name: String,
    pub description: Option<String>,
    pub background: Option<Background>,
    pub scenarios: Vec<Scenario>,
    pub rules: Vec<Rule>,
    pub tags: Vec<String>,
    pub language: Option<String>,
}

/// Represents a Gherkin Rule (Gherkin 6+)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Rule {
    pub name: String,
    pub description: Option<String>,
    pub background: Option<Background>,
    pub scenarios: Vec<Scenario>,
    pub tags: Vec<String>,
}

/// Represents a Gherkin Background
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Background {
    pub name: Option<String>,
    pub steps: Vec<Step>,
}

/// Represents different types of scenarios
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Scenario {
    Simple(SimpleScenario),
    Outline(ScenarioOutline),
}

impl Default for Scenario {
    fn default() -> Self {
        Scenario::Simple(SimpleScenario::default())
    }
}

/// Represents a simple Gherkin Scenario
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SimpleScenario {
    pub name: String,
    pub steps: Vec<Step>,
    pub tags: Vec<String>,
}

/// Represents a Gherkin Scenario Outline
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ScenarioOutline {
    pub name: String,
    pub steps: Vec<Step>,
    pub tags: Vec<String>,
    pub examples: Examples,
}

/// Represents Examples for Scenario Outline
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Examples {
    pub name: Option<String>,
    pub table: Table,
}

/// Represents a data table
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Table {
    pub rows: Vec<Vec<String>>,
}

/// Represents a DocString (triple-quoted string in Gherkin)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DocString {
    pub content: String,
    pub content_type: Option<String>, // ex: "json", "xml"
}

/// Represents different types of steps
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Step {
    Given(String, Option<DocString>, Option<Table>),
    When(String, Option<DocString>, Option<Table>),
    Then(String, Option<DocString>, Option<Table>),
    And(String, Option<DocString>, Option<Table>),
    But(String, Option<DocString>, Option<Table>),
}

impl Default for Step {
    fn default() -> Self {
        Step::Given(String::new(), None, None)
    }
}

impl Step {
    pub fn keyword(&self) -> &str {
        match self {
            Step::Given(_, _, _) => "Given",
            Step::When(_, _, _) => "When",
            Step::Then(_, _, _) => "Then",
            Step::And(_, _, _) => "And",
            Step::But(_, _, _) => "But",
        }
    }

    pub fn text(&self) -> &str {
        match self {
            Step::Given(text, _, _) => text,
            Step::When(text, _, _) => text,
            Step::Then(text, _, _) => text,
            Step::And(text, _, _) => text,
            Step::But(text, _, _) => text,
        }
    }

    pub fn doc_string(&self) -> Option<&DocString> {
        match self {
            Step::Given(_, doc, _) => doc.as_ref(),
            Step::When(_, doc, _) => doc.as_ref(),
            Step::Then(_, doc, _) => doc.as_ref(),
            Step::And(_, doc, _) => doc.as_ref(),
            Step::But(_, doc, _) => doc.as_ref(),
        }
    }

    pub fn data_table(&self) -> Option<&Table> {
        match self {
            Step::Given(_, _, table) => table.as_ref(),
            Step::When(_, _, table) => table.as_ref(),
            Step::Then(_, _, table) => table.as_ref(),
            Step::And(_, _, table) => table.as_ref(),
            Step::But(_, _, table) => table.as_ref(),
        }
    }
}
