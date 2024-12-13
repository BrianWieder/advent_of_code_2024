use std::{
    char,
    collections::VecDeque,
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug, Clone, Copy)]
enum State {
    Used(u32, u32),
    Free(u32),
}

fn defrag_part1(file_system: Vec<State>) -> Vec<u32> {
    let mut file_system = VecDeque::from(file_system);
    let mut defragged = Vec::new();
    while file_system.len() > 0 {
        let curr = file_system.pop_front().unwrap();
        match curr {
            State::Used(count, id) => {
                for _ in 0..count {
                    defragged.push(id);
                }
            }
            State::Free(count) => {
                let mut free = count;
                while free > 0 && file_system.len() > 0 {
                    let back = file_system.pop_back().unwrap();
                    match back {
                        State::Used(count, id) => {
                            if count > free {
                                for _ in 0..free {
                                    defragged.push(id);
                                }
                                file_system.push_back(State::Used(count - free, id));
                                free = 0;
                            } else if free > count {
                                for _ in 0..count {
                                    defragged.push(id);
                                }
                                free -= count;
                            } else {
                                for _ in 0..count {
                                    defragged.push(id);
                                }
                                free = 0;
                            }
                        }
                        State::Free(_) => {}
                    }
                }
            }
        }
    }
    return defragged;
}

fn defrag_part2(mut file_system: Vec<State>) -> Vec<u32> {
    for i in  (0..file_system.len()).rev() {
        match file_system[i] {
            State::Used(count, _) => {
                for x in 0..i {
                    match file_system[x] {
                        State::Used(_, _) => {}
                        State::Free(free_size) => {
                            if free_size >= count {
                                file_system[x] = file_system[i];
                                file_system[i] = State::Free(count);
                                if free_size > count {
                                    file_system.insert(x + 1, State::Free(free_size - count));
                                }
                                break;
                            }
                        }
                    }
                }
            }
            State::Free(_) => {}
        }
    }
    return file_system
        .iter()
        .map(|x| match x {
            State::Used(count, id) => vec![*id; *count as usize],
            State::Free(count) => vec![0; *count as usize],
        })
        .reduce(|mut a, mut b| {
            a.append(&mut b);
            return a;
        }).expect("Failed to reduce");
}

fn checksum(file_system: &Vec<u32>) -> u64 {
    let mut fs_checksum: u64 = 0;
    for i in 0..file_system.len() {
        fs_checksum += i as u64 * file_system[i] as u64;
    }
    return fs_checksum;
}

pub fn day9(file: &str) {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut chars_read = 0;
    let mut file_system = Vec::new();
    for char in reader.bytes() {
        let char = char.unwrap();
        let char = char::from(char);
        let count: u32 = char.to_digit(10).expect("Failed to parse char");
        if chars_read % 2 == 0 {
            file_system.push(State::Used(count, chars_read / 2));
        } else {
            file_system.push(State::Free(count));
        }
        chars_read += 1;
    }

    let defragged_part1 = defrag_part1(file_system.clone());
    let defragged_part2 = defrag_part2(file_system);

    println!("Part 1: {}", checksum(&defragged_part1));
    println!("Part 2: {}", checksum(&defragged_part2));
}
