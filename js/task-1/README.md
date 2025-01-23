# Async/Await Example with Array Manipulation and Promises

This project demonstrates the use of JavaScript's `async/await` functionality in conjunction with Promises to handle asynchronous operations. The code performs array manipulation and calculates the sum of elements to determine if the total exceeds a specified threshold.

## Code Overview

### Main Async Function

The main function is an Immediately Invoked Async Function Expression (IIFE) that performs the following steps:
1. Defines an array `arr` with initial elements `[1, 2, 3, 4, 5]`.
2. Removes the first element of the array using `shift()`.
3. Calls the `helper` function, passing the modified array and the removed element as arguments.
4. Logs the result of the operation or any errors encountered.

### Helper Function

The `helper` function is an `async` function that:
1. Constructs a new array `arr2` by combining the provided element, additional numbers `[6, 7, -8, -9]`, and the rest of the original array.
2. Calculates the total sum of all elements in `arr2`.
3. Returns a Promise:
   - Resolves with a message `"more than 30"` if the sum exceeds 30.
   - Rejects with a message `"Less than 30."` if the sum is 30 or less.

### Example Output

Based on the initial array `[1, 2, 3, 4, 5]`, the code will:
1. Construct `arr2` as `[1, 6, 7, -8, -9, 2, 3, 4, 5]`.
2. Calculate the sum of elements in `arr2`: `1 + 6 + 7 - 8 - 9 + 2 + 3 + 4 + 5 = 11`.
3. Reject the Promise with the message `"Less than 30."`.
