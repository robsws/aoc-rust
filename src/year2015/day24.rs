use crate::input_file::read_lines;

use itertools::Itertools;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let presents = parse_lines(lines);
    let qe = find_best_passenger_qe(presents, 3);
    println!("{}", qe);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let presents = parse_lines(lines);
    let qe = find_best_passenger_qe(presents, 4);
    println!("{}", qe);
}

fn parse_lines(lines: Vec<String>) -> Vec<u64> {
    let mut presents = Vec::<u64>::with_capacity(lines.len());
    for line in lines {
        presents.push(line.parse::<u64>().expect("Line did not parse as integer."))
    }
    presents
}

fn find_best_passenger_qe(presents: Vec<u64>, compartments: usize) -> u64 {
    let target_weight = target_weight(&presents, compartments);
    let passenger_combos = find_smallest_combos(presents, target_weight);
    best_quantum_entanglement(passenger_combos)  
}

fn target_weight(presents: &Vec<u64>, compartments: usize) -> u64 {
    // add up all numbers and divide by 3 to get amount each section has to weigh
    let mut sum = 0;
    for p in presents {
        sum += p
    }
    if sum % compartments as u64 != 0 {
        panic!("Impossible to balance.")
    }
    sum / compartments as u64
}

fn find_smallest_combos(presents: Vec<u64>, target_weight: u64) -> Vec<Vec<u64>> {
    // find all smallest combos of presents that add up to target weight
    let mut best_combos = Vec::<Vec<u64>>::new();
    for n in 1..7 {
        for combo in presents.clone().into_iter().combinations(n) {
            if combo.iter().sum::<u64>() == target_weight {
                best_combos.push(combo);
            }
        }
        if best_combos.len() > 0 {
            break;
        }
    }
    best_combos
}

fn best_quantum_entanglement(combos: Vec<Vec<u64>>) -> u64 {
    // find combo with the best quantum entanglement
    let mut best_qe: u64 = combos[0].iter().product();
    for combo in combos {
        let qe: u64 = combo.iter().product();
        if qe < best_qe {
            best_qe = qe;
        }
    }
    best_qe
}