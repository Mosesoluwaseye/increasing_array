pub fn increasing_array(numbers: &[u64]) -> u64 {
    let mut moves = 0;
    let mut current_max = numbers[0];

    for &num in numbers.iter().skip(1) {
        if num < current_max {
            moves += current_max - num;
        } else {
            current_max = num;
        }
    }

    moves
}

pub fn increasing_array_naive(numbers: &[u64]) -> u64 {
    let mut moves = 0;
    let mut current = numbers[0];

    for &num in numbers.iter().skip(1) {
        let mut value = num;

        while value < current {
            value += 1;
            moves += 1;
        }

        current = value;
    }

    moves
}