use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const LINE_REGEX: &str =
    r"p=(?<x_pos>[0-9]+),(?<y_pos>[0-9]+) v=(?<x_vel>-?[0-9]+),(?<y_vel>-?[0-9]+)";
const SIM_TIME: i64 = 100;

// Real Data:
const TILES_WIDE: i64 = 101;
const TILES_TALL: i64 = 103;

// Example Data:
// const TILES_WIDE: i64 = 11;
// const TILES_TALL: i64 = 7;

struct Robot {
    x: i64,
    y: i64,
    x_vel: i64,
    y_vel: i64,
}

impl Robot {
    fn get_new_pos(&self, time: i64) -> (i64, i64) {
        let mut new_x_pos = (self.x + self.x_vel * time) % TILES_WIDE;
        let mut new_y_pos = (self.y + self.y_vel * time) % TILES_TALL;
        if new_x_pos < 0 {
            new_x_pos = TILES_WIDE + new_x_pos;
        }
        if new_y_pos < 0 {
            new_y_pos = TILES_TALL + new_y_pos;
        }
        return (new_x_pos, new_y_pos);
    }

    fn move_once(&mut self) {
        let (new_x_pos, new_y_pos) = self.get_new_pos(1);
        self.x = new_x_pos;
        self.y = new_y_pos;
    }
}

fn display(robots: &Vec<Robot>) {
    let mut grid = vec![vec![0; TILES_WIDE as usize]; TILES_TALL as usize];
    for robot in robots {
        grid[robot.y as usize][robot.x as usize] += 1;
    }
    for row in grid {
        println!(
            "{:?}",
            row.iter()
                .map(|x| if *x == 0 {
                    ".".to_string()
                } else {
                    x.to_string()
                })
                .collect::<String>()
        );
    }
}

fn part2(robots: &mut Vec<Robot>) -> i64 {
    let mut iters = 0;
    loop {
        iters += 1;
        for robot in robots.iter_mut() {
            robot.move_once();
        }
        println!("Time: {}", iters);
        display(robots);
        if iters >= SIM_TIME * 100 {
            break;
        }
    }
    return iters;
}

pub fn day14(file: &str) {
    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(file);
    let re = Regex::new(LINE_REGEX).unwrap();
    let mut robots = Vec::new();
    let mut quads = [0; 4];
    for line in reader.lines() {
        let line = line.unwrap();
        let matches = re.captures(&line).unwrap();
        let x_pos: i64 = matches
            .name("x_pos")
            .unwrap()
            .as_str()
            .parse()
            .expect("Failed to parse into number");
        let y_pos: i64 = matches
            .name("y_pos")
            .unwrap()
            .as_str()
            .parse()
            .expect("Failed to parse into number");

        let x_vel: i64 = matches
            .name("x_vel")
            .unwrap()
            .as_str()
            .parse()
            .expect("Failed to parse into number");
        let y_vel: i64 = matches
            .name("y_vel")
            .unwrap()
            .as_str()
            .parse()
            .expect("Failed to parse into number");

        let robot = Robot {
            x: x_pos,
            y: y_pos,
            x_vel: x_vel,
            y_vel: y_vel,
        };

        let (new_x_pos, new_y_pos) = robot.get_new_pos(SIM_TIME);
        if new_x_pos < TILES_WIDE / 2 {
            if new_y_pos < TILES_TALL / 2 {
                quads[0] += 1;
            } else if new_y_pos > TILES_TALL / 2 {
                quads[1] += 1;
            }
        } else if new_x_pos > TILES_WIDE / 2 {
            if new_y_pos < TILES_TALL / 2 {
                quads[2] += 1;
            } else if new_y_pos > TILES_TALL / 2 {
                quads[3] += 1;
            }
        }
        robots.push(robot);
    }
    part2(&mut robots);

    println!(
        "Part 1: {:?}",
        quads.iter().copied().reduce(|a, b| a * b).unwrap()
    );
    println!("Part 2: {:?}", part2(&mut robots));
}
