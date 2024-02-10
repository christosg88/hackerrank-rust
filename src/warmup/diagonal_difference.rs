use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;
    let num_lines = arr.len();

    for row in 0..num_lines {
        sum1 += arr[row][row];
        sum2 += arr[row][num_lines - row - 1];
    }

    (sum1 - sum2).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonal_difference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

#[test]
fn test() {
    assert_eq!(
        diagonal_difference(&vec![vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]]),
        15
    );
}
