Feature: University Results Page

  Scenario Outline: Check student results by enrollment number
    Given I am on the university results page
    When I select "<Institution>" from the "Institution" dropdown
    And I select "<Degree>" from the "Degree" dropdown
    And I select "<Semester>" from the "Semester" dropdown
    And I select "<Exam>" from the "Exam" dropdown
    And I enter "<Student ID>" in the "Student ID" field
    And I click the "Show Marksheet" button
    Then I should see "<Student Name>" on the page

  Examples:
    | Institution | Degree     | Semester | Exam           | Student ID | Student Name               |
    | DEPSTAR     | BTECH(CS)  | 7        | NOVEMBER 2024  | 21dcs121   | SUTHAR LAKSHYA AMRUTBHAI   |
    | DEPSTAR     | BTECH(CS)  | 7        | NOVEMBER 2024  | 21dcs136   | SUTHAR LAKSHYA AMRUTBHAI   |
    | DEPSTAR     | BTECH(CS)  | 7        | NOVEMBER 2024  | 21dcs136   | HARSH RAKESHKUMAR VARMA    |
