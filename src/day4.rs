use std::{fs::File, io::Read};

fn xmas_count_at_pos(lines: &Vec<Vec<char>>, x: i32, y: i32, rows: i32, cols: i32) -> u32 {
    if lines[x as usize][y as usize] != 'X' {
        return 0;
    }
    let mut count = 0;
    let goal = ['M', 'A', 'S'];
    // [[(usize, usize); 3]; 8]
    let indexes_to_check: [[(i32, i32); 3]; 8] = [
        [(x + 1, y), (x + 2, y), (x + 3, y)],
        [(x - 1, y), (x - 2, y), (x - 3, y)],
        [(x, y + 1), (x, y + 2), (x, y + 3)],
        [(x, y - 1), (x, y - 2), (x, y - 3)],
        [(x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)],
        [(x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)],
        [(x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)],
        [(x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)],
    ];
    for indexes in indexes_to_check.iter() {
        let mut found = true;
        for i in 0..indexes.len() {
            let (x, y) = indexes[i];
            if x >= rows || y >= cols || x < 0 || y < 0 {
                found = false;
                break;
            }
            if lines[x as usize][y as usize] != goal[i] {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
        }
    }
    return count;
}

fn is_mas_x(lines: &Vec<Vec<char>>, x: i32, y: i32, rows: i32, cols: i32) -> bool {
    if lines[x as usize][y as usize] != 'A' {
        return false;
    }
    if x <= 0 || x >= rows - 1 || y <= 0 || y >= cols - 1 {
        return false;
    }
    let x: usize = x as usize;
    let y: usize = y as usize;
    if (lines[x - 1][y - 1] != 'M' || lines[x + 1][y + 1] != 'S') && (lines[x - 1][y - 1] != 'S' || lines[x + 1][y + 1] != 'M') {
        return false;
    }
    if (lines[x - 1][y + 1] != 'M' || lines[x + 1][y - 1] != 'S') && (lines[x - 1][y + 1] != 'S' || lines[x + 1][y - 1] != 'M') {
        return false;
    }
    return true;
}


pub fn day4(file: &str) {
    let mut file = File::open(file).expect(format!("Failed to open file: {}", file).as_str());
    let mut lines = String::new();
    file.read_to_string(&mut lines)
        .expect("Failed to read file");

    let lines: Vec<Vec<char>> = lines.lines().map(|l| l.chars().collect()).collect();
    let rows : i32 = lines.len().try_into().unwrap();
    let cols: i32 = lines[0].len().try_into().unwrap();
    let mut part1_count = 0;
    let mut part2_count = 0;
    for line in 0..lines.len() {
        for c in 0..lines[line].len() {
            let line: i32 = line.try_into().unwrap();
            let c: i32 = c.try_into().unwrap();
            part1_count += xmas_count_at_pos(
                &lines,
                line,
                c,
                rows,
                cols,
            );
            if is_mas_x(&lines, line, c, rows, cols) {
                part2_count += 1;
            }
        }
    }
    println!("Part 1: {}", part1_count);
    println!("Part 2: {}", part2_count);
}
