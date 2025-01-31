import mongoose from 'mongoose';


const testResultSchema = new mongoose.Schema({
  description: String,
  elements: [
    {
      description: String,
      id: String,
      keyword: String,
      name: String,
      steps: [
        {
          arguments: [String],
          keyword: String,
          line: Number,
          name: String,
          match: {
            location: String,
          },
          result: {
            status: String,
            duration: Number,
          },
        },
      ],
      tags: [String],
      type: String,
    },
  ],
  id: String,
  line: Number,
  keyword: String,
  name: String,
  tags: [String],
  uri: String,
});

const TestResult = mongoose.model('TestResult', testResultSchema);

export default TestResult;
