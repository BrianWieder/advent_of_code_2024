use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader}, vec,
};

// Real grid size
const GRID_SIZE: usize = 71;
// Example grid size
// const GRID_SIZE: usize = 7;

const SECONDS: usize = 1024;
// const SECONDS: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct State {
    point: Point,
    dist: usize,
}

fn collect_parents(parents: &HashMap<Point, Point>, point: &Point) -> Vec<Point> {
    let mut path = vec![];
    let mut current = point;
    while let Some(parent) = parents.get(current) {
        path.push(*current);
        current = parent;
    }
    path.push(*current);
    return path
}

fn explore(grid: &Vec<Vec<bool>>) -> Vec<Point> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; GRID_SIZE]; GRID_SIZE];
    let mut parents: HashMap<Point, Point> = HashMap::new();
    queue.push_back(State {
        point: Point { x: 0, y: 0 },
        dist: 0,
    });
    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        let point = state.point;
        // Check if we reached the goal
        if point.x == GRID_SIZE - 1 && point.y == GRID_SIZE - 1 {
            return collect_parents(&parents, &point);
        }
        if visited[point.x][point.y] {
            continue;
        }
        visited[point.x][point.y] = true;
        let mut possible_moves = vec![
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
        ];
        if point.x > 0 {
            possible_moves.push(Point {
                x: point.x - 1,
                y: point.y,
            });
        }
        if point.y > 0 {
            possible_moves.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }
        for m in possible_moves {
            if m.x < GRID_SIZE && m.y < GRID_SIZE && !grid[m.x][m.y] && !visited[m.x][m.y] {
                if !parents.contains_key(&m) {
                    parents.insert(m, point);
                }
                queue.push_back(State {
                    point: m,
                    dist: state.dist + 1,
                });
            }
        }
    }
    return vec![];
}

pub fn day18(file: &str) {
    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut grid = vec![vec![false; GRID_SIZE]; GRID_SIZE];
    let mut lines = reader.lines();

    for _ in 0..SECONDS {
        let line = lines
            .next()
            .expect("Unable to read line")
            .expect("Unable to read line");
        let coords: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
        let y = coords[0];
        let x = coords[1];
        grid[x][y] = true;
    }

    let mut path = explore(&grid);

    println!("Part 1: {}", path.len() - 1);
    let mut last_point: Option<Point> = None;

    while path.len() > 0 {
        let points: HashSet<Point> = HashSet::from_iter(path.iter().cloned());
        while let Some(line) = lines.next() {
            let line = line.expect("Unable to read line");
            let coords: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
            let point = Point {
                x: coords[1],
                y: coords[0],
            };
            grid[point.x][point.y] = true;
            last_point = Some(point);
            if points.contains(&point) {
                break;
            }
        }
        path = explore(&grid);
    }

    let last_point = last_point.unwrap();
    println!("Part 2: {},{}", last_point.y, last_point.x);
}
