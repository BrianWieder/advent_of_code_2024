use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct MapEntry {
    height: i32,
    visited: bool,
    reachable: Option<HashSet<Point>>,
    rating: i32,
}

fn can_and_should_visit(point: Point, prev_height: i32, map: &Vec<Vec<MapEntry>>) -> bool {
    if point.x < 0 || point.y < 0 || point.x >= map.len() as i32 || point.y >= map[0].len() as i32 {
        return false;
    }
    let entry = &map[point.x as usize][point.y as usize];
    if entry.height == prev_height + 1 {
        return true;
    }
    return false;
}

fn explore_point<'a>(
    map: &'a mut Vec<Vec<MapEntry>>,
    point: Point,
) -> (i32, Option<HashSet<Point>>, i32) {
    if point.x < 0 || point.y < 0 || point.x >= map.len() as i32 || point.y >= map[0].len() as i32 {
        return (-1, None, 0);
    }
    let entry = map[point.x as usize][point.y as usize].clone();
    // Only visit once
    if entry.visited {
        if entry.reachable.is_none() {
            return (-1, None, entry.rating);
        }
        return (
            entry.height,
            Some(entry.reachable.as_ref().unwrap().clone()),
            entry.rating
        );
    }
    map[point.x as usize][point.y as usize].visited = true;
    let mut reachable: HashSet<Point> = HashSet::new();
    let mut rating = 0;

    if entry.height == 9 {
        reachable.insert(point);
        rating = 1;
    } else {
        let options = vec![
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Point {
                x: point.x,
                y: point.y - 1,
            },
        ];
        for option in options {
            if can_and_should_visit(option, entry.height, map) {
                let (height, reachable_from_option, option_rating) = explore_point(map, option);
                if height == entry.height + 1 && reachable_from_option.is_some() {
                    reachable.extend(reachable_from_option.unwrap().iter());
                }
                rating += option_rating; 
            }
        }
    }
    map[point.x as usize][point.y as usize].visited = true;
    map[point.x as usize][point.y as usize].reachable = Some(reachable);
    map[point.x as usize][point.y as usize].rating = rating;
    return (
        entry.height,
        Some(
            map[point.x as usize][point.y as usize]
                .reachable
                .as_ref()
                .unwrap()
                .clone(),
        ),
        rating,
    );
}

pub fn day10(file: &str) {
    let file = File::open(file).expect(format!("File not found {}", file).as_str());
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<MapEntry>> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let mut row: Vec<MapEntry> = Vec::new();
        for c in line.chars() {
            let height = c.to_digit(10).expect("Failed to parse to digit") as i32;
            row.push(MapEntry {
                height,
                visited: false,
                reachable: None,
                rating: 0,
            });
        }
        map.push(row);
    }
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let point = Point {
                x: i as i32,
                y: j as i32,
            };
            explore_point(&mut map, point);
            if map[i][j].height == 0 {
                let reachable = match &map[i][j].reachable {
                    Some(reachable) => reachable.len(),
                    None => 0,
                };
                part1 += reachable;
                part2 += map[i][j].rating;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
