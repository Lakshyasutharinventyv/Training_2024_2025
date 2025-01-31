export  default  {
  require: ["./steps/*.js"],
  format: ["progress", "json:test-results/cucumber-report.json"],
  paths: ["./features/*.feature"]
};
  