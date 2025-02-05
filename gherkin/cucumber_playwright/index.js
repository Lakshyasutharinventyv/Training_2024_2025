import express from "express";
import { fileURLToPath } from "url";
import path from "path";



import {connectToTiDB,saveJsonToTiDB} from "./tidb.js"
import {connectToCouchBase,saveJsonToCouchbase} from "./couchbase.js";
import {connectToMongoDB,saveJsonToMongoDB} from "./mongodb.js"


const app = express();
const port = 3000;

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

(async () => {
    // await connectToCouchBase();
    await connectToTiDB();
    // await connectToMongoDB();

    app.listen(port, () => {
        console.log(`Server running on http://localhost:${port}`);
    });
})();


app.get("/", async (req, res) => {
    try {
        // await saveJsonToCouchbase();
        await saveJsonToTiDB();
        // await saveJsonToMongoDB();
        res.sendFile(path.join(__dirname, "test-result", "cucumber-report.html"));
    } catch (error) {
        res.status(500).send("Failed to save data or send report");
    }
});
