use crate::input_file::read_all_to_string;
use lazy_static::lazy_static;

pub fn part1(input_file_path: &str) {
    let input = read_all_to_string(input_file_path);
    let crabs = parse_input(input);
    let fuel = best_crab_fuel(&crabs, false);
    println!("{}", fuel);
}

pub fn part2(input_file_path: &str) {
    let input = read_all_to_string(input_file_path);
    let crabs = parse_input(input);
    let fuel = best_crab_fuel(&crabs, true);
    println!("{}", fuel);
}

fn parse_input(input: String) -> Vec<u32> {
    input
        .split(',')
        .map(
            |s|
            s.parse().expect("Failed to parse input number")
        )
        .collect()
}

fn best_crab_fuel(crabs: &Vec<u32>, triangular: bool) -> u32 {
    let max_pos = crabs.iter().max().unwrap();
    let mut best_fuel = u32::MAX;
    for pos in 0..*max_pos {
        let fuel = calculate_fuel_used(crabs, pos, triangular);
        if fuel < best_fuel {
            best_fuel = fuel;
        }
    }
    best_fuel
}

fn triangle_nums() -> [usize; 2000] {
    let mut nums = [0; 2000];
    let mut curr = 0;
    for i in 0..2000 {
        curr += i;
        nums[i] = curr;
    }
    nums
}

lazy_static! {
    static ref TRIANGLE_NUMS: [usize; 2000] = triangle_nums();
}

fn calculate_fuel_used(crabs: &Vec<u32>, pos: u32, triangular: bool) -> u32 {
    crabs.iter()
        .map(
            |crab| {
                let mut fuel = i32::abs(pos as i32 - *crab as i32) as usize;
                if triangular {
                    fuel = TRIANGLE_NUMS[fuel as usize];
                }
                fuel as u32
            }
        )
        .sum()
}