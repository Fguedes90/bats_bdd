#!/usr/bin/env bash
# Step definitions for all_step_keywords.feature

# Scenario: Using Given keyword
step_given_i_have_a_precondition() {
  export PRECONDITION_1="true"
}

step_given_i_have_another_precondition() {
  export PRECONDITION_2="true"
}

step_when_i_execute_the_test() {
  export TEST_EXECUTED="true"
}

step_then_the_test_should_pass() {
  if [[ "$PRECONDITION_1" != "true" ]] || [[ "$PRECONDITION_2" != "true" ]]; then
    echo "Preconditions not met" >&2
    return 1
  fi
}

# Scenario: Using When keyword
step_given_i_have_setup_the_environment() {
  export ENV_SETUP="true"
}

step_when_i_perform_an_action() {
  export ACTION_1="performed"
}

step_when_i_perform_another_action() {
  export ACTION_2="performed"
}

step_then_the_actions_should_complete() {
  if [[ "$ACTION_1" != "performed" ]] || [[ "$ACTION_2" != "performed" ]]; then
    echo "Actions not completed" >&2
    return 1
  fi
}

# Scenario: Using Then keyword
step_given_i_have_executed_the_code() {
  export CODE_EXECUTED="true"
}

step_when_i_check_the_results() {
  export RESULTS_CHECKED="true"
}

step_then_the_results_should_be_correct() {
  if [[ "$CODE_EXECUTED" != "true" ]]; then
    echo "Code not executed" >&2
    return 1
  fi
}

step_then_the_output_should_match_expectations() {
  if [[ "$RESULTS_CHECKED" != "true" ]]; then
    echo "Results not checked" >&2
    return 1
  fi
}

# Scenario: Using And keyword for continuation
step_given_i_have_initialized_the_system() {
  export SYSTEM_INITIALIZED="true"
}

step_given_i_have_loaded_the_configuration() {
  export CONFIG_LOADED="true"
}

step_given_i_have_connected_to_the_database() {
  export DB_CONNECTED="true"
}

step_when_i_run_the_query() {
  export QUERY_EXECUTED="true"
}

step_then_i_should_get_results() {
  if [[ "$SYSTEM_INITIALIZED" != "true" ]] || [[ "$CONFIG_LOADED" != "true" ]] || [[ "$DB_CONNECTED" != "true" ]]; then
    echo "Prerequisites not met" >&2
    return 1
  fi
}

step_then_the_results_should_not_be_empty() {
  if [[ "$QUERY_EXECUTED" != "true" ]]; then
    echo "Query not executed" >&2
    return 1
  fi
}

step_then_the_query_should_complete_quickly() {
  # Performance check (placeholder)
  export PERFORMANCE_OK="true"
}

# Scenario: Using But keyword for negation
step_given_i_have_a_valid_user() {
  export USER_VALID="true"
}

step_when_i_submit_the_form() {
  export FORM_SUBMITTED="true"
}

step_then_the_form_should_be_accepted() {
  if [[ "$USER_VALID" != "true" ]]; then
    echo "User not valid" >&2
    return 1
  fi
}

step_but_the_password_should_not_be_stored_in_plain_text() {
  # Security assertion - password should be hashed
  export PASSWORD_HASHED="true"
}

step_then_the_session_should_be_created() {
  if [[ "$FORM_SUBMITTED" != "true" ]] || [[ "$PASSWORD_HASHED" != "true" ]]; then
    echo "Session requirements not met" >&2
    return 1
  fi
}

# Scenario: Mixing all keywords
step_given_i_have_a_complete_test_setup() {
  export TEST_SETUP="complete"
}

step_given_i_have_all_dependencies_available() {
  export DEPENDENCIES="available"
}

step_when_i_execute_the_full_workflow() {
  export WORKFLOW_EXECUTED="true"
}

step_when_i_monitor_the_execution() {
  export EXECUTION_MONITORED="true"
}

step_then_the_workflow_should_complete_successfully() {
  if [[ "$TEST_SETUP" != "complete" ]] || [[ "$DEPENDENCIES" != "available" ]]; then
    echo "Setup incomplete" >&2
    return 1
  fi
}

step_then_all_assertions_should_pass() {
  if [[ "$WORKFLOW_EXECUTED" != "true" ]]; then
    echo "Workflow not executed" >&2
    return 1
  fi
}

step_but_no_errors_should_be_logged() {
  # Verify no errors
  export NO_ERRORS="true"
}

step_then_the_performance_should_be_acceptable() {
  if [[ "$EXECUTION_MONITORED" != "true" ]] || [[ "$NO_ERRORS" != "true" ]]; then
    echo "Monitoring or errors issue" >&2
    return 1
  fi
}
