import express from "express";
import path from "path";
import { fileURLToPath } from "url";
import dotenv from "dotenv";
import { connectDB } from "./database/connection.js";
import fs from "fs/promises";
import mongoose from "mongoose";
import resultModel from "./database/models/result.model.js";

dotenv.config();
const app = express();

connectDB();

const port = 3000;
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function saveJsonToDB() {
    try {
        const filePath = "D:/internship_training/Training_2024_2025/gherkin/cucumber_playwright/test-result/cucumber-report.json";

        const data = await fs.readFile(filePath, "utf8");
        const jsonData = JSON.parse(data);

        console.log("Parsed JSON:", jsonData);
        let totalReports = await resultModel.countDocuments();
        
        await resultModel.create({ reportName:`Report ${totalReports+1}`, data: JSON.stringify(jsonData) });

        console.log("Test results saved to MongoDB successfully.");
        await mongoose.connection.close();
        return jsonData
    } catch (error) {
        console.error("Error saving data to the database:", error);
    }
}

app.get("/", async (req, res) => {
    let data = await saveJsonToDB();
    res.sendFile(path.join(__dirname, "test-result", "cucumber-report.html"));
});

app.listen(port, () => {
    console.log(`App is running on port ${port}`);
});
