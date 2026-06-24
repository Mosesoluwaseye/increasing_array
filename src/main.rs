use std::io;
use increasing_array::increasing_array;

fn main() {
    let mut n_line = String::new();
    io::stdin().read_line(&mut n_line).unwrap();

    let mut numbers_line = String::new();
    io::stdin().read_line(&mut numbers_line).unwrap();

    let numbers: Vec<u64> = numbers_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", increasing_array(&numbers));
}