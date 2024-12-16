use std::{
    collections::HashMap,
    fs::{self},
};

#[derive(Hash, Eq, PartialEq)]
struct Rock {
    value: u64,
    depth: i32,
}

fn get_num_after_splits(rock: u64, splits: i32, cache: &mut HashMap<Rock, u64>) -> u64 {
    if splits == 0 {
        return 1;
    }
    let goal = Rock {
        value: rock,
        depth: splits,
    };
    if cache.contains_key(&goal) {
        return *cache.get(&goal).unwrap();
    }
    if rock == 0 {
        let result = get_num_after_splits(1, splits - 1, cache);
        cache.insert(goal, result);
        return result;
    }
    let rock_str = rock.to_string();
    if rock_str.len() % 2 == 0 {
        let half = rock_str.len() / 2;
        let first_half = rock_str[..half].parse::<u64>().unwrap();
        let second_half = rock_str[half..].parse::<u64>().unwrap();
        let result = get_num_after_splits(first_half, splits - 1, cache)
            + get_num_after_splits(second_half, splits - 1, cache);
        cache.insert(goal, result);
        return result;
    } else {
        let result = get_num_after_splits(rock * 2024, splits - 1, cache);
        cache.insert(goal, result);
        return result;
    }
}

pub fn day11(file: &str) {
    let content = fs::read_to_string(file).expect("Failed to read file");
    let rocks: Vec<u64> = content
        .split_whitespace()
        .map(|x| x.parse().expect("Unable to parse to u64"))
        .collect();
    let mut cache = HashMap::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for rock in rocks {
        part2 += get_num_after_splits(rock, 75, &mut cache);
        part1 += get_num_after_splits(rock, 25, &mut cache);
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
