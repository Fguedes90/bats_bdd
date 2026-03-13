#!/usr/bin/env bash
# Step definitions for scenario_outline_simple.feature
# Updated to use parameterized functions (new architecture)

# Step: When I add <a> and <b>
# Now accepts arguments instead of requiring unique functions per row
step_when_i_add_a_and_b() {
  local a="$1"
  local b="$2"
  export RESULT=$((a + b))
}

# Step: Then the result should be <sum>
step_then_the_result_should_be_sum() {
  local expected="$1"
  if [[ "$RESULT" -ne "$expected" ]]; then
    echo "Expected $expected, got $RESULT" >&2
    return 1
  fi
}
