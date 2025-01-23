# Symbol-Based Sum Calculator with Promises
This project demonstrates advanced JavaScript concepts, including the use of `Symbol` objects, array manipulation, type conversion, and Promises for conditional logic based on computed values.

## Code Overview

### Key Features

1. **`sum_of_array` Function**:
   - Accepts an `element` and an array (`arr`).
   - Creates a new array (`arr_2`) by combining the `element`, additional `Symbol` objects, and the elements of the input array.
   - Converts the descriptions of all `Symbol` objects in `arr_2` to numbers and calculates their sum.
   - Returns a Promise:
     - Resolves with the message `"Sum is greater than 30"` if the sum exceeds 30.
     - Rejects with the message `"Sum is less than 30"` otherwise.

2. **Self-Invoking Function**:
   - Creates an initial array `arr_1` of `Symbol` objects.
   - Removes the first element using `shift()` and passes it, along with the remaining array, to `sum_of_array`.

### Example Workflow

1. **Initial Array (`arr_1`)**:
   - Contains `Symbol` objects: `[Symbol(1), Symbol(2), Symbol("3"), Symbol("4"), Symbol("5")]`.

2. **Processing in `sum_of_array`**:
   - Combines the removed `Symbol(1)` with new `Symbol` objects (`Symbol("99")`, `Symbol("1")`, `Symbol("1")`) and the remaining array to create `arr_2`.
   - Converts `Symbol` descriptions (e.g., `Symbol("1")` → `"1"`) to numbers.
   - Calculates the sum of these values.

3. **Promise Handling**:
   - Logs whether the sum exceeds 30 or not based on the Promise result.

### Example Output

Given the initial `arr_1`:
- `arr_2` will be: `[Symbol(1), Symbol("99"), Symbol("1"), Symbol("1"), Symbol(2), Symbol("3"), Symbol("4"), Symbol("5")]`.
- Sum calculation:
  - `1 + 99 + 1 + 1 + 2 + 3 + 4 + 5 = 116`
- Result:
  - The Promise resolves with the message `"Sum is greater than 30"`.

