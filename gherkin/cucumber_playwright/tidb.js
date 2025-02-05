import { fileURLToPath } from "url";
import path from "path";
import fs from "fs/promises";
import dotenv from 'dotenv';
dotenv.config();

import mysql from "mysql2/promise";

// CREATE TABLE reports (
//     id INT PRIMARY KEY AUTO_INCREMENT,
//     report_name VARCHAR(255) NOT NULL,
//     report_data TEXT NOT NULL
// );


// TiDB Configuration
const tidbConfig = {
    host: process.env.TIDB_HOST,
    user: process.env.TIDB_USER,
    password: process.env.TIDB_PASSWORD,
    database: process.env.TIDB_DATABASE,
    port: process.env.TIDB_PORT,
  };
  
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
let tidbConnection;

// Connect to TiDB
export async function connectToTiDB() {
    try {
        tidbConnection = await mysql.createConnection(tidbConfig);
        console.log("Connected to TiDB");
    } catch (err) {
        console.error("TiDB Connection Failed:", err);
        process.exit(1);
    }
}

// Save JSON to TiDB
export async function saveJsonToTiDB() {
    try {
        const filePath = path.join(__dirname, "test-result", "cucumber-report.json");
        const data = await fs.readFile(filePath, "utf8");
        const parsedJson = JSON.stringify(JSON.parse(data));

        const query = `INSERT INTO reports (report_name, report_data) VALUES (?, ?)`;
        const values = [`Report-${Date.now()}`, parsedJson];

        await tidbConnection.execute(query, values);
        console.log("Test results saved to TiDB successfully.");
    } catch (error) {
        console.error("Error saving data to TiDB:", error.message);
    }
}