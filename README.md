# Sorting Algorithms in Rust

## Performance

Average performances of algorithms in 100 iterations f `Vec`s of sizes `4`, `16`, `256`, `2_048`:

| Algorithm    | Time (seconds) |
| ------------ | -------------- |
| rust stable  | 0.00012s       |
| quicksort    | 0.00016s       |
| heapsort     | 0.00022s       |
| merge        | 0.00024s       |
| insertion    | 0.00344s       |
| selection    | 0.00809s       |
| bubble       | 0.01431s       |

## Time and Space Complexity

| Algorithm    | Time (best)   | Time (average) | Time (worst)  | Space (worst) |
| ------------ | ------------- | -------------- | ------------- | ------------- |
| Rust Stable  | O(n)          | O(n log n)     | O(n log n)    | ?             |
| Quicksort    | O(n log n)    | O(n log n)     | O(n^2)        | O(log n)      |
| Heapsort     | O(n log n)    | O(n log n)     | O(n log n)    | O(1)          |
| Merge        | O(n log n)    | O(n log n)     | O(n log n)    | O(n)          |
| Insertion    | O(n)          | O(n^2)         | O(n^2)        | O(1)          |
| Selection    | O(n^2)        | O(n^2)         | O(n^2)        | O(1)          |
| Bubble       | O(n)          | O(n^2)         | O(n^2)        | O(1)          |
