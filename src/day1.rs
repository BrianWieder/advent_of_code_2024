use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day1(file: &str) {
    let file = File::open(file).expect(format!("Failed to open file: {}", file).as_str());
    let reader = BufReader::new(file);

    let mut list1: BinaryHeap<i32> = BinaryHeap::new();
    let mut list2: BinaryHeap<i32> = BinaryHeap::new();
    let mut list2_map: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut nums = line.trim().split_whitespace();
        list1.push(nums.next().expect("Failed to read number").parse().expect("Failed to parse number"));
        let second_num = nums.next().expect("Failed to read number").parse().expect("Failed to parse number");
        list2.push(second_num);
        let original = list2_map.get(&second_num).unwrap_or(&0);
        list2_map.insert(second_num, original + 1);
    }

    let mut part1_answer = 0;
    let mut part2_answer = 0;
    while !list1.is_empty() {
        let num1 = list1.pop().expect("Failed to pop number");
        let count = list2_map.get(&num1).unwrap_or(&0);
        part1_answer += (num1 - list2.pop().expect("Failed to pop number")).abs();
        part2_answer += num1 * count;
    }
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}