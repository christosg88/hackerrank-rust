// https://www.hackerrank.com/challenges/mini-max-sum

use std::io::{self, BufRead};

fn mini_max_sum(arr: &[i32]) -> (i64, i64) {
    let mut min_val = arr[0];
    let mut max_val = arr[0];
    let mut total: i64 = 0;

    for &val in arr.iter() {
        if val < min_val {
            min_val = val;
        } else if val > max_val {
            max_val = val;
        }
        total += val as i64;
    }
    (total - max_val as i64, total - min_val as i64)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let (min, max) = mini_max_sum(&arr);
    println!("{} {}", min, max);
}

#[test]
fn test() {
    assert_eq!(mini_max_sum(&[1, 2, 3, 4, 5]), (10, 14));
    assert_eq!(mini_max_sum(&[7, 69, 2, 221, 8974]), (299, 9271));
    assert_eq!(
        mini_max_sum(&[256741038, 623958417, 467905213, 714532089, 938071625]),
        (2063136757, 2744467344)
    );
}
