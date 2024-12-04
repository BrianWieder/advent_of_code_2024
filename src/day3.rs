use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use regex::Regex;

const MUL_REGEX_STR: &str  = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";
const DO_DONT_REGEX_STR: &str = ""

pub fn day3(file: &str) {
    let file = File::open(file).expect(format!("Failed to open file: {}", file).as_str());
    let reader = BufReader::new(file);
    let re = Regex::new(MUL_REGEX_STR).unwrap();
    

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut enabled = true;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
re.find_iter(haystack)
        for (_, [num1_str, num2_str]) in re.captures_iter(&line).map(|c| c.extract()) {
            let num1: i32 = num1_str.parse().expect("Failed to parse number");
            let num2: i32 = num2_str.parse().expect("Failed to parse number");
            part1_sum += num1 * num2;
        }
    }
    println!("Part 1: {}", part1_sum);
}
