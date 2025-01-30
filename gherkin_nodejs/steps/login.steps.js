const { Given, When, Then, After } = require('@cucumber/cucumber');
const { chromium } = require('playwright');

let browser, page;

Given('the user is on the login page', async function () {
  browser = await chromium.launch({ headless: false });
  page = await browser.newPage();
  await page.goto('http://localhost:3000/login');
});

When('the user enters a valid username and password', async function () {
  await page.fill('#username', 'test');
  await page.fill('#password', 'password');
});

When('clicks the "Login" button', async function () {
  await page.click('#loginButton');
});

Then('the user should be redirected to the dashboard', async function () {

  await page.waitForURL('http://localhost:3000/dashboard');
  
  const dashboardText = await page.textContent('h1');
  if (dashboardText !== 'Welcome to the Dashboard') {
    throw new Error("Redirect to dashboard failed.");
  }
});

When('the user enters an invalid username and password', async function () {
  await page.fill('#username', 'invalid'); 
  await page.fill('#password', 'wrongpassword'); 
});

Then('the user should see the message {string}', async function (expectedMessage) {
  const errorMessage = await page.textContent('h1');
  if (errorMessage !== expectedMessage) {
    throw new Error(`Expected message to be "${expectedMessage}" but got "${errorMessage}".`);
  }
});

After(async function () {
  await browser.close(); 
});
