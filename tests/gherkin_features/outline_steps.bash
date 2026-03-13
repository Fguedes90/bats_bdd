#!/usr/bin/env bash
# Step definitions for scenario_outline.feature

step_when_i_calculate_2__3() {
  export CALC_RESULT=$((2 + 3))
}

step_then_the_result_should_be_5() {
  if [[ "$CALC_RESULT" -ne 5 ]]; then
    echo "Expected 5, got $CALC_RESULT" >&2
    return 1
  fi
}

step_when_i_calculate_10__5() {
  export CALC_RESULT=$((10 + 5))
}

step_then_the_result_should_be_15() {
  if [[ "$CALC_RESULT" -ne 15 ]]; then
    echo "Expected 15, got $CALC_RESULT" >&2
    return 1
  fi
}

step_when_i_calculate_100__200() {
  export CALC_RESULT=$((100 + 200))
}

step_then_the_result_should_be_300() {
  if [[ "$CALC_RESULT" -ne 300 ]]; then
    echo "Expected 300, got $CALC_RESULT" >&2
    return 1
  fi
}

step_when_i_calculate_10_5() {
  export CALC_RESULT=$((10 - 5))
}

step_then_the_result_should_be_5() {
  if [[ "$CALC_RESULT" -ne 5 ]]; then
    echo "Expected 5, got $CALC_RESULT" >&2
    return 1
  fi
}

step_when_i_calculate_20_8() {
  export CALC_RESULT=$((20 - 8))
}

step_then_the_result_should_be_12() {
  if [[ "$CALC_RESULT" -ne 12 ]]; then
    echo "Expected 12, got $CALC_RESULT" >&2
    return 1
  fi
}

step_when_i_calculate_100_50() {
  export CALC_RESULT=$((100 - 50))
}

step_then_the_result_should_be_50() {
  if [[ "$CALC_RESULT" -ne 50 ]]; then
    echo "Expected 50, got $CALC_RESULT" >&2
    return 1
  fi
}

step_given_i_have_the_string_hello_world() {
  export TEST_STRING="hello world"
}

step_when_i_check_if_it_contains_hello() {
  if [[ "$TEST_STRING" == *"hello"* ]]; then
    export STRING_MATCH="true"
  else
    export STRING_MATCH="false"
  fi
}

step_then_the_match_should_be_true() {
  if [[ "$STRING_MATCH" != "true" ]]; then
    echo "Expected match to be true, got $STRING_MATCH" >&2
    return 1
  fi
}

step_given_i_have_the_string_foo_bar() {
  export TEST_STRING="foo bar"
}

step_when_i_check_if_it_contains_baz() {
  if [[ "$TEST_STRING" == *"baz"* ]]; then
    export STRING_MATCH="true"
  else
    export STRING_MATCH="false"
  fi
}

step_then_the_match_should_be_false() {
  if [[ "$STRING_MATCH" != "false" ]]; then
    echo "Expected match to be false, got $STRING_MATCH" >&2
    return 1
  fi
}
