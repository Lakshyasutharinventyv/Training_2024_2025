import fs from 'fs/promises';
import { expect } from '@playwright/test';
import { chromium } from 'playwright';
import { setDefaultTimeout, Given, When, Then, After } from '@cucumber/cucumber';
import TestResult from "../../database/models/result.model.js";
import {connectDB} from "../../database/connection.js";
import mongoose from 'mongoose';


setDefaultTimeout(100000);

let browser, page;

Given('I am on the university results page', async function () {
    browser = await chromium.launch({ headless: true });
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
   
    await browser.close(); 
});

