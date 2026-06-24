use increasing_array::increasing_array;

#[test]
fn test_example_case() {
    let nums = vec![3, 2, 5, 1, 7];
    assert_eq!(increasing_array(&nums), 5);
}

#[test]
fn test_already_increasing() {
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(increasing_array(&nums), 0);
}

#[test]
fn test_all_equal() {
    let nums = vec![5, 5, 5, 5];
    assert_eq!(increasing_array(&nums), 0);
}

#[test]
fn test_decreasing() {
    let nums = vec![5, 4, 3, 2, 1];
    assert_eq!(increasing_array(&nums), 10);
}