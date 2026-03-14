#!/usr/bin/env bash
# Step definitions for scenario_outline.feature
# Using parameterized architecture to match transpiler output

# Calculator functions with parameters
step_when_i_add_a_and_b() {
  local a="$1"
  local b="$2"
  export CALC_RESULT=$((a + b))
}

step_when_i_subtract_b_from_a() {
  local a="$1"
  local b="$2"
  export CALC_RESULT=$((a - b))
}

step_then_the_result_should_be() {
  local expected="$1"
  if [[ "$CALC_RESULT" -ne "$expected" ]]; then
    echo "Expected $expected, got $CALC_RESULT" >&2
    return 1
  fi
}

# String matching functions
step_given_i_have_the_string() {
  local str="$1"
  export TEST_STRING="$str"
}

step_when_i_check_if_it_contains() {
  local substring="$1"
  if [[ "$TEST_STRING" == *"$substring"* ]]; then
    export STRING_MATCH="true"
  else
    export STRING_MATCH="false"
  fi
}

step_then_the_match_should_be() {
  local expected="$1"
  if [[ "$STRING_MATCH" != "$expected" ]]; then
    echo "Expected match to be $expected, got $STRING_MATCH" >&2
    return 1
  fi
}
