use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Loc {
    Empty,
    Visited,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct VisitedLocation {
    pos: (i32, i32),
    direction: Direction,
}

fn incremented_direction(pos: (i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
    }
}

fn is_in_bounds(map: &Vec<Vec<Loc>>, pos: (i32, i32)) -> bool {
    pos.0 >= 0
        && pos.1 >= 0
        && pos.0 < map.len().try_into().expect("failed to parse into i32")
        && pos.1
            < map[pos.0 as usize]
                .len()
                .try_into()
                .expect("failed to parse into i32")
}

fn would_loop(map: &mut Vec<Vec<Loc>>, pos: (i32, i32), dir: &Direction) -> bool {
    let mut pos = pos;
    let mut direction = dir.clone();
    let mut visited = HashSet::new();
    let box_loc = incremented_direction(pos, &dir);
    let prev = map[box_loc.0 as usize][box_loc.1 as usize];
    assert!(prev == Loc::Empty || prev == Loc::Visited);
    map[box_loc.0 as usize][box_loc.1 as usize] = Loc::Wall;
    visited.insert(VisitedLocation {
        pos: pos,
        direction: dir.clone(),
    });
    loop {
        loop {
            let new_pos = incremented_direction(pos, &direction);
            if !is_in_bounds(map, new_pos) {
                map[box_loc.0 as usize][box_loc.1 as usize] = prev;
                return false;
            }
            let new_x = new_pos.0 as usize;
            let new_y = new_pos.1 as usize;
            if map[new_x][new_y] != Loc::Wall {
                pos = new_pos;
                let curr_location = VisitedLocation {
                    pos: pos,
                    direction: direction.clone(),
                };
                if visited.contains(&curr_location) {
                    map[box_loc.0 as usize][box_loc.1 as usize] = prev;
                    return true;
                }
                visited.insert(curr_location);
                break;
            }
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        }
    }
}

fn navigate_until_oob(map: &mut Vec<Vec<Loc>>, pos: (i32, i32)) -> (i32, i32) {
    let mut pos = pos;
    let mut direction = Direction::Up;
    let mut num_visited = 0;
    let mut num_loops = 0;
    loop {
        let x: i32 = pos.0;
        let y: i32 = pos.1;
        if map[x as usize][y as usize] != Loc::Visited {
            num_visited += 1;
        }
        map[x as usize][y as usize] = Loc::Visited;
        loop {
            let new_pos = incremented_direction(pos, &direction);
            if !is_in_bounds(map, new_pos) {
                return (num_visited, num_loops);
            }
            let new_x = new_pos.0 as usize;
            let new_y = new_pos.1 as usize;
            if map[new_x][new_y] != Loc::Wall {
                if map[new_x][new_y] != Loc::Visited {
                    if would_loop(map, pos, &direction) {
                        num_loops += 1;
                    }
                }
                pos = new_pos;
                break;
            }
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        }
    }
}

pub fn day6(file: &str) {
    let file = File::open(file).expect(format!("Failed to open file {}", file).as_str());
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<Loc>> = Vec::new();
    let mut pos: (i32, i32) = (0, 0);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut row: Vec<Loc> = Vec::with_capacity(line.chars().count());
        for c in line.chars() {
            match c {
                '.' => row.push(Loc::Empty),
                '#' => row.push(Loc::Wall),
                '^' => {
                    pos = (
                        map.len().try_into().expect("failed to parse into i32"),
                        row.len().try_into().expect("failed to parse into i32"),
                    );
                    row.push(Loc::Empty);
                }
                _ => panic!("Invalid character in input"),
            }
        }
        map.push(row);
    }

    let (num_visited, num_loops) = navigate_until_oob(&mut map, pos);

    println!("Part 1: {}", num_visited);
    println!("Part 2: {}", num_loops);
}
