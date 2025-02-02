Feature: University Results Page

  Scenario: Check student results by enrollment number
    Given I am on the university results page
    When I select "DEPSTAR" from the "Institution" dropdown
    And I select "BTECH(CS)" from the "Degree" dropdown
    And I select "7" from the "Semester" dropdown
    And I select "NOVEMBER 2024" from the "Exam" dropdown
    And I enter "21dcs121" in the "Student ID" field
    And I click the "Show Marksheet" button
    Then I should see "SUTHAR LAKSHYA AMRUTBHAI" on the page
