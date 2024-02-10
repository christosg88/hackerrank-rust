// https://www.hackerrank.com/challenges/maximum-perimeter-triangle
//
// From: https://en.wikipedia.org/wiki/Triangle
//
// The triangle inequality states that the sum of the lengths of any two sides of a triangle must
// be greater than or equal to the length of the third side. That sum can equal the length of the
// third side only in the case of a degenerate triangle, one with collinear vertices. It is not
// possible for that sum to be less than the length of the third side. A triangle with three given
// positive side lengths exists if and only if those side lengths satisfy the triangle inequality.

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn is_valid_triangle(length1: i32, length2: i32, length3: i32) -> bool {
    length1 + length2 > length3 && length1 + length3 > length2 && length2 + length3 > length1
}

fn maximum_perimeter_triangle(sticks: &[i32]) -> Vec<i32> {
    let mut sticks = sticks.to_vec();
    sticks.sort_unstable_by(|a, b| b.cmp(a));

    let len = sticks.len();
    for idx1 in 0..len - 2 {
        for idx2 in idx1 + 1..len - 1 {
            for idx3 in idx2 + 1..len {
                if is_valid_triangle(sticks[idx1], sticks[idx2], sticks[idx3]) {
                    return vec![sticks[idx3], sticks[idx2], sticks[idx1]];
                }
            }
        }
    }
    vec![-1]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let sticks: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = maximum_perimeter_triangle(&sticks);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}

#[test]
fn test() {
    assert_eq!(maximum_perimeter_triangle(&[1, 1, 1, 3, 3]), &[1, 3, 3]);
    assert_eq!(maximum_perimeter_triangle(&[1, 2, 3]), &[-1]);
    assert_eq!(maximum_perimeter_triangle(&[1, 1, 1, 2, 3, 5]), &[1, 1, 1]);
}
