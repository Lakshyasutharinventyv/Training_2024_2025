import couchbase from "couchbase";
import { fileURLToPath } from "url";
import path from "path";
import fs from "fs/promises";
import dotenv from 'dotenv';
dotenv.config();

// Couchbase Configuration
const clusterConnStr = process.env.COUCHBASE_URI;
const username = process.env.COUCHBASE_USERNAME;
const password = process.env.COUCHBASE_PASSWORD;
const bucketName = process.env.COUCHBASE_BUCKET_NAME;


const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

let cluster, bucket, collection;

export async function connectToCouchBase() {
    try {
        cluster = await couchbase.connect(clusterConnStr, { username, password });
        bucket = cluster.bucket(bucketName);
        collection = bucket.defaultCollection();
        console.log("Connected to Couchbase");
    } catch (err) {
        console.error("Couchbase Connection Failed:", err);
        process.exit(1);
    }
}

// ✅ Save JSON to Couchbase
export async function saveJsonToCouchbase() {
    try {
        const filePath = path.join(__dirname, "test-result", "cucumber-report.json");
        const data = await fs.readFile(filePath, "utf8");
        const parsedJson = JSON.parse(data);

        const docId = `report-${Date.now()}`;
        await collection.upsert(docId, parsedJson);
        console.log("Test results saved to Couchbase successfully.");
    } catch (error) {
        console.error("Error saving data to Couchbase:", error.message);
    }
}