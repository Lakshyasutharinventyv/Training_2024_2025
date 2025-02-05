import { expect } from '@playwright/test';
import { chromium } from 'playwright';
import { setDefaultTimeout, Given, When, Then, After } from '@cucumber/cucumber';
import * as Minio from 'minio';
import fs from 'fs';
import path from 'path';

setDefaultTimeout(100000); 

let browser, page;

const minioClient = new Minio.Client({
    endPoint: 'localhost', 
    port: 9000,
    useSSL: false, 
    accessKey: 'wPVWRNgcDlV9NUsxCfWr', 
    secretKey: 'OEYe6QxvhzILODeELT5rNkiW9iBv6t5gfHwmDmOl'
});

const bucketName = 'cucumber-playwright';
const screenshotDir = './screenshots';

if (!fs.existsSync(screenshotDir)) {
    fs.mkdirSync(screenshotDir, { recursive: true });
}

Given('I am on the university results page', async function () {
    browser = await chromium.launch({ headless: true });
    page = await browser.newPage();
    await page.goto('https://charusat.edu.in:912/Uniexamresult/');
    await takeScreenshot('university_results_page');
});

When('I select {string} from the "Institution" dropdown', async function (institution) {
    await page.selectOption('#ddlInst', { label: institution });
    await takeScreenshot('institution_selected');
});

When('I select {string} from the "Degree" dropdown', async function (degree) {
    await page.selectOption('#ddlDegree', { label: degree });
    await takeScreenshot('degree_selected');
});

When('I select {string} from the "Semester" dropdown', async function (semester) {
    await page.selectOption('#ddlSem', { label: semester });
    await takeScreenshot('semester_selected');
});

When('I select {string} from the "Exam" dropdown', async function (exam) {
    await page.selectOption('#ddlScheduleExam', { label: exam });
    await takeScreenshot('exam_selected');
});

When('I enter {string} in the "Student ID" field', async function (studentId) {
    await page.fill('#txtEnrNo', studentId);
    await takeScreenshot('student_id_entered');
});

When('I click the "Show Marksheet" button', async function () {
    await page.click('#btnSearch');
    await takeScreenshot('marksheet_shown');
});

Then('I should see {string} on the page', async function (expectedText) {
    const studentNameLocator = page.locator('#uclGrd1_lblStudentName');
    await expect(studentNameLocator).toHaveText(expectedText);
    await takeScreenshot('student_name_verified');
});

After(async function () {
    await browser.close();
});


async function takeScreenshot(stepName) {
    const timestamp = Date.now();
    const screenshotPath = path.join(screenshotDir, `screenshot-${stepName}-${timestamp}.png`);

    await page.screenshot({ path: screenshotPath });
    console.log(`Screenshot saved locally: ${screenshotPath}`);


    try {
        await minioClient.fPutObject(bucketName, path.basename(screenshotPath), screenshotPath);
        console.log(`Screenshot uploaded successfully to MinIO: ${screenshotPath}`);
    } catch (error) {
        console.error(`Failed to upload screenshot to MinIO: ${error.message}`);
    }
}
