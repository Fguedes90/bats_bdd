@smoke@api
Feature: API Tests
  Tests for API endpoints

  Scenario: Health check
    Given the API is running
    Then the health endpoint should return 200
