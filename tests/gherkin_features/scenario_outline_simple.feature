Feature: Simple Scenario Outline
  Test basic scenario outline functionality

  Scenario Outline: Add numbers
    When I add <a> and <b>
    Then the result should be <sum>

    Examples:
| a | b | sum |
| 1 | 2 | 3   |
| 3 | 4 | 7   |
| 5 | 6 | 11  |