import mongoose from "mongoose";
import { fileURLToPath } from "url";
import path from "path";
import fs from "fs/promises";
import dotenv from 'dotenv';
dotenv.config();

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// MongoDB Configuration
const mongoURI = process.env.MONGO_URI;

const resultSchema = new mongoose.Schema({
    reportName: String,
    data: String,
});
const resultModel = mongoose.model("TestReport", resultSchema);


// Connect to MongoDB
export async function connectToMongoDB() {
    try {
        await mongoose.connect(mongoURI, { useNewUrlParser: true, useUnifiedTopology: true });
        console.log("Connected to MongoDB");
    } catch (err) {
        console.error("MongoDB Connection Failed:", err);
        process.exit(1);
    }
}





// Save JSON to MongoDB
export async function saveJsonToMongoDB() {
    try {
        const filePath = path.join(__dirname, "test-result", "cucumber-report.json");
        const data = await fs.readFile(filePath, "utf8");
        const jsonData = JSON.parse(data);

        let totalReports = await resultModel.countDocuments();
        await resultModel.create({ reportName: `Report ${totalReports + 1}`, data: JSON.stringify(jsonData) });

        console.log("Test results saved to MongoDB successfully.");
    } catch (error) {
        console.error("Error saving data to MongoDB:", error);
    }
}

