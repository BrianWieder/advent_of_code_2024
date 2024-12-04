use std::{env, io};

mod day1;
mod day2;
mod day3;

fn main() {
    println!("What day to run?");
    let mut day_str = String::new();
    io::stdin().read_line(&mut day_str).expect( "Failed to read line");
    let day: i32 = match day_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            return;
        }
    };
    println!("Running day {}", day);
    let f = env::args().nth(1).expect("Please provide a file name as an argument");
    match day {
        1 => day1::day1(&f),
        2 => day2::day2(&f),
        3 => day3::day3(&f),
        _ => println!("Day not implemented"),
    }
}
