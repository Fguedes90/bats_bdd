Feature: Calculator
  Basic arithmetic operations for demonstration

  Background:
    Given I have a calculator

  Scenario: Add two numbers
    When I add 2 and 3
    Then the result should be 5

  Scenario: Subtract two numbers
    When I subtract 5 from 10
    Then the result should be 5

  Scenario: Multiply two numbers
    When I multiply 3 and 4
    Then the result should be 12
