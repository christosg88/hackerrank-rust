use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

// https://www.hackerrank.com/challenges/game-of-stones-1

fn game_of_stones(n: i32) -> String {
    // in case the First player cannot make a single move, the Second player
    // wins
    if n < 2 {
        return "Second".to_string();
    }

    for choice in [2, 3, 5] {
        let rem = n - choice;
        if rem < 2 || (rem % 2 != 0 && rem % 3 != 0 && rem % 5 != 0) {
            return "First".to_string();
        }
    }

    "Second".to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let result = game_of_stones(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}

#[test]
fn test() {
    assert_eq!(game_of_stones(1), "Second");
    assert_eq!(game_of_stones(2), "First");
    assert_eq!(game_of_stones(3), "First");
    assert_eq!(game_of_stones(4), "First");
    assert_eq!(game_of_stones(5), "First");
    assert_eq!(game_of_stones(6), "First");
    assert_eq!(game_of_stones(7), "Second");
    assert_eq!(game_of_stones(8), "Second");
    assert_eq!(game_of_stones(9), "First");
    assert_eq!(game_of_stones(10), "First");
    assert_eq!(game_of_stones(11), "First");
    assert_eq!(game_of_stones(12), "First");
    assert_eq!(game_of_stones(13), "First");
    assert_eq!(game_of_stones(14), "Second");
    assert_eq!(game_of_stones(15), "Second");
    assert_eq!(game_of_stones(16), "First");
    assert_eq!(game_of_stones(17), "First");
    assert_eq!(game_of_stones(18), "First");
    assert_eq!(game_of_stones(19), "First");
    assert_eq!(game_of_stones(20), "First");
    assert_eq!(game_of_stones(21), "Second");
    assert_eq!(game_of_stones(22), "Second");
    assert_eq!(game_of_stones(23), "First");
    assert_eq!(game_of_stones(24), "First");
    assert_eq!(game_of_stones(25), "First");
    assert_eq!(game_of_stones(26), "First");
    assert_eq!(game_of_stones(27), "First");
    assert_eq!(game_of_stones(28), "Second");
    assert_eq!(game_of_stones(29), "Second");
    assert_eq!(game_of_stones(30), "First");
    assert_eq!(game_of_stones(31), "First");
    assert_eq!(game_of_stones(32), "First");
    assert_eq!(game_of_stones(33), "First");
    assert_eq!(game_of_stones(34), "First");
    assert_eq!(game_of_stones(35), "Second");
    assert_eq!(game_of_stones(36), "Second");
    assert_eq!(game_of_stones(37), "First");
    assert_eq!(game_of_stones(38), "First");
    assert_eq!(game_of_stones(39), "First");
    assert_eq!(game_of_stones(40), "First");
    assert_eq!(game_of_stones(41), "First");
    assert_eq!(game_of_stones(42), "Second");
    assert_eq!(game_of_stones(43), "Second");
    assert_eq!(game_of_stones(44), "First");
    assert_eq!(game_of_stones(45), "First");
    assert_eq!(game_of_stones(46), "First");
    assert_eq!(game_of_stones(47), "First");
    assert_eq!(game_of_stones(48), "First");
    assert_eq!(game_of_stones(49), "Second");
    assert_eq!(game_of_stones(50), "Second");
    assert_eq!(game_of_stones(51), "First");
    assert_eq!(game_of_stones(52), "First");
    assert_eq!(game_of_stones(53), "First");
    assert_eq!(game_of_stones(54), "First");
    assert_eq!(game_of_stones(55), "First");
    assert_eq!(game_of_stones(56), "Second");
    assert_eq!(game_of_stones(57), "Second");
    assert_eq!(game_of_stones(58), "First");
    assert_eq!(game_of_stones(59), "First");
    assert_eq!(game_of_stones(60), "First");
    assert_eq!(game_of_stones(61), "First");
    assert_eq!(game_of_stones(62), "First");
    assert_eq!(game_of_stones(63), "Second");
    assert_eq!(game_of_stones(64), "Second");
    assert_eq!(game_of_stones(65), "First");
    assert_eq!(game_of_stones(66), "First");
    assert_eq!(game_of_stones(67), "First");
    assert_eq!(game_of_stones(68), "First");
    assert_eq!(game_of_stones(69), "First");
    assert_eq!(game_of_stones(70), "Second");
    assert_eq!(game_of_stones(71), "Second");
    assert_eq!(game_of_stones(72), "First");
    assert_eq!(game_of_stones(73), "First");
    assert_eq!(game_of_stones(74), "First");
    assert_eq!(game_of_stones(75), "First");
    assert_eq!(game_of_stones(76), "First");
    assert_eq!(game_of_stones(77), "Second");
    assert_eq!(game_of_stones(78), "Second");
    assert_eq!(game_of_stones(79), "First");
    assert_eq!(game_of_stones(80), "First");
    assert_eq!(game_of_stones(81), "First");
    assert_eq!(game_of_stones(82), "First");
    assert_eq!(game_of_stones(83), "First");
    assert_eq!(game_of_stones(84), "Second");
    assert_eq!(game_of_stones(85), "Second");
    assert_eq!(game_of_stones(86), "First");
    assert_eq!(game_of_stones(87), "First");
    assert_eq!(game_of_stones(88), "First");
    assert_eq!(game_of_stones(89), "First");
    assert_eq!(game_of_stones(90), "First");
    assert_eq!(game_of_stones(91), "Second");
    assert_eq!(game_of_stones(92), "Second");
    assert_eq!(game_of_stones(93), "First");
    assert_eq!(game_of_stones(94), "First");
    assert_eq!(game_of_stones(95), "First");
    assert_eq!(game_of_stones(96), "First");
    assert_eq!(game_of_stones(97), "First");
    assert_eq!(game_of_stones(98), "Second");
    assert_eq!(game_of_stones(99), "Second");
    assert_eq!(game_of_stones(100), "First");
}
