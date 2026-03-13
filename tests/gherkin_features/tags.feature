Feature: Tags Support
  Test that tags are parsed and preserved in generated BATS files
  Tags should appear as comments in the generated .bats file

  @smoke
  Scenario: Scenario with single tag
    Given I have a tagged scenario
    When I check the tags
    Then the smoke tag should be present
  @regression @critical
  Scenario: Scenario with multiple tags
    Given I have a multi-tagged scenario
    When I check the tags
    Then both regression and critical tags should be present
  @api
  Scenario: API test tag
    Given I have an API test
    When I run the API test
    Then the api tag should be in the output
  @slow @integration
  Scenario: Slow integration test
    Given I have a slow test
    When I run the integration test
    Then both slow and integration tags should be present
  @wip
  Scenario: Work in progress test
    Given I have a WIP test
    Then it should be marked as work in progress
