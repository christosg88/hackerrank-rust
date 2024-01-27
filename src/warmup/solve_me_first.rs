use std::io;

// https://www.hackerrank.com/challenges/solve-me-first

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // variable declaration
    let mut num1 = String::new();
    let mut num2 = String::new();

    // read variables
    io::stdin().read_line(&mut num1)?;
    io::stdin().read_line(&mut num2)?;

    // parse integers
    let num1: i32 = num1.trim().parse()?;
    let num2: i32 = num2.trim().parse()?;

    // print the sum
    println!("{}", add(num1, num2));
    Ok(())
}

#[test]
fn solve_me_first() {
    assert_eq!(add(2, 3), 5);
}
