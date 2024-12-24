use std::{fs::File, io::{BufRead, BufReader}};

// Real grid size
// const GRID_SIZE: usize = 71;
// Example grid size
const GRID_SIZE: usize = 7;

const SECONDS: usize = 12;

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for cell in row {
            if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn day18(file: &str) {
    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut grid = vec![vec![false; GRID_SIZE]; GRID_SIZE];
    let mut lines = reader.lines();

    for _ in 0..SECONDS {
        let line = lines.next().expect("Unable to read line").expect("Unable to read line");
        let coords: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
        let y = coords[0];
        let x = coords[1];
        grid[x][y] = true;
    }

    print_grid(&grid);
}
