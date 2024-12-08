use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn mul(&self, rhs: i32) -> Point {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }

    fn add(&self, rhs: &Point) -> Point {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn is_in_bounds(point: &Point, map_width: i32, map_height: i32) -> bool {
    return point.x >= 0 && point.x < map_height && point.y >= 0 && point.y < map_width;
}

fn get_part1_antinodes(
    nodes: &HashMap<char, Vec<Point>>,
    map_width: i32,
    map_height: i32,
) -> Vec<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (_, points) in nodes.iter() {
        for i in 0..points.len() - 1 {
            for x in i + 1..points.len() {
                let p1 = &points[i];
                let p2 = &points[x];
                let possible_pos = vec![
                    Point {
                        x: (2 * p1.x - p2.x),
                        y: (2 * p1.y - p2.y),
                    },
                    Point {
                        x: (2 * p2.x - p1.x),
                        y: (2 * p2.y - p1.y),
                    },
                ];
                for possible in possible_pos {
                    if is_in_bounds(&possible, map_width, map_height) {
                        antinodes.insert(possible);
                    }
                }
            }
        }
    }
    return Vec::from_iter(antinodes);
}

fn get_part2_antinodes(
    nodes: &HashMap<char, Vec<Point>>,
    map_width: i32,
    map_height: i32,
) -> Vec<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (_, points) in nodes.iter() {
        for i in 0..points.len() - 1 {
            for x in i + 1..points.len() {
                let p1 = &points[i];
                let p2 = &points[x];
                let diff = Point {
                    x: p2.x - p1.x,
                    y: p2.y - p1.y,
                };
                let mut mul = 0;
                loop {
                    let possible = diff.mul(mul).add(p1);
                    if is_in_bounds(&possible, map_width, map_height) {
                        antinodes.insert(possible);
                    } else {
                        break;
                    }
                    mul += 1;
                }
                mul = -1;
                loop {
                    let possible = diff.mul(mul).add(p1);
                    if is_in_bounds(&possible, map_width, map_height) {
                        antinodes.insert(possible);
                    } else {
                        break;
                    }
                    mul -= 1;
                }
            }
        }
    }
    return Vec::from_iter(antinodes);
}

pub fn day8(file: &str) {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut nodes: HashMap<char, Vec<Point>> = HashMap::new();

    let mut line_num = 0;
    let mut char_num = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        char_num = 0;
        for char in line.chars() {
            if char != '.' {
                let point = Point {
                    x: line_num,
                    y: char_num,
                };
                nodes.entry(char).or_insert_with(Vec::new).push(point);
            }
            char_num += 1;
        }
        line_num += 1;
    }
    println!(
        "Part 1: {}",
        get_part1_antinodes(&nodes, char_num, line_num).len()
    );
    println!("Part 2: {}", get_part2_antinodes(&nodes, char_num, line_num).len());
}
