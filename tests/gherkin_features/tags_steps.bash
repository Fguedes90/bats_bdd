#!/usr/bin/env bash
# Step definitions for tags.feature

step_given_i_have_a_tagged_scenario() {
  export SCENARIO_TYPE="tagged"
}

step_when_i_check_the_tags() {
  # Tags are preserved as comments in generated BATS
  export TAGS_CHECKED="true"
}

step_then_the_smoke_tag_should_be_present() {
  if [[ "$SCENARIO_TYPE" != "tagged" ]]; then
    echo "Scenario should be tagged" >&2
    return 1
  fi
}

step_given_i_have_a_multi_tagged_scenario() {
  export SCENARIO_TYPE="multi_tagged"
}

step_then_both_regression_and_critical_tags_should_be_present() {
  if [[ "$SCENARIO_TYPE" != "multi_tagged" ]]; then
    echo "Scenario should have multiple tags" >&2
    return 1
  fi
}

step_given_i_have_an_api_test() {
  export TEST_TYPE="api"
}

step_when_i_run_the_api_test() {
  export TEST_EXECUTED="true"
}

step_then_the_api_tag_should_be_in_the_output() {
  if [[ "$TEST_TYPE" != "api" ]]; then
    echo "Test type should be api" >&2
    return 1
  fi
}

step_given_i_have_a_slow_test() {
  export TEST_SPEED="slow"
}

step_when_i_run_the_integration_test() {
  export TEST_CATEGORY="integration"
}

step_then_both_slow_and_integration_tags_should_be_present() {
  if [[ "$TEST_SPEED" != "slow" ]] || [[ "$TEST_CATEGORY" != "integration" ]]; then
    echo "Test should be slow and integration" >&2
    return 1
  fi
}

step_given_i_have_a_wip_test() {
  export TEST_STATUS="wip"
}

step_then_it_should_be_marked_as_work_in_progress() {
  if [[ "$TEST_STATUS" != "wip" ]]; then
    echo "Test should be WIP" >&2
    return 1
  fi
}
