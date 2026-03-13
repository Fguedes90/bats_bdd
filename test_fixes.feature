Feature: System command execution
  Test that system steps correctly extract regex capture groups

  Background:
    Given I run the command `echo "background setup"`

  Scenario: Execute command and check output
    Given I run the command `echo "hello world"`
    Then the command output should contain hello
    And the command exit code should be 0

  Scenario: Execute command and save variable
    Given I run the command `echo "test-value"` and save the output as MY_VAR
    Then the command output should contain test-value
