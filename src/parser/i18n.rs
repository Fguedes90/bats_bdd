use std::collections::HashMap;

/// Language keywords for Gherkin parsing
#[derive(Debug, Clone)]
pub struct LanguageKeywords {
    pub feature: Vec<String>,
    pub background: Vec<String>,
    pub scenario: Vec<String>,
    pub scenario_outline: Vec<String>,
    pub examples: Vec<String>,
    pub given: Vec<String>,
    pub when: Vec<String>,
    pub then: Vec<String>,
    pub and: Vec<String>,
    pub but: Vec<String>,
}

impl LanguageKeywords {
    /// Create English language keywords
    pub fn english() -> Self {
        Self {
            feature: vec!["Feature".to_string()],
            background: vec!["Background".to_string()],
            scenario: vec!["Scenario".to_string()],
            scenario_outline: vec![
                "Scenario Outline".to_string(),
                "Scenario Template".to_string(),
            ],
            examples: vec!["Examples".to_string(), "Scenarios".to_string()],
            given: vec!["Given".to_string(), "*".to_string()],
            when: vec!["When".to_string()],
            then: vec!["Then".to_string()],
            and: vec!["And".to_string()],
            but: vec!["But".to_string()],
        }
    }

    /// Create Portuguese language keywords
    pub fn portuguese() -> Self {
        Self {
            feature: vec!["Funcionalidade".to_string(), "Característica".to_string()],
            background: vec!["Contexto".to_string(), "Cenário de Fundo".to_string()],
            scenario: vec!["Cenário".to_string(), "Caso".to_string()],
            scenario_outline: vec![
                "Esquema do Cenário".to_string(),
                "Delineação do Cenário".to_string(),
            ],
            examples: vec!["Exemplos".to_string(), "Cenários".to_string()],
            given: vec![
                "Dado".to_string(),
                "Dada".to_string(),
                "Dados".to_string(),
                "Dadas".to_string(),
                "*".to_string(),
            ],
            when: vec!["Quando".to_string()],
            then: vec!["Então".to_string(), "Logo".to_string()],
            and: vec!["E".to_string()],
            but: vec!["Mas".to_string()],
        }
    }

    /// Detect the step type from a keyword string
    pub fn detect_step_type(&self, keyword: &str) -> Option<StepType> {
        let normalized = keyword.trim();

        if self
            .given
            .iter()
            .any(|k| k.eq_ignore_ascii_case(normalized))
        {
            return Some(StepType::Given);
        }
        if self.when.iter().any(|k| k.eq_ignore_ascii_case(normalized)) {
            return Some(StepType::When);
        }
        if self.then.iter().any(|k| k.eq_ignore_ascii_case(normalized)) {
            return Some(StepType::Then);
        }
        if self.and.iter().any(|k| k.eq_ignore_ascii_case(normalized)) {
            return Some(StepType::And);
        }
        if self.but.iter().any(|k| k.eq_ignore_ascii_case(normalized)) {
            return Some(StepType::But);
        }

        None
    }
}

/// Registry for supported languages
#[derive(Debug, Clone)]
pub struct LanguageRegistry {
    languages: HashMap<String, LanguageKeywords>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            languages: HashMap::new(),
        };

        // Register English as default
        registry.register("en".to_string(), LanguageKeywords::english());
        registry.register("pt".to_string(), LanguageKeywords::portuguese());

        registry
    }

    pub fn register(&mut self, code: String, keywords: LanguageKeywords) {
        self.languages.insert(code, keywords);
    }

    pub fn get(&self, code: &str) -> Option<&LanguageKeywords> {
        self.languages.get(code)
    }

    /// Get default language keywords (English)
    pub fn default_keywords(&self) -> &LanguageKeywords {
        self.languages
            .get("en")
            .expect("English language must be registered")
    }

    /// Detect the step type from a keyword string
    pub fn detect_step_type(&self, keyword: &str, language: Option<&str>) -> Option<StepType> {
        let keywords = match language {
            Some(lang) => self.get(lang).unwrap_or_else(|| self.default_keywords()),
            None => self.default_keywords(),
        };

        keywords.detect_step_type(keyword)
    }
}

/// Represents the type of a Gherkin step
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepType {
    Given,
    When,
    Then,
    And,
    But,
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}
