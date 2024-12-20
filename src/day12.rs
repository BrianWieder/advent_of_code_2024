use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Region {
    area: i32,
    perimeter: i32,
    sides: i32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Neighbor {
    point: Point,
    direction: Direction,
}

fn parse_to_int_map(s: String) -> Vec<Vec<u8>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_ascii_lowercase() as u8 - 'a'.to_ascii_lowercase() as u8)
                .collect()
        })
        .collect()
}

fn should_visit(map: &Vec<Vec<u8>>, point: &Point, target: u8) -> bool {
    let x = point.x;
    let y = point.y;
    if x < 0 || y < 0 || x >= map.len().try_into().unwrap() || y >= map[0].len().try_into().unwrap()
    {
        return false;
    }
    return map[x as usize][y as usize] == target;
}

fn get_neighbor(point: &Point, direction: Direction) -> Neighbor {
    let x = point.x;
    let y = point.y;
    match direction {
        Direction::Up => Neighbor {
            point: Point { x: x - 1, y },
            direction: Direction::Up,
        },
        Direction::Down => Neighbor {
            point: Point { x: x + 1, y },
            direction: Direction::Down,
        },
        Direction::Left => Neighbor {
            point: Point { x, y: y - 1 },
            direction: Direction::Left,
        },
        Direction::Right => Neighbor {
            point: Point { x, y: y + 1 },
            direction: Direction::Right,
        },
        Direction::None => Neighbor {
            point: Point { x, y },
            direction: Direction::None,
        },
    }
}

fn has_same_edge(map: &Vec<Vec<u8>>, point: &Point, direction: Direction, target: u8) -> bool {
    return should_visit(map, point, target)
        && !should_visit(map, &get_neighbor(point, direction).point, target);
}

fn explore(map: &Vec<Vec<u8>>, x: isize, y: isize, visited: &mut Vec<Vec<bool>>) -> Region {
    let mut area = 0;
    let mut perimeter = 0;
    let mut sides = 0;
    let mut sides_visited = HashSet::new();
    let mut queue: VecDeque<Neighbor> = VecDeque::new();
    queue.push_back(Neighbor {
        point: Point { x, y },
        direction: Direction::None,
    });

    while queue.len() > 0 {
        let curr = queue.pop_front().unwrap();
        let point = curr.point;
        let x = point.x as usize;
        let y = point.y as usize;
        if visited[x][y] {
            continue;
        }
        visited[x][y] = true;
        area += 1;

        let possible_points = vec![
            get_neighbor(&point, Direction::Up),
            get_neighbor(&point, Direction::Down),
            get_neighbor(&point, Direction::Right),
            get_neighbor(&point, Direction::Left),
        ];

        for possible in possible_points {
            if should_visit(map, &possible.point, map[x][y]) {
                queue.push_back(possible);
            } else {
                perimeter += 1;
                match possible.direction {
                    Direction::Up | Direction::Down => {
                        let mut left_neighbor = get_neighbor(&point, Direction::Left);
                        while should_visit(map, &left_neighbor.point, map[x][y])
                            && has_same_edge(
                                map,
                                &left_neighbor.point,
                                possible.direction,
                                map[x][y],
                            )
                        {
                            left_neighbor = get_neighbor(&left_neighbor.point, Direction::Left);
                        }
                        let possible_hit = Neighbor {
                            point: left_neighbor.point,
                            direction: possible.direction,
                        };
                        if !sides_visited.contains(&possible_hit) {
                            sides += 1;
                            sides_visited.insert(possible_hit);
                        }
                    }
                    Direction::Left | Direction::Right => {
                        let mut up_neighbor = get_neighbor(&point, Direction::Up);
                        while should_visit(map, &up_neighbor.point, map[x][y])
                            && has_same_edge(map, &up_neighbor.point, possible.direction, map[x][y])
                        {
                            up_neighbor = get_neighbor(&up_neighbor.point, Direction::Up);
                        }
                        let possible_hit = Neighbor {
                            point: up_neighbor.point,
                            direction: possible.direction,
                        };
                        if !sides_visited.contains(&possible_hit) {
                            sides += 1;
                            sides_visited.insert(possible_hit);
                        }
                    }
                    Direction::None => {}
                }
            }
        }
    }

    return Region {
        area,
        perimeter,
        sides,
    };
}

impl Region {
    fn part1_cost(&self) -> i32 {
        return self.area * self.perimeter;
    }

    fn part2_cost(&self) -> i32 {
        return self.area * self.sides;
    }
}

pub fn day12(file: &str) {
    let contents = std::fs::read_to_string(file).expect("Failed to read file");
    let mut regions: Vec<Region> = Vec::new();
    let map = parse_to_int_map(contents);
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut part1 = 0;
    let mut part2 = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if !visited[x][y] {
                let region = explore(&map, x as isize, y as isize, &mut visited);
                part1 += region.part1_cost();
                part2 += region.part2_cost();
                regions.push(region);
            }
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
