const fs = require("fs");
const reporter = require("cucumber-html-reporter");

const jsonFile = "test-result/cucumber-report.json";

// Ensure JSON file exists before proceeding
if (!fs.existsSync(jsonFile) || fs.statSync(jsonFile).size === 0) {
    console.error("❌ Error: JSON report is missing or empty. Run your tests first.");
    process.exit(1);
}

const options = {
    theme: "bootstrap",
    jsonFile: jsonFile,
    output: "test-result/custom-cucumber-report.html",
    reportSuiteAsScenarios: true,
    scenarioTimestamp: true,
    launchReport: true,  // Opens the report automatically
    metadata: {
        "Test Environment": "QA",
        "Browser": "Chrome 120",
        "Platform": "Windows 11",
        "Executed": "Remote"
    }
};

// Generate the report
reporter.generate(options);
console.log("✅ Custom HTML report generated at test-result/custom-cucumber-report.html");
