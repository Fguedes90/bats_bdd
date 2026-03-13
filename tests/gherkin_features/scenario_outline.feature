Feature: Scenario Outline with Examples
  Test that scenario outlines generate multiple test cases
  Each example row should create a separate @test block

  Scenario Outline: Basic arithmetic operations
    When I calculate <a> <operation> <b>
    Then the result should be <expected>

    Examples: Addition
| a | operation | b | expected |
| 2 | + | 3 | 5 |
| 10 | + | 5 | 15 |
| 100 | + | 200 | 300 |

  Scenario Outline: Subtraction examples
    When I calculate <x> - <y>
    Then the result should be <result>

    Examples:
| x | y | result |
| 10 | 5 | 5 |
| 20 | 8 | 12 |
| 100 | 50 | 50 |

  Scenario Outline: String operations
    Given I have the string "<input>"
    When I check if it contains "<substring>"
    Then the match should be <matches>

    Examples:
| input | substring | matches |
| hello world | hello | true |
| hello world | world | true |
| hello world | foo | false |
