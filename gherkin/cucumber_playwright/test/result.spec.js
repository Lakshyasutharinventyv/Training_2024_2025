import { test, expect } from '@playwright/test';

test('result test', async ({ page }) => {
  await page.goto('https://charusat.edu.in:912/Uniexamresult/');
  await page.locator('#ddlInst').selectOption('21');
  await page.goto('https://charusat.edu.in:912/Uniexamresult/frmUniversityResult.aspx');
  await page.locator('#ddlDegree').selectOption('134');
  await page.goto('https://charusat.edu.in:912/Uniexamresult/frmUniversityResult.aspx');
  await page.getByRole('textbox', { name: 'Enter Enrollment No' }).click();
  await page.getByRole('textbox', { name: 'Enter Enrollment No' }).click();
  await page.getByRole('textbox', { name: 'Enter Enrollment No' }).fill('21dcs121');
  await page.getByRole('button', { name: 'Show Marksheet' }).click();
  await page.goto('https://charusat.edu.in:912/Uniexamresult/frmUniversityResult.aspx');
  await page.getByText('SUTHAR LAKSHYA AMRUTBHAI').click();
});