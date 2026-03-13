#!/usr/bin/env bash
# Step definitions for calculator.feature example

step_given_i_have_a_calculator() {
  export CALCULATOR_RESULT=0
}

step_when_i_add_2_and_3() {
  CALCULATOR_RESULT=$((2 + 3))
}

step_when_i_subtract_5_from_10() {
  CALCULATOR_RESULT=$((10 - 5))
}

step_when_i_multiply_3_and_4() {
  CALCULATOR_RESULT=$((3 * 4))
}

step_then_the_result_should_be_5() {
  if [[ "$CALCULATOR_RESULT" -ne 5 ]]; then
    echo "Expected result to be 5, but got $CALCULATOR_RESULT" >&2
    return 1
  fi
}

step_then_the_result_should_be_12() {
  if [[ "$CALCULATOR_RESULT" -ne 12 ]]; then
    echo "Expected result to be 12, but got $CALCULATOR_RESULT" >&2
    return 1
  fi
}
