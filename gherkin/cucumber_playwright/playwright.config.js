import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './test', 
  use: {
    headless: false,  
  },
}); 