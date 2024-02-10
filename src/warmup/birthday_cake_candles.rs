// https://www.hackerrank.com/challenges/birthday-cake-candles

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let mut max_val: i32 = candles[0];
    let mut count_max: i32 = 0;

    for candle in candles {
        if *candle > max_val {
            max_val = *candle;
            count_max = 1;
        } else if *candle == max_val {
            count_max += 1;
        }
    }

    count_max
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let candles: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthday_cake_candles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}


#[test]
fn test() {
    assert_eq!(birthday_cake_candles(&[3, 2, 1, 3]), 2);
}
