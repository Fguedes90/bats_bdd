Feature: Background Steps
  Test that background steps are executed before each scenario
  Background is transpiled to BATS setup() function

  Background:
    Given I initialize the test environment
    And I set the default timeout to 30 seconds

  Scenario: First scenario uses background
    When I run the first test
    Then the background should be executed

  Scenario: Second scenario also uses background
    When I run the second test
    Then the background should be executed again

  Scenario: Third scenario verifies isolation
    Given I verify the state is clean
    When I run the third test
    Then each scenario should have fresh background
