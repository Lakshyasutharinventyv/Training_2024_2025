const { test, expect } = require('@playwright/test');

test('Login valid test', async ({ page }) => {
 
  await page.goto('http://localhost:3000/login');
  
  await page.fill('#username', 'test');
  await page.fill('#password', 'password');
  await page.click('#loginButton');
  
  await page.waitForURL('http://localhost:3000/dashboard');
  const dashboardText = await page.textContent('h1');
  expect(dashboardText).toBe('Welcome to the Dashboard');
});

test('Login invalid test', async ({ page }) => {
  await page.goto('http://localhost:3000/login');
  
  await page.fill('#username', 'test_invalid');
  await page.fill('#password', 'password');
  await page.click('#loginButton');
  
  const errorMessage = await page.textContent('h1');
  expect(errorMessage).toBe('Login Failed: Invalid credentials');
});
