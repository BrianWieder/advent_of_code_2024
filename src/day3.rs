use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use regex::Regex;

const MUL_REGEX_STR: &str  = r"(mul\((?<num1>[0-9]{1,3}),(?<num2>[0-9]{1,3})\)|do\(\)|don\'t\(\))";

pub fn day3(file: &str) {
    let file = File::open(file).expect(format!("Failed to open file: {}", file).as_str());
    let reader = BufReader::new(file);
    let re = Regex::new(MUL_REGEX_STR).unwrap();
    

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut enabled = true;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        for caps in re.captures_iter(&line) {
            if caps.name("num1").is_some() && caps.name("num2").is_some() {
                let num1: i32 = caps.name("num1").unwrap().as_str().parse().expect("Failed to parse number");
                let num2: i32 = caps.name("num2").unwrap().as_str().parse().expect("Failed to parse number");
                let mul = num1 * num2;
                part1_sum += mul;
                if enabled {
                    part2_sum += mul;
                }
            } else{
                let op  = caps.get(0).expect("Unable to get capture group").as_str();
                enabled = op.contains("do()");
            }
        }
    }
    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}
