import mongoose from 'mongoose';


const testResultSchema = new mongoose.Schema({
  reportName:String,
  data:String
});

const TestResult = mongoose.model('TestResult', testResultSchema);

export default TestResult;
