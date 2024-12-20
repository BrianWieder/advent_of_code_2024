use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const BUTTON_REGEX: &str = r"Button (A|B): X\+(?<x_offset>[0-9]+), Y\+(?<y_offset>[0-9]+)";
const PRIZE_REGEX: &str = r"Prize: X=(?<x_offset>[0-9]+), Y=(?<y_offset>[0-9]+)";

fn get_offsets(line: &str, re: &Regex) -> (i64, i64) {
    let matches = re.captures(line).unwrap();
    let x_offset: i64 = matches
        .name("x_offset")
        .unwrap()
        .as_str()
        .parse()
        .expect("Failed to parse number");
    let y_offset: i64 = matches
        .name("y_offset")
        .unwrap()
        .as_str()
        .parse()
        .expect("Failed to parse number");
    (x_offset, y_offset)
}

pub fn day13(file: &str) {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().peekable();
    let mut part1 = 0;
    let mut part2 = 0;
    let button_re = Regex::new(BUTTON_REGEX).unwrap();
    let prize_re = Regex::new(PRIZE_REGEX).unwrap();

    while lines.peek().is_some() {
        let line_a = lines.next().unwrap().unwrap();
        let line_b = lines.next().unwrap().unwrap();
        let line_prize = lines.next().unwrap().unwrap();

        let (a_x_offset, a_y_offset) = get_offsets(&line_a, &button_re);
        let (b_x_offset, b_y_offset) = get_offsets(&line_b, &button_re);
        let (prize_x_offset, prize_y_offset) = get_offsets(&line_prize, &prize_re);

        let a_num = (prize_y_offset * b_x_offset) - (prize_x_offset * b_y_offset);
        let a_denom = (a_y_offset * b_x_offset) - (a_x_offset * b_y_offset);
        if a_num % a_denom == 0 {
            let a = a_num / a_denom;
            let b_num = prize_x_offset - (a_x_offset * a);
            let b_denom = b_x_offset;
            if b_num % b_denom == 0 {
                let b = b_num / b_denom;
                if a >= 0 && b >= 0 {
                    part1 += 3 * a + b;
                }
            }
        }

        let prize_x_offset = prize_x_offset + 10000000000000;
        let prize_y_offset = prize_y_offset + 10000000000000;

        let a_num = (prize_y_offset * b_x_offset) - (prize_x_offset * b_y_offset);
        let a_denom = (a_y_offset * b_x_offset) - (a_x_offset * b_y_offset);
        if a_num % a_denom == 0 {
            let a = a_num / a_denom;
            let b_num = prize_x_offset - (a_x_offset * a);
            let b_denom = b_x_offset;
            if b_num % b_denom == 0 {
                let b = b_num / b_denom;
                if a >= 0 && b >= 0 {
                    part2 += 3 * a + b;
                }
            }
        }

        // If there is another line, it should be empty, so we skip it
        if lines.peek().is_some() {
            lines.next();
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
