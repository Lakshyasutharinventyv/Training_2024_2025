Feature: User Login
  As a user
  I want to log in to my account
  So that I can access my dashboard securely

  Scenario: Successful login with valid credentials
    Given the user is on the login page
    When the user enters a valid username and password
    And clicks the "Login" button
    Then the user should be redirected to the dashboard

  Scenario: Failed to login with invalid credentials
    Given the user is on the login page
    When the user enters an invalid username and password
    And clicks the "Login" button
    Then the user should see the message "Login Failed: Invalid credentials"
