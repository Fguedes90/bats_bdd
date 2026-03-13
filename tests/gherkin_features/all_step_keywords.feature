Feature: All Step Keywords
  Test that all Gherkin step keywords are supported
  Keywords: Given, When, Then, And, But

  Scenario: Using Given keyword
    Given I have a precondition
    And I have another precondition
    When I execute the test
    Then the test should pass

  Scenario: Using When keyword
    Given I have setup the environment
    When I perform an action
    And I perform another action
    Then the actions should complete

  Scenario: Using Then keyword
    Given I have executed the code
    When I check the results
    Then the results should be correct
    And the output should match expectations

  Scenario: Using And keyword for continuation
    Given I have initialized the system
    And I have loaded the configuration
    And I have connected to the database
    When I run the query
    Then I should get results
    And the results should not be empty
    And the query should complete quickly

  Scenario: Using But keyword for negation
    Given I have a valid user
    When I submit the form
    Then the form should be accepted
    But the password should not be stored in plain text
    And the session should be created

  Scenario: Mixing all keywords
    Given I have a complete test setup
    And all dependencies are available
    When I execute the full workflow
    And I monitor the execution
    Then the workflow should complete successfully
    And all assertions should pass
    But no errors should be logged
    And the performance should be acceptable
