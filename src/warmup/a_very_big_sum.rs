use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn a_very_big_sum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ar_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let ar: Vec<i64> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    let result = a_very_big_sum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}

#[test]
fn test1() {
    assert_eq!(
        a_very_big_sum(&[1000000001, 1000000002, 1000000003, 1000000004, 1000000005]),
        5000000015
    )
}
