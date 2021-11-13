use regex::Regex;
use lazy_static::lazy_static;
use crate::input_file::read_lines;
use crate::data_structs::{Coord, Grid};

/// Represents a single command for switching on/off
/// lights in the light matrix.
enum Command {
    Toggle(Coord, Coord),
    TurnOn(Coord, Coord),
    TurnOff(Coord, Coord)
}

/// Set a matrix of lights on or off depending on
/// instructions in the input file, and then calculate
/// how many lights are lit.
pub fn part1(input_file_path: &str) {
    let instructions = read_lines(input_file_path);
    let mut lights: Grid<bool> =
        Grid::new(1000, 1000, false);
    // Execute the instructions in the file.
    for instruction in instructions {
        match parse_instruction(&instruction) {
            Command::Toggle(from, to) => toggle_lights(&mut lights, from, to),
            Command::TurnOn(from, to) => turn_on_lights(&mut lights, from, to),
            Command::TurnOff(from, to) => turn_off_lights(&mut lights, from, to),
        }
    }
    // Count the lights switched on.
    let on_count = count_lights_on(&lights);
    println!("{}", on_count);
}

/// Set the brightness of matrix of lights depending on
/// instructions in the input file, and then calculate
/// total brightness of all lights.
pub fn part2(input_file_path: &str) {
    let instructions = read_lines(input_file_path);
    let mut lights: Grid<u32> =
        Grid::new(1000, 1000, 0);
    // Execute the instructions in the file.
    for instruction in instructions {
        match parse_instruction(&instruction) {
            Command::Toggle(from, to) => brighten_lights(&mut lights, from, to, 2),
            Command::TurnOn(from, to) => brighten_lights(&mut lights, from, to, 1),
            Command::TurnOff(from, to) => dim_lights(&mut lights, from, to, 1),
        }
    }
    // Count the lights switched on.
    let total_brightness = sum_total_brightness(&lights);
    println!("{}", total_brightness);
}

// Set up regex as static so it's not recompiled every
// iteration.
lazy_static! {
    static ref INSTR_RE: Regex =
        Regex::new(r"^(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
}

/// Parse a single instruction from the input file into
/// a Command object.
fn parse_instruction(line: &str) -> Command {
    match INSTR_RE.captures(&line) {
        None => panic!("Line in input file is not of expected format."),
        Some(caps) => {
            let command_str = caps.get(1).expect("No command found in line.").as_str();
            // Convert captures to the correct format
            let x1 = caps.get(2)
                .expect("No x1 found in line.")
                .as_str()
                .parse::<usize>()
                .expect("x1 was not a positive integer in line.");
            let y1 = caps.get(3)
                .expect("No y1 found in line.")
                .as_str()
                .parse::<usize>()
                .expect("y1 was not a positive integer in line.");
            let x2 = caps.get(4)
                .expect("No x2 found in line.")
                .as_str()
                .parse::<usize>()
                .expect("x2 was not a positive integer in line.");
            let y2 = caps.get(5)
                .expect("No y2 found in line.")
                .as_str()
                .parse::<usize>()
                .expect("y2 was not a positive integer in line.");
            return match command_str {
                "toggle" =>
                    Command::Toggle(
                        Coord {x: x1, y: y1},
                        Coord {x: x2, y: y2}
                    ),
                "turn on" =>
                    Command::TurnOn(
                        Coord {x: x1, y: y1},
                        Coord {x: x2, y: y2}
                    ),
                "turn off" =>
                    Command::TurnOff(
                        Coord {x: x1, y: y1},
                        Coord {x: x2, y: y2}
                    ),
                _ => panic!("Invalid command found in instruction.")
            }
        }
    }
}

/// Given a range of coordinates into the light matrix,
/// switch off those that are on and switch on those that
/// are off.
fn toggle_lights(grid: &mut Grid<bool>, start: Coord, end: Coord) {
    for x in start.x .. end.x+1 {
        for y in start.y .. end.y+1 {
            grid.set(x, y, !grid.get(x, y));
        }
    }
}

/// Given a range of coordinates into the light matrix,
/// switch off those lights. 
fn turn_off_lights(grid: &mut Grid<bool>, start: Coord, end: Coord) {
    for x in start.x .. end.x+1 {
        for y in start.y .. end.y+1 {
            grid.set(x, y, false);
        }
    }
}

/// Given a range of coordinates into the light matrix,
/// switch on those lights. 
fn turn_on_lights(grid: &mut Grid<bool>, start: Coord, end: Coord) {
    for x in start.x .. end.x+1 {
        for y in start.y .. end.y+1 {
            grid.set(x, y, true);
        }
    }
}

/// Given a range of coordinates into the light matrix,
/// make those lights brighter by the given amount.
fn brighten_lights(grid: &mut Grid<u32>, start: Coord, end: Coord, amount: u32) {
    for x in start.x .. end.x+1 {
        for y in start.y .. end.y+1 {
            grid.set(x, y, grid.get(x, y).saturating_add(amount));
        }
    }
}

/// Given a range of coordinates into the light matrix,
/// make those lights dimmer by the given amount.
fn dim_lights(grid: &mut Grid<u32>, start: Coord, end: Coord, amount: u32) {
    for x in start.x .. end.x+1 {
        for y in start.y .. end.y+1 {
            grid.set(x, y, grid.get(x, y).saturating_sub(amount));
        }
    }
}

/// Count how many lights are on in the given light matrix.
fn count_lights_on(grid: &Grid<bool>) -> u32 {
    let mut count: u32 = 0;
    for light in grid {
        if *light {
            count += 1;
        }
    }
    return count;
}

/// Calculate the total brightness of all the lights in the matrix.
fn sum_total_brightness(grid: &Grid<u32>) -> u32 {
    let mut total: u32 = 0;
    for light in grid {
        total += *light;
    }
    return total;
}