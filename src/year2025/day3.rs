use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let result: u32 = lines.iter().map(|l| largest_joltage_2(l)).sum();
    println!("{}", result);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let result: u64 = lines.iter().map(|l| largest_joltage_12(l)).sum();
    println!("{}", result);
}

fn largest_joltage_2(battery_bank: &str) -> u32 {
    let batteries: Vec<u32> =
        battery_bank.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
    
    let mut max_tens_idx = 0;
    let mut max_units_idx = 1;
    for i in 1..batteries.len() {
        if i != batteries.len() - 1 && batteries[max_tens_idx] < batteries[i] {
            max_tens_idx = i;
            max_units_idx = i + 1;
        }
        else if batteries[max_units_idx] < batteries[i] {
            max_units_idx = i;
        }
    }
    
    batteries[max_tens_idx] * 10 + batteries[max_units_idx]
}

fn largest_joltage_12(battery_bank: &str) -> u64 {
    let batteries: Vec<u64> =
        battery_bank.chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

    let mut rem_batteries = batteries.as_slice();
    let mut total: u64 = 0;
    
    for radix in (0..12).rev() {
        // do not consider righthand n numbers, because otherwise
        // there wouldn't be enough digits left for the full number
        let left_batteries =
            &rem_batteries[0..rem_batteries.len()-radix];
        // find the max battery
        let (max_i, max_battery) =
            max_with_index(left_batteries);
        // multiply that digit by 10 ^ n and add to total
        total += max_battery * (10u64.pow(radix as u32));
        // repeat on sublist right of the max battery
        rem_batteries = &rem_batteries[max_i+1..];
    }

    //println!("{}", total);

    total
}

fn max_with_index(vals: &[u64]) -> (usize, u64) {
    if vals.is_empty() {
        panic!("empty list give to max_with_index");
    }
    let mut max_val = vals[0];
    let mut max_i = 0;
    for i in 1..vals.len() {
        if vals[i] > vals[max_i] {
            max_val = vals[i];
            max_i = i;
        }
    }
    (max_i, max_val)
}
