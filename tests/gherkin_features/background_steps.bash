#!/usr/bin/env bash
# Step definitions for background.feature

step_given_i_initialize_the_test_environment() {
  export TEST_ENV="initialized"
}

step_and_i_set_the_default_timeout_to_30_seconds() {
  export TIMEOUT=30
}

step_when_i_run_the_first_test() {
  export CURRENT_TEST="first"
}

step_then_the_background_should_be_executed() {
  if [[ "$TEST_ENV" != "initialized" ]]; then
    echo "Background was not executed! TEST_ENV=$TEST_ENV" >&2
    return 1
  fi
  if [[ "$TIMEOUT" != "30" ]]; then
    echo "Background timeout not set! TIMEOUT=$TIMEOUT" >&2
    return 1
  fi
}

step_when_i_run_the_second_test() {
  export CURRENT_TEST="second"
}

step_then_the_background_should_be_executed_again() {
  if [[ "$TEST_ENV" != "initialized" ]]; then
    echo "Background was not executed again! TEST_ENV=$TEST_ENV" >&2
    return 1
  fi
}

step_given_i_verify_the_state_is_clean() {
  # Verify state is clean before test
  if [[ -n "$TEST_DIRTY" ]]; then
    echo "State is dirty!" >&2
    return 1
  fi
}

step_when_i_run_the_third_test() {
  export CURRENT_TEST="third"
}

step_then_each_scenario_should_have_fresh_background() {
  if [[ "$TEST_ENV" != "initialized" ]]; then
    echo "Background not fresh! TEST_ENV=$TEST_ENV" >&2
    return 1
  fi
}
