import fs from 'fs/promises'; // Use ES Module import syntax
import { expect } from '@playwright/test';
import { chromium } from 'playwright';
import { setDefaultTimeout, Given, When, Then, After } from '@cucumber/cucumber';
import testResultModel from "../database/models/result.model.js";

setDefaultTimeout(25000);  // Set default timeout for Cucumber steps

let browser, page;

Given('I am on the university results page', async function () {
    browser = await chromium.launch({ headless: false });
    page = await browser.newPage();
    await page.goto('https://charusat.edu.in:912/Uniexamresult/');
});

When('I select {string} from the "Institution" dropdown', async function (institution) {
    await page.selectOption('#ddlInst', { label: institution });
});

When('I select {string} from the "Degree" dropdown', async function (degree) {
    await page.selectOption('#ddlDegree', { label: degree });
});

When('I select {string} from the "Semester" dropdown', async function (semester) {
    await page.selectOption('#ddlSem', { label: semester });
});

When('I select {string} from the "Exam" dropdown', async function (exam) {
    await page.selectOption('#ddlScheduleExam', { label: exam });
});

When('I enter {string} in the "Student ID" field', async function (studentId) {
    await page.fill('#txtEnrNo', studentId);
});

When('I click the "Show Marksheet" button', async function () {
    await page.click('#btnSearch');
});

Then('I should see {string} on the page', async function (expectedText) {
    const studentNameLocator = page.locator('#uclGrd1_lblStudentName');
    await expect(studentNameLocator).toHaveText(expectedText);
});

After(async function () {
    await saveJsonToDB();
    await browser.close(); 
});

async function saveJsonToDB() {
    try {
        const filePath = 'D:/internship_training/Training_2024_2025/gherkin/cucumber_playwright/test-results/cucumber-report.json';
        
        // Wait for the file to exist and ensure it has content
        let fileExists = false;
        let fileHasContent = false;

        while (!fileExists || !fileHasContent) {
            try {
                await fs.access(filePath);
                fileExists = true;
                const resultData = await fs.readFile(filePath, 'utf8');
                fileHasContent = resultData.trim().length > 0;  // Ensure file has content
            } catch (err) {
                console.log("Waiting for file to be generated...");
                await new Promise(resolve => setTimeout(resolve, 1000)); // Wait for 1 second before checking again
            }
        }

        const resultData = await fs.readFile(filePath, 'utf8');
        
        const jsonData = JSON.parse(resultData); // Parse the JSON data
        console.log(jsonData);  // Log the data to check its structure

        // Save the result data following the correct schema structure
        for (const result of jsonData) {
        //     const testResultData = {
        //         description: result.description,
        //         elements: result.elements.map(element => ({
        //             description: element.description,
        //             id: element.id,
        //             keyword: element.keyword,
        //             name: element.name,
        //             steps: element.steps.map(step => ({
        //                 arguments: step.arguments,
        //                 keyword: step.keyword,
        //                 line: step.line,
        //                 name: step.name,
        //                 match: step.match,
        //                 result: step.result
        //             })),
        //             tags: element.tags,
        //             type: element.type,
        //         })),
        //         id: result.id,
        //         line: result.line,
        //         keyword: result.keyword,
        //         name: result.name,
        //         tags: result.tags,
        //         uri: result.uri,
        //     };
        //     console.log(testResultData);
            
        }
        //     // Create a new document for the test result
        //     const testResult = new testResultModel(testResultData);
        //     await testResult.save();
        // }

        console.log('Test results saved to MongoDB successfully.');
    } catch (error) {
        console.error('Error saving data to the database:', error);
    }
}
