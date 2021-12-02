use core::panic;

use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let instructions = parse_lines(lines);
    let mut sub_coords = (0, 0);
    for inst in instructions {
        match inst {
            Instruction::Forward(v) => sub_coords.0 += v,
            Instruction::Down(v) => sub_coords.1 += v,
            Instruction::Up(v) => sub_coords.1 -= v,
        }
    }
    println!("{}", sub_coords.0 * sub_coords.1);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let instructions = parse_lines(lines);
    let mut sub_coords = (0, 0);
    let mut aim = 0;
    for inst in instructions {
        match inst {
            Instruction::Forward(v) => {
                sub_coords.0 += v;
                sub_coords.1 += aim * v
            }
            Instruction::Down(v) => aim += v,
            Instruction::Up(v) => aim -= v,
        }
    }
    println!("{}", sub_coords.0 * sub_coords.1);
}

fn parse_lines(lines: Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();
    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();
        if parts.len() != 2 {
            panic!("Invalid syntax for instruction.");
        }
        let amount = parts[1].parse::<u32>().expect("Could not parse value in instruction.");
        let instruction = match parts[0] {
            "forward" => Instruction::Forward(amount),
            "down" => Instruction::Down(amount),
            "up" => Instruction::Up(amount),
            _ => panic!("Unhandled instruction type")
        };
        instructions.push(instruction);
    }
    instructions
}

enum Instruction {
    Forward(u32),
    Up(u32),
    Down(u32)
}