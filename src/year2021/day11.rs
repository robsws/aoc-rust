use crate::{input_file::read_lines, data_structs::Grid};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let octopuses = parse_lines(lines);
    let flashes = simulate_octopuses(&octopuses, 100);
    println!("{}", flashes);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let octopuses = parse_lines(lines);
    let step = get_simultaneous_flash(&octopuses);
    println!("{}", step);
}

fn parse_lines(lines: Vec<String>) -> Grid<u8> {
    let elements = lines.concat().chars().map(
        |c|
        c.to_digit(10).unwrap_or_else(|| panic!("Unable to parse character to int: {}", c)) as u8
    ).collect();
    Grid::<u8>::with_elements(lines[0].len(), lines.len(), elements)
}

fn simulate_octopuses(octopuses: &Grid<u8>, steps: u32) -> u32 {
    let mut octopuses = octopuses.clone();
    let mut flashes = 0;
    for _ in 0..steps {
        // Raise all energy levels by 1
        for x in 0..10 {
            for y in 0..10 {
                let octopus = octopuses.get_mut(x, y);
                *octopus += 1
            }
        }
        // Flash octopuses with energy 9
        let mut modified = true;
        while modified {
            modified = false;
            for x in 0..10 {
                for y in 0..10 {
                    let octopus = octopuses.get_mut(x, y);
                    if *octopus == 10 {
                        modified = true;
                        flashes += 1;
                        *octopus += 1;
                        for x1 in x.saturating_sub(1) .. x+2 {
                            for y1 in y.saturating_sub(1) .. y+2 {
                                if x1 < 10 && y1 < 10 && !(x == x1 && y == y1) {
                                    let neighbour = octopuses.get_mut(x1, y1);
                                    if *neighbour < 10 {
                                        *neighbour += 1
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // Reset flashers to 0
        for x in 0..10 {
            for y in 0..10 {
                let octopus = octopuses.get_mut(x, y);
                if *octopus > 9 {
                    *octopus = 0;
                }
            }
        }
    }
    flashes
}

fn get_simultaneous_flash(octopuses: &Grid<u8>) -> u32 {
    let mut octopuses = octopuses.clone();
    for i in 1..1000000 {
        let mut flashes = 0;
        // Raise all energy levels by 1
        for x in 0..10 {
            for y in 0..10 {
                let octopus = octopuses.get_mut(x, y);
                *octopus += 1
            }
        }
        // Flash octopuses with energy 9
        let mut modified = true;
        while modified {
            modified = false;
            for x in 0..10 {
                for y in 0..10 {
                    let octopus = octopuses.get_mut(x, y);
                    if *octopus == 10 {
                        modified = true;
                        flashes += 1;
                        *octopus += 1;
                        for x1 in x.saturating_sub(1) .. x+2 {
                            for y1 in y.saturating_sub(1) .. y+2 {
                                if x1 < 10 && y1 < 10 && !(x == x1 && y == y1) {
                                    let neighbour = octopuses.get_mut(x1, y1);
                                    if *neighbour < 10 {
                                        *neighbour += 1
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // Check if everyone flashed
        if flashes == 100 {
            return i;
        }
        // Reset flashers to 0
        for x in 0..10 {
            for y in 0..10 {
                let octopus = octopuses.get_mut(x, y);
                if *octopus > 9 {
                    *octopus = 0;
                }
            }
        }
    }
    0
}