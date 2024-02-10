// https://www.hackerrank.com/challenges/circular-array-rotation

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn circular_array_rotation(arr: &[i32], num_rotations: i32, queries: &[i32]) -> Vec<i32> {
    let num_rotations : usize = (num_rotations as usize) % arr.len();

    let wrapped_queries: Vec<usize> = queries
        .iter()
        .map(|query| ((*query as usize + arr.len() - num_rotations) % arr.len()))
        .collect();

    let mut ret = Vec::with_capacity(queries.len());
    for wrapped_query in wrapped_queries {
        ret.push(arr[wrapped_query]);
    }

    ret
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let q = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let mut queries: Vec<i32> = Vec::with_capacity(q as usize);

    for _ in 0..q {
        let queries_item = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        queries.push(queries_item);
    }

    let result = circular_array_rotation(&a, k, &queries);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}

#[test]
fn test() {
    assert_eq!(circular_array_rotation(&[1, 2, 3], 2, &[0, 1, 2]), &[2, 3, 1]);
}
