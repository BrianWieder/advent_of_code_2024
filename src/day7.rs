use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn is_valid(test_values: &[i64], target: i64) -> bool {
    if test_values.len() == 0 {
        return target == 0;
    }
    if test_values.len() == 1 {
        return test_values[0] == target;
    }
    let left = &test_values[..test_values.len() - 1];
    let curr = test_values[test_values.len() - 1];
    if target % curr == 0 && is_valid(left, target / curr) {
        return true;
    }
    return is_valid(left, target - curr);
}

pub fn is_valid_with_concat(test_values: &[i64], target: i64) -> bool {
    if test_values.len() == 0 {
        return target == 0;
    }
    if test_values.len() == 1 {
        return test_values[0] == target;
    }
    let left = &test_values[..test_values.len() - 1];
    let curr = test_values[test_values.len() - 1];
    if target % curr == 0 && is_valid_with_concat(left, target / curr) {
        return true;
    }
    let curr_str = curr.to_string();
    let target_str = target.to_string();

    if target_str.len() > curr_str.len()
        && target_str.ends_with(&curr_str)
        && is_valid_with_concat(
            left,
            target_str
                .strip_suffix(&curr_str)
                .unwrap()
                .parse()
                .expect("Failed to parse target back into i64"),
        )
    {
        return true;
    }
    return is_valid_with_concat(left, target - curr);
}

pub fn day7(file: &str) {
    let file = File::open(file).expect("file not found");
    let reader = BufReader::new(file);

    let mut part1_total = 0;
    let mut part2_total = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut split = line.split(": ");
        let target: i64 = split
            .next()
            .expect("Failed to get target")
            .parse()
            .expect("Failed to parse target into i32");
        let values: Vec<i64> = split
            .next()
            .expect("Failed to get values")
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to convert to in32"))
            .collect();
        if is_valid(&values, target) {
            part1_total += target;
            part2_total += target;
        } else if is_valid_with_concat(&values, target) {
            part2_total += target;
        }
    }

    println!("Part 1: {}", part1_total);
    println!("Part 2: {}", part2_total);
}
