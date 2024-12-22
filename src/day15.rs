use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Box,
    Empty,
    Robot,
    LeftBox,
    RightBox,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn get_point_in_direction(p: &Point, d: &Direction) -> Point {
    match d {
        Direction::Up => Point { x: p.x - 1, y: p.y },
        Direction::Down => Point { x: p.x + 1, y: p.y },
        Direction::Left => Point { x: p.x, y: p.y - 1 },
        Direction::Right => Point { x: p.x, y: p.y + 1 },
    }
}

fn push_tile(board: &mut Vec<Vec<Tile>>, p: &Point, d: &Direction, make_move: bool) -> bool {
    if *d == Direction::Up || *d == Direction::Down {
        match board[p.x][p.y] {
            Tile::Empty => return true,
            Tile::Wall => return false,
            Tile::LeftBox | Tile::RightBox => {
                // Handle multiple boxes together
                match board[p.x][p.y] {
                    Tile::LeftBox => {
                        let mut can_move = true;
                        let left_push_point = get_point_in_direction(p, d);
                        let right_tile_pos = get_point_in_direction(&p, &Direction::Right);
                        let right_push_point = get_point_in_direction(&right_tile_pos, d);
                        if !push_tile(board, &left_push_point, d, false) {
                            can_move = false;
                        }
                        if !push_tile(board, &right_push_point, d, false) {
                            can_move = false;
                        }

                        if make_move && can_move {
                            assert!(push_tile(board, &left_push_point, d, make_move));
                            assert!(push_tile(board, &right_push_point, d, make_move));
                            board[left_push_point.x][left_push_point.y] = Tile::LeftBox;
                            board[right_push_point.x][right_push_point.y] = Tile::RightBox;
                            board[p.x][p.y] = Tile::Empty;
                            board[right_tile_pos.x][right_tile_pos.y] = Tile::Empty;
                        }
                        return can_move;
                    }
                    Tile::RightBox => {
                        let mut can_move = true;
                        let right_push_point = get_point_in_direction(p, d);
                        let left_box_pos = get_point_in_direction(&p, &Direction::Left);
                        let left_push_point = get_point_in_direction(&left_box_pos, d);
                        if !push_tile(board, &left_push_point, d, false) {
                            can_move = false;
                        }
                        if !push_tile(board, &right_push_point, d, false) {
                            can_move = false;
                        }

                        if make_move && can_move {
                            assert!(push_tile(board, &left_push_point, d, make_move));
                            assert!(push_tile(board, &right_push_point, d, make_move));
                            board[left_push_point.x][left_push_point.y] = Tile::LeftBox;
                            board[right_push_point.x][right_push_point.y] = Tile::RightBox;
                            board[p.x][p.y] = Tile::Empty;
                            board[left_box_pos.x][left_box_pos.y] = Tile::Empty;
                        }
                        return can_move;
                    }
                    _ => panic!("This shouldn't happen"),
                }
            }
            _ => {
                let push_point = get_point_in_direction(p, d);
                if push_tile(board, &push_point, d, make_move) {
                    if make_move {
                        board[push_point.x][push_point.y] = board[p.x][p.y];
                    }
                    return true;
                }
            }
        }
    }
    match board[p.x][p.y] {
        Tile::Empty => return true,
        Tile::Wall => return false,
        _ => {
            let push_point = get_point_in_direction(p, d);
            if push_tile(board, &push_point, d, make_move) {
                if make_move {
                    board[push_point.x][push_point.y] = board[p.x][p.y];
                }
                return true;
            }
        }
    }
    return false;
}

fn make_move(board: &mut Vec<Vec<Tile>>, robot: &Point, d: &Direction) -> Point {
    if push_tile(board, robot, d, true) {
        board[robot.x][robot.y] = Tile::Empty;
        return get_point_in_direction(robot, d);
    }
    return *robot;
}

pub fn day15(file: &str) {
    let file = File::open(file).expect("Failed to read file");
    let reader = BufReader::new(file);
    let mut board1: Vec<Vec<Tile>> = Vec::new();
    let mut board2: Vec<Vec<Tile>> = Vec::new();
    let mut moves: Vec<Direction> = Vec::new();

    let mut found_empty_line = false;
    let mut robot1_pos = Point { x: 0, y: 0 };
    let mut robot2_pos = Point { x: 0, y: 0 };

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // If we found an empty line already, we are reading moves
        if found_empty_line {
            let mut new_moves: Vec<Direction> = line
                .chars()
                .map(|c| match c {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => panic!("Invalid character in input"),
                })
                .collect();
            moves.append(&mut new_moves);
        } else {
            // If we haven't found an empty line, we are reading the board
            if line.trim().is_empty() {
                found_empty_line = true;
                continue;
            }
            let row: Vec<Tile> = line
                .chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '@' => {
                        robot1_pos.y = i.try_into().unwrap();
                        robot1_pos.x = board1.len();
                        Tile::Robot
                    }
                    '.' => Tile::Empty,
                    _ => panic!("Invalid character in input"),
                })
                .collect();
            board1.push(row);
            let row: Vec<Tile> = line
                .chars()
                .enumerate()
                .flat_map(|(i, c)| match c {
                    '#' => vec![Tile::Wall, Tile::Wall],
                    'O' => vec![Tile::LeftBox, Tile::RightBox],
                    '@' => {
                        robot2_pos.y = (i as isize * 2) as usize;
                        robot2_pos.x = board2.len();
                        vec![Tile::Robot, Tile::Empty]
                    }
                    '.' => vec![Tile::Empty, Tile::Empty],
                    _ => panic!("Invalid character in input"),
                })
                .collect();
            board2.push(row);
        }
    }

    for m in moves {
        robot1_pos = make_move(&mut board1, &robot1_pos, &m);
        robot2_pos = make_move(&mut board2, &robot2_pos, &m);
    }

    let mut part1 = 0;

    for (i, row) in board1.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == Tile::Box {
                part1 += 100 * i + j;
            }
        }
    }

    let mut part2 = 0;
    for (i, row) in board2.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == Tile::LeftBox {
                part2 += 100 * i + j;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
