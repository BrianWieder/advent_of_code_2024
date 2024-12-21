use std::{env, io};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

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
        4 => day4::day4(&f),
        5 => day5::day5(&f),
        6 => day6::day6(&f),
        7 => day7::day7(&f),
        8 => day8::day8(&f),
        9 => day9::day9(&f),
        10 => day10::day10(&f),
        11 => day11::day11(&f),
        12 => day12::day12(&f),
        13 => day13::day13(&f),
        14 => day14::day14(&f),
        _ => println!("Day not implemented"),
    }
}
