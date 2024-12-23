use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

struct Device<'a> {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    program: &'a Vec<i64>,
    pc: usize,
    outputs: Vec<i64>,
}

fn adv(device: &mut Device, operand: i64) {
    let operand = device.get_combo(operand);
    device.reg_a = device.reg_a / 2_i64.pow(operand.try_into().unwrap());
    device.pc += 2;
}
fn bxl(device: &mut Device, operand: i64) {
    device.reg_b = device.reg_b ^ operand;
    device.pc += 2;
}
fn bst(device: &mut Device, operand: i64) {
    let operand = device.get_combo(operand);
    device.reg_b = operand % 8;
    device.pc += 2;
}
fn jnz(device: &mut Device, operand: i64) {
    if device.reg_a == 0 {
        device.pc += 2;
    } else {
        device.pc = operand as usize;
    }
}
fn bxc(device: &mut Device, _: i64) {
    device.reg_b = device.reg_b ^ device.reg_c;
    device.pc += 2;
}
fn out(device: &mut Device, operand: i64) {
    let operand = device.get_combo(operand);
    device.outputs.push(operand % 8);
    device.pc += 2;
}
fn bdv(device: &mut Device, operand: i64) {
    let operand = device.get_combo(operand);
    device.reg_b = device.reg_a / 2_i64.pow(operand.try_into().unwrap());
    device.pc += 2;
}
fn cdv(device: &mut Device, operand: i64) {
    let operand = device.get_combo(operand);
    device.reg_c = device.reg_a / 2_i64.pow(operand.try_into().unwrap());
    device.pc += 2;
}

type Instruction = fn(&mut Device, i64);

fn get_instruction(opcode: i64) -> Instruction {
    match opcode {
        0 => adv,
        1 => bxl,
        2 => bst,
        3 => jnz,
        4 => bxc,
        5 => out,
        6 => bdv,
        7 => cdv,
        _ => panic!("Unknown opcode"),
    }
}

impl<'a> Device<'a> {
    fn get_combo(&self, operand: i64) -> i64 {
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Unknown operand"),
        }
    }

    fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                break;
            }
            let opcode = self.program[self.pc];
            let operand = self.program[self.pc + 1];
            let instruction = get_instruction(opcode);
            instruction(self, operand);
        }
    }
}

fn get_register_intial_value(it: &mut Lines<BufReader<File>>, register: &str) -> i64 {
    let line = it
        .next()
        .expect("Unable to read line")
        .expect("Unable to read line");
    let expected = format!("Register {}", register);
    assert!(line.starts_with(&expected));
    return line.split_whitespace().last().unwrap().parse().unwrap();
}

pub fn day17(file: &str) {
    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let reg_a_init = get_register_intial_value(&mut lines, "A");
    let reg_b_init = get_register_intial_value(&mut lines, "B");
    let reg_c_init = get_register_intial_value(&mut lines, "C");

    assert!(lines
        .next()
        .expect("Expected empty line")
        .expect("Expected empty line")
        .is_empty());

    let program: Vec<i64> = lines
        .next()
        .expect("Expected program line")
        .expect("Expected program line")
        .split_whitespace()
        .last()
        .expect("Error parsing program")
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut device = Device {
        reg_a: reg_a_init,
        reg_b: reg_b_init,
        reg_c: reg_c_init,
        program: &program,
        pc: 0,
        outputs: Vec::new(),
    };

    device.run();
    let output = device
        .outputs
        .iter()
        .map(|x| x.to_string())
        .reduce(|a, b| a + "," + &b)
        .or(Some("".to_string()))
        .expect("Error parsing output");
    println!("Part 1: {}", output);
}
