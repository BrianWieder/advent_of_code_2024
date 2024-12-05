use std::{
    collections::{HashMap, HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}
};

fn is_valid(pages: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut visited: HashSet<i32> = HashSet::new();
    for page in pages.iter().rev() {
        visited.insert(*page);
        if !rules.contains_key(page) {
            continue;
        }
        let required = rules.get(page).unwrap();
        for required_page in required.iter() {
            if visited.contains(required_page) {
                return false;
            }
        }
    }
    return true;
}

fn make_valid(pages: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut pages_updated: HashSet<i32> = HashSet::new();
    let mut pages_to_update: HashSet<i32> = HashSet::new();
    let mut correctly_ordered: Vec<i32> = Vec::new();
    let mut pages_to_insert = VecDeque::with_capacity(pages.len());
    for page in pages {
        pages_to_insert.push_back(page);
        pages_to_update.insert(*page);
    }

    while pages_to_insert.len() > 0 {
        let page = pages_to_insert.pop_front().unwrap();
        // If we've already updated this page, skip it
        if pages_updated.contains(page) {
            continue;
        }
        // If this page doesn't have any requirements, add it to the correctly ordered list
        if !rules.contains_key(page) {
            correctly_ordered.push(*page);
            pages_updated.insert(*page);
            continue;
        }
        let required = rules.get(page).unwrap();
        let mut requirements_met = true;
        // If any of the required pages haven't been updated, add them to the pages to insert before we add back the current page
        for page_required in required.iter() {
            if pages_updated.contains(page_required) || !pages_to_update.contains(page_required) {
                continue;
            }
            requirements_met = false;
            pages_to_insert.push_back(page_required);
        }
        // If all requirements are met, add the page to the correctly ordered list, else add it back to the pages to insert
        if requirements_met {
            correctly_ordered.push(*page);
            pages_updated.insert(*page);
        } else {
            pages_to_insert.push_back(page);
        }
    }
    
    return correctly_ordered;
}

pub fn day5(file: &str) {
    let file = File::open(file).expect(format!("Failed to open file: {}", file).as_str());
    let reader = BufReader::new(file);
    let mut ordering_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut processing_rules = true;
    let mut part1_answer = 0;
    let mut part2_answer = 0;
    for line in reader.lines() {
        let line_full = line.expect("Failed to read line.");
        let line = line_full.trim();
        if processing_rules {
            // Empty line between rules and updates
            if line.len() == 0 {
                processing_rules = false;
                continue;
            }
            let mut update_rule = line.split("|");
            let before: i32 = update_rule
                .next()
                .expect("Failed to parse rule line")
                .parse()
                .expect("Failed to conver to i32");
            let after: i32 = update_rule
                .next()
                .expect("Failed to parse rule line")
                .parse()
                .expect("Failed to convert to i32");
            ordering_rules.entry(after).or_insert_with(HashSet::new).insert(before);
        } else {
            // If we aren't processing rules, we are processing updates
            let pages: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
            if is_valid(&pages, &ordering_rules) {
                assert!(pages.len() % 2 == 1);
                let middle = pages.len() / 2;
                part1_answer += pages[middle];
            } else {
                let valid_pages = make_valid(&pages, &ordering_rules);
                assert!(valid_pages.len() % 2 == 1);
                let middle = valid_pages.len() / 2;
                part2_answer += valid_pages[middle];
            }
        }
    }
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}
