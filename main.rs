use std::io::{Write, stdout};
use std::time::Instant;

mod day01;
mod day02;

fn take_int() -> i64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    print!("Enter day to solve: ");
    let _ = stdout().flush();
    let day = take_int();

    print!("Enter part to solve: ");
    let _ = stdout().flush();
    let part = take_int();

    println!();
    let start = Instant::now();

    print!("Answer: ");
    match (day, part) {
        (1, 1) => println!("{}", day01::part_one()),
        (1, 2) => println!("{}", day01::part_two()),
        (2, 1) => println!("{}", day02::part_one()),
        (2, 2) => println!("{}", day02::part_two()),

        _ => panic!("sorry not implemented yet"),
    };

    let duration = start.elapsed();
    println!("Runtime: {:?}", duration);
}
