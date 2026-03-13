Feature: Tags Support
@smoke
Scenario: Scenario with single tag
  Given I have a tagged scenario
  Then the smoke tag should be present
@regression @critical
Scenario: Scenario with multiple tags
  Given I have a multi-tagged scenario
  Then both tags should be present
