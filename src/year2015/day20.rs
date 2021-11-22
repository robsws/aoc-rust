use std::collections::{HashMap, HashSet};

use primes;

use crate::input_file::read_all_to_string;

pub fn part1(input_file_path: &str) {
    let target = read_all_to_string(input_file_path).parse::<u64>()
        .expect("Input is not numeric.");
    let house_num = first_to_target(target);
    println!("{}", house_num);
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