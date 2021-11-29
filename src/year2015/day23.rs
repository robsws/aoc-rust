use std::collections::HashMap;

use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let instructions = parse_lines(lines);
    let mut registers = HashMap::<String, u32>::new();
    registers.insert("a".to_string(), 0);
    registers.insert("b".to_string(), 0);
    let output = run_program(instructions, &registers);
    println!("{}", output["b"]);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let instructions = parse_lines(lines);
    let mut registers = HashMap::<String, u32>::new();
    registers.insert("a".to_string(), 1);
    registers.insert("b".to_string(), 0);
    let output = run_program(instructions, &registers);
    println!("{}", output["b"]);
}

fn run_program(
    instructions: Vec<Instruction>,
    init_registers: &HashMap<String, u32>
) -> HashMap<String, u32> {
    let mut program_counter = 0isize;
    let mut registers = init_registers.clone();
    while program_counter >= 0 && program_counter < instructions.len() as isize {
        let instruction = instructions.get(program_counter as usize).unwrap();
        match instruction {
            Instruction::Half(r) => {
                match registers.get_mut(r) {
                    Some(v) => {
                        *v = *v / 2;
                        program_counter += 1;
                    },
                    None => panic!("Register {} does not exist.", r)
                }
            },
            Instruction::Triple(r) => {
                match registers.get_mut(r) {
                    Some(v) => {
                        *v = *v * 3;
                        program_counter += 1;
                    },
                    None => panic!("Register {} does not exist.", r)
                }
            },
            Instruction::Increment(r) => {
                match registers.get_mut(r) {
                    Some(v) => {
                        *v = *v + 1;
                        program_counter += 1;
                    },
                    None => panic!("Register {} does not exist.", r)
                }
            },
            Instruction::Jump(offset) => {
                program_counter += offset;
            },
            Instruction::JumpIfEven(r, offset) => {
                match registers.get_mut(r) {
                    Some(v) => {
                        if *v % 2 == 0 {
                            program_counter += offset;
                        } else {
                            program_counter += 1;
                        }
                    }
                    None => panic!("Register {} does not exist.", r)
                }
            },
            Instruction::JumpIfOne(r, offset) => {
                match registers.get_mut(r) {
                    Some(v) => {
                        if *v == 1 {
                            program_counter += offset;
                        } else {
                            program_counter += 1;
                        }
                    }
                    None => panic!("Register {} does not exist.", r)
                }
            },
        }
    }
    registers
}

enum Instruction {
    Half(String),
    Triple(String),
    Increment(String),
    Jump(isize),
    JumpIfEven(String, isize),
    JumpIfOne(String, isize)
}

fn parse_lines(lines: Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::with_capacity(lines.len());
    for line in lines {
        let line = line.replace(",", "");
        let tokens = line.split(' ').collect::<Vec<&str>>();
        if tokens.is_empty() {
            panic!("No instruction on line.");
        }
        let instruction = match tokens[0] {
            "hlf" => parse_hlf(&tokens[1..]),
            "tpl" => parse_tpl(&tokens[1..]),
            "inc" => parse_inc(&tokens[1..]),
            "jmp" => parse_jmp(&tokens[1..]),
            "jie" => parse_jie(&tokens[1..]),
            "jio" => parse_jio(&tokens[1..]),
            _ => panic!("Unknown instruction in list.")
        };
        instructions.push(instruction);
    }
    instructions
}

fn parse_hlf(args: &[&str]) -> Instruction {
    if args.len() != 1 {
        panic!("Wrong number of args to 'hlf'");
    }
    Instruction::Half(args[0].to_string())
}

fn parse_tpl(args: &[&str]) -> Instruction {
    if args.len() != 1 {
        panic!("Wrong number of args to 'tpl'");
    }
    Instruction::Triple(args[0].to_string())
}

fn parse_inc(args: &[&str]) -> Instruction {
    if args.len() != 1 {
        panic!("Wrong number of args to 'inc'");
    }
    Instruction::Increment(args[0].to_string())
}

fn parse_jmp(args: &[&str]) -> Instruction {
    if args.len() != 1 {
        panic!("Wrong number of args to 'jmp'");
    }
    Instruction::Jump(args[0].to_string().parse::<isize>().unwrap())
}

fn parse_jie(args: &[&str]) -> Instruction {
    if args.len() != 2 {
        panic!("Wrong number of args to 'jie'");
    }
    Instruction::JumpIfEven(args[0].to_string(), args[1].to_string().parse::<isize>().unwrap())
}

fn parse_jio(args: &[&str]) -> Instruction {
    if args.len() != 2 {
        panic!("Wrong number of args to 'jio'");
    }
    Instruction::JumpIfOne(args[0].to_string(), args[1].to_string().parse::<isize>().unwrap())
}