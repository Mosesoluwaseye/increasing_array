# Increasing Array

A Rust implementation of the CSES Increasing Array problem using two different algorithms.

## Problem Description

Given an array of integers, determine the minimum number of moves required to make the array non-decreasing.

A move consists of increasing any element by one.

Example:

Input:

5
3 2 5 1 7

Output:

5

---

## Project Structure

```text
increasing_array/
├── src/
│   ├── lib.rs
│   └── main.rs
├── tests/
│   └── increasing_array_tests.rs
├── benches/
│   └── benchmark.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## Algorithm 1: Greedy Approach

### Description

The greedy algorithm scans the array from left to right while maintaining the maximum value encountered so far.

If the current element is smaller than the maximum value, the difference is added to the move count. Otherwise, the maximum value is updated.

### Time Complexity

O(n)

### Space Complexity

O(1)

---

## Algorithm 2: Naive Incremental Approach

### Description

The naive algorithm processes the array from left to right. Whenever an element is smaller than the previous valid value, it repeatedly increments the element one step at a time until it becomes large enough.

This directly simulates the required operations.

### Time Complexity

O(n + m)

where m is the number of performed increments.

### Space Complexity

O(1)

---

## Benchmark Results

| Algorithm | Time      |
| --------- | --------- |
| Greedy    | 15.239 ns |
| Naive     | 17.108 ns |

---

## Benchmark Interpretation

Both algorithms produce the same correct result.

The greedy algorithm performs better because it computes the required number of moves directly using arithmetic operations. Each element is processed exactly once, giving linear time complexity.

The naive algorithm simulates every increment individually. Although it remains correct, it performs additional work whenever large adjustments are required. This increases execution time.

From the benchmark results, the greedy implementation is faster than the naive implementation.

The greedy algorithm also exhibits better cache behaviour because it performs a simple sequential scan over the array with minimal operations. The naive algorithm performs additional loop iterations, increasing instruction count and CPU workload.

For large inputs, the performance gap grows significantly because the greedy algorithm scales linearly while the naive algorithm depends on both the number of elements and the total number of increments required.

Therefore, the greedy algorithm is the preferred solution for production use.

---

## Running the Program

```bash
cargo run
```

---

## Running Tests

```bash
cargo test
```

---

## Running Benchmarks

```bash
cargo bench
```
