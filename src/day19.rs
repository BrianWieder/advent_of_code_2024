use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn get_ways_to_build(pattern: &str, patterns: &HashSet<&str>, cache: &mut HashMap<String, i64>) -> i64 {
    if cache.contains_key(pattern) {
        return *cache.get(pattern).unwrap();
    }
    if pattern.len() == 0 {
        return 1;
    }
    let mut ways = 0;
    for i in 1..pattern.len()+1 {
        if patterns.contains(&pattern[..i]) {
            ways += get_ways_to_build(&pattern[i..], patterns, cache);
        }
    }
    cache.insert(pattern.to_owned(), ways);
    return ways;
}

pub fn day19(file: &str) {
    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let possible_patterns_str = lines.next().unwrap().unwrap();
    let patterns: HashSet<&str> = HashSet::from_iter(
        possible_patterns_str
            .split(", ")
            .clone()
            .collect::<Vec<&str>>(),
    );
    lines.next(); // skip empty line

    let mut part1 = 0;
    let mut part2 = 0;

    let mut cache = HashMap::new();

    for line in lines {
        let pattern = line.unwrap();
        let ways_to_build = get_ways_to_build(&pattern, &patterns, &mut cache);
        if ways_to_build > 0 {
            part1 += 1;
        }
        part2 += ways_to_build;
    }


    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
