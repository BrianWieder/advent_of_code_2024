use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader}, usize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    point: Point,
    d: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    pos: Position,
    cost: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).then(self.pos.cmp(&other.pos))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Start,
    End,
}

fn get_point_forward(point: &Point, direction: &Direction) -> Point {
    match direction {
        Direction::Up => Point {
            x: point.x - 1,
            y: point.y,
        },
        Direction::Down => Point {
            x: point.x + 1,
            y: point.y,
        },
        Direction::Left => Point {
            x: point.x,
            y: point.y - 1,
        },
        Direction::Right => Point {
            x: point.x,
            y: point.y + 1,
        },
    }
}

fn get_possible_direction_changes(direction: &Direction) -> Vec<Direction> {
    match direction {
        Direction::Up => vec![Direction::Left, Direction::Right],
        Direction::Down => vec![Direction::Left, Direction::Right],
        Direction::Left => vec![Direction::Up, Direction::Down],
        Direction::Right => vec![Direction::Up, Direction::Down],
    }
}

fn explore_backwards(parents: &HashMap<Position, Vec<Position>>, ends: &Vec<Position>) -> usize {
    let mut to_visit = ends.clone();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut points = HashSet::new();
    while !to_visit.is_empty() {
        let pos = to_visit.pop().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        points.insert(pos.point);
        if let Some(parents) = parents.get(&pos) {
            for parent in parents {
                to_visit.push(*parent);
            }
        }
    }
    return points.len();
}

fn explore(grid: &Vec<Vec<Tile>>, start_pos: State) -> (usize, usize) {
    let mut to_visit = BinaryHeap::new();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut parents: HashMap<Position, Vec<Position>> = HashMap::new();
    let mut costs: HashMap<Position, usize> = HashMap::new();
    to_visit.push(Reverse(start_pos));
    let mut end_goal = usize::MAX;
    let mut ends = Vec::new();
    while !to_visit.is_empty() {
        let Reverse(state) = to_visit.pop().unwrap();
        if visited.contains(&state.pos) {
            continue;
        }
        if state.cost > end_goal {
            return (end_goal, explore_backwards(&parents, &ends));
        }
        if grid[state.pos.point.x][state.pos.point.y] == Tile::End {
            end_goal = state.cost;
            ends.push(state.pos);
        }
        visited.insert(state.pos);
        let point_forward = get_point_forward(&state.pos.point, &state.pos.d);
        let possible_directions = get_possible_direction_changes(&state.pos.d);
        let possible_moves = vec![
            State {
                cost: state.cost + 1,
                pos: Position {
                    point: point_forward,
                    d: state.pos.d,
                },
            },
            State {
                cost: state.cost + 1000,
                pos: Position {
                    point: state.pos.point,
                    d: possible_directions[0],
                },
            },
            State {
                cost: state.cost + 1000,
                pos: Position {
                    point: state.pos.point,
                    d: possible_directions[1],
                },
            },
        ];
        for possible_move in possible_moves {
            if grid[possible_move.pos.point.x][possible_move.pos.point.y] == Tile::Wall {
                continue;
            }
            let curr_cost = costs.entry(possible_move.pos).or_insert(usize::MAX);
            if *curr_cost >= possible_move.cost {
                to_visit.push(Reverse(possible_move));
                if *costs.entry(possible_move.pos).or_insert(possible_move.cost)
                    > possible_move.cost
                {
                    parents.remove(&possible_move.pos);
                }
                costs.insert(possible_move.pos, possible_move.cost);
                parents
                    .entry(possible_move.pos)
                    .or_insert_with(Vec::new)
                    .push(state.pos);
            }
        }
    }
    return (0, 0);
}

pub fn day16(file: &str) {
    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut grid = Vec::new();
    let mut start = State {
        pos: Position {
            point: Point { x: 0, y: 0 },
            d: Direction::Right,
        },
        cost: 0,
    };
    for line in reader.lines() {
        let line = line.unwrap();
        let row: Vec<Tile> = line
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'S' => {
                    start.pos.point.x = grid.len();
                    start.pos.point.y = i;
                    Tile::Start
                }
                'E' => Tile::End,
                _ => panic!("Invalid character in input"),
            })
            .collect();
        grid.push(row);
    }
    let (part1, part2) = explore(&grid, start);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
