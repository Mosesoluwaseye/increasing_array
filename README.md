# Increasing Array

Rust solution for the CSES Increasing Array problem.

## Problem

Given an array of integers, determine the minimum number of moves required to make the array non-decreasing.

A move consists of increasing an element by one.

## Algorithm

Traverse the array from left to right.

* Keep track of the largest value seen so far.
* If the current value is smaller than the largest value, add the difference to the move count.
* Otherwise update the largest value.

## Time Complexity

O(n)

## Space Complexity

O(1)

## Project Structure

```text
increasing_array/
├── src/
│   ├── lib.rs
│   └── main.rs
├── tests/
│   └── increasing_array_tests.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Run

```bash
cargo run
```

## Run Tests

```bash
cargo test
```
