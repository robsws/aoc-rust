use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let readings = parse_lines(lines);
    let increasing = no_of_increasing_readings(readings);
    println!("{}", increasing);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let readings = parse_lines(lines);
    let sums = sums_of_three(readings);
    let increasing = no_of_increasing_readings(sums);
    println!("{}", increasing);
}

fn parse_lines(lines: Vec<String>) -> Vec<u32> {
    let mut readings = Vec::<u32>::new();
    for line in lines {
        let reading = line.parse::<u32>().expect("Could not parse number in input file.");
        readings.push(reading);
    }
    readings
}

fn no_of_increasing_readings(readings: Vec<u32>) -> u32 {
    let mut increasing_readings = 0;
    for i in 1..readings.len() {
        if readings[i] > readings[i-1] {
            increasing_readings += 1;
        }
    }
    increasing_readings
}

fn sums_of_three(readings: Vec<u32>) -> Vec<u32> {
    let mut sums = Vec::<u32>::new();
    for i in 2..readings.len() {
        sums.push(readings[i] + readings[i-1] + readings[i-2]);
    }
    sums
}