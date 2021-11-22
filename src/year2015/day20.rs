use std::collections::{HashMap, HashSet};
use primes;

use crate::input_file::read_all_to_string;

pub fn part1(input_file_path: &str) {
    let target = read_all_to_string(input_file_path).parse::<u64>()
        .expect("Input is not numeric.");
    let house_num = first_to_target(target);
    println!("{}", house_num);
}

pub fn part2(input_file_path: &str) {
    let target = read_all_to_string(input_file_path).parse::<u32>()
        .expect("Input is not numeric.");
    let house = first_to_target_limited_presents(target);
    println!("{}", house);
}

fn first_to_target(target: u64) -> u64 {
    let mut factorizer = Factorizer::new();
    // Remove factor of 10
    for i in 2..u64::MAX {
        let f = factorizer.get_factors(i).into_iter().collect::<HashSet<u64>>();
        let s = f.iter().sum::<u64>() * 10 + 10;
        if s >= target {
            return i;
        }
    }
    panic!("Didn't find an answer.");
}

struct Factorizer {
    factors: HashMap<u64, Vec<u64>>
}

impl Factorizer {
    pub fn new() -> Factorizer {
        Factorizer { factors: HashMap::<u64, Vec<u64>>::new() }
    }

    pub fn get_factors(&mut self, n: u64) -> Vec<u64> {
        match self.factors.get(&n) {
            Some(f) => return f.to_vec(),
            None => ()
        }
        let pfactors = primes::factors(n);
        let mut nfactors = vec![n];
        // Base case - one factor means n is prime
        if pfactors.len() > 1 {
            let pset = pfactors.into_iter().collect::<HashSet<u64>>();
            // Recursive case - divide by all factors
            for p in pset {
                let f = n / p;
                nfactors.extend(self.get_factors(f));
            }
        }
        self.factors.insert(n, nfactors.clone());
        nfactors
    }
}

const MAX_HOUSES: usize = 10000000;

fn first_to_target_limited_presents(target: u32) -> u32 {
    // need to use a vec so that it is heap allocated
    // stack is not large enough!
    let mut houses = vec![0; MAX_HOUSES];
    let mut max_found = 0;
    for elf in 1..MAX_HOUSES+1 {
        for present in 1..50+1 {
            let address = elf * present;
            if address >= MAX_HOUSES {
                break;
            }
            houses[address] += elf as u32 * 11;
        }
    }
    for i in 0..MAX_HOUSES {
        if houses[i] > target {
            return i as u32;
        }
        if houses[i] > max_found {
            max_found = houses[i];
        }
    }
    println!("{}", max_found);
    panic!("No house found that exceeds target.");
}