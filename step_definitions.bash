#!/usr/bin/env bash
# step_definitions.bash - Example step definitions for BATS-BDD
#
# This file contains user-defined step implementations.
# Each function name corresponds to a Gherkin step:
#   step_<keyword>_<text_in_snake_case>()
#
# Example:
#   Given I have a calculator
#   → step_given_i_have_a_calculator()

# Load BATS helper libraries (optional)
# load 'bats-support/load'
# load 'bats-assert/load'

# =============================================================================
# Calculator Example Steps
# =============================================================================

# Step: Given I have a calculator
step_given_i_have_a_calculator() {
  # Initialize calculator application path
  export CALCULATOR_APP="./examples/calculator.sh"
  
  # Or use a simple bash arithmetic for demonstration
  export CALCULATOR_RESULT=0
}

# Step: When I add 2 and 3
step_when_i_add_2_and_3() {
  # Simple bash arithmetic
  CALCULATOR_RESULT=$((2 + 3))
}

# Step: Then the result should be 5
step_then_the_result_should_be_5() {
  if [[ "$CALCULATOR_RESULT" -ne 5 ]]; then
    echo "Expected result to be 5, but got $CALCULATOR_RESULT" >&2
    return 1
  fi
}

# =============================================================================
# Command Execution Steps (Generic)
# =============================================================================

# Step: Given I run the command `echo "hello"`
step_given_i_run_the_command_echo_hello() {
  run echo "hello"
}

# Step: Then the command output should contain hello
step_then_the_command_output_should_contain_hello() {
  if [[ "$output" != *"hello"* ]]; then
    echo "Expected output to contain 'hello', but got: $output" >&2
    return 1
  fi
}

# Step: Then the command exit code should be 0
step_then_the_command_exit_code_should_be_0() {
  if [[ "$status" -ne 0 ]]; then
    echo "Expected exit code 0, but got $status" >&2
    return 1
  fi
}

# =============================================================================
# HTTP/API Steps (using curl)
# =============================================================================

# Step: Given I send a GET request to http://localhost:8080/health
step_given_i_send_a_get_request_to_http_localhost_8080_health() {
  HTTP_RESPONSE=$(curl -s -o /tmp/http_response_body.txt -w "%{http_code}" http://localhost:8080/health)
}

# Step: Then the response status should be 200
step_then_the_response_status_should_be_200() {
  if [[ "$HTTP_RESPONSE" != "200" ]]; then
    echo "Expected HTTP status 200, but got $HTTP_RESPONSE" >&2
    return 1
  fi
}

# Step: Then the response should contain ok
step_then_the_response_should_contain_ok() {
  if ! grep -q "ok" /tmp/http_response_body.txt; then
    echo "Expected response to contain 'ok'" >&2
    return 1
  fi
}

# =============================================================================
# File Operation Steps
# =============================================================================

# Step: Given I have a file named test.txt
step_given_i_have_a_file_named_test_txt() {
  touch /tmp/test.txt
}

# Step: When I write "hello world" to the file
step_when_i_write_hello_world_to_the_file() {
  echo "hello world" > /tmp/test.txt
}

# Step: Then the file should contain hello world
step_then_the_file_should_contain_hello_world() {
  if ! grep -q "hello world" /tmp/test.txt; then
    echo "Expected file to contain 'hello world'" >&2
    return 1
  fi
}

# =============================================================================
# Generic Steps with Parameters (using environment variables)
# =============================================================================

# Step: Given the configuration value is set to production
step_given_the_configuration_value_is_set_to_production() {
  export APP_ENV="production"
}

# Step: Then the environment should be production
step_then_the_environment_should_be_production() {
  if [[ "$APP_ENV" != "production" ]]; then
    echo "Expected APP_ENV to be 'production', but got '$APP_ENV'" >&2
    return 1
  fi
}
