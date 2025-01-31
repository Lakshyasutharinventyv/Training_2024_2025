import express from "express";
import path from "path"
import { fileURLToPath } from 'url';
import dotenv from "dotenv";
import {connectDB} from "./database/connection.js";
dotenv.config();
const app = express();

connectDB();

const port = 3000;
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);


app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'public', 'index.html'));
});

app.listen(port,()=>{
    console.log(`App is running on port ${port}`);
})