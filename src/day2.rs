use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn levels_needed_to_delete_to_be_safe(levels: &Vec<i32>) -> u32 {
    let mut num_increasing = 0;
    let mut num_decreasing = 0;
    let mut last = levels[0];
    for num in levels.iter().skip(1) {
        if last < *num {
            num_increasing += 1;
        } else if last > *num {
            num_decreasing += 1;
        }
        last = *num;
    }
    let increasing = num_increasing > num_decreasing;
    let mut max_nums = 1;
    let mut lengths: Vec<u32> = Vec::new();
    lengths.push(1);
    for i in 1..levels.len() {
        let mut length = 1;
        let num = levels[i];
        for j in (0..i).rev() {
            let last_num = levels[j];
            let last_length = lengths[j];
            if increasing {
                if num > last_num && num - last_num <= 3 {
                    length = cmp::max(length, last_length + 1);
                }
            } else {
                if num < last_num && last_num - num <= 3 {
                    length = cmp::max(length, last_length + 1);
                }
            }
        }
        lengths.push(length);
        max_nums = cmp::max(max_nums, length);
    }
    return levels.len() as u32 - max_nums;
}

pub fn day2(file: &str) {
    let file = File::open(file).expect(format!("Failed to open file: {}", file).as_str());
    let reader = BufReader::new(file);

    let mut num_safe = 0;
    let mut num_almost_safe = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let levels_str = line.trim().split_whitespace();
        let levels: Vec<i32> = levels_str
            .map(|x| x.parse().expect("Failed to parse number"))
            .collect();
        let violations = levels_needed_to_delete_to_be_safe(&levels);
        if violations == 0 {
            num_safe += 1;
        }
        if violations <= 1 {
            num_almost_safe += 1;
        }
    }
    println!("Part 1: {}", num_safe);
    println!("Part 2: {}", num_almost_safe);
}
