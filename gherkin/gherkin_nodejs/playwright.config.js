module.exports = {
    reporter: [
      ['html', { outputFolder: 'test-results', open: 'never' }],
      ['json', { outputFile: 'test-results/playwright-test.json', open: 'never' }],
    ],
  };
  