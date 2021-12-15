use std::collections::HashMap;

use crate::{input_file::read_lines};

pub fn part1(input_file_path: &str) {
    do_part(input_file_path, 10);
}

pub fn part2(input_file_path: &str) {
    do_part(input_file_path, 40);
}

fn do_part(input_file_path: &str, iterations: u32) {
    let lines = read_lines(input_file_path);
    let (polymer, rules) = parse_lines(lines);
    let first = polymer.chars().next().unwrap();
    let last = polymer.chars().rev().next().unwrap();
    let mut polymer = string_polymer_to_pair_map(&polymer);
    for _ in 0..iterations {
        polymer = polymerize(&polymer, &rules);
    }
    let (least_common_count, most_common_count) = 
        least_and_most_common_element_count(&polymer, first, last);
    println!("{}", most_common_count - least_common_count);
}

fn parse_lines(lines: Vec<String>) -> (String, HashMap<(char, char), char>) {
    let polymer = lines[0].clone();
    let mut rules = HashMap::<(char, char), char>::new();
    for i in 2 .. lines.len() {
        let line = &lines[i];
        let parts: Vec<&str> = line.split(" -> ").collect();
        let key: Vec<char> = parts[0].chars().collect();
        let key = (key[0], key[1]);
        let val = parts[1].chars().next().unwrap();
        rules.insert(key, val);
    }
    (polymer, rules)
}

// fn polymerize(polymer: &str, rules: &HashMap<(char, char), char>) -> String {
//     let elements: Vec<char> = polymer.chars().collect();
//     let mut new_polymer = Vec::<char>::new();
//     for i in 0 .. elements.len() - 1 {
//         let current_element = elements[i];
//         let next_element = elements[i+1];
//         new_polymer.push(current_element);
//         match rules.get(&(current_element, next_element)) {
//             None => (),
//             Some(c) => new_polymer.push(*c)
//         }
//     }
//     new_polymer.push(elements[elements.len()-1]);
//     new_polymer.into_iter().collect()
// }

fn string_polymer_to_pair_map(polymer_str: &str) -> HashMap<(char, char), u64> {
    let elements: Vec<char> = polymer_str.chars().collect();
    let mut pairs = HashMap::<(char, char), u64>::new();
    for i in 0 .. elements.len() - 1 {
        let pair = (elements[i], elements[i+1]);
        match pairs.get_mut(&pair) {
            None => { pairs.insert(pair, 1); },
            Some(value) => *value += 1
        }
    }
    pairs
}

fn polymerize(
    polymer: &HashMap<(char, char), u64>,
    rules: &HashMap<(char, char), char>
) -> HashMap<(char, char), u64> {
    let mut new_polymer = polymer.clone();
    for rule in rules {
        // Find all cases of the rule in the polymer
        match polymer.get(rule.0) {
            None => (),
            Some(rule_pair_amount) => {
                let original_amount = *rule_pair_amount;
                // Work out the new pairs that are created as a result of the polymerization
                let new_pair_a = ((rule.0).0, *rule.1);
                let new_pair_b = (*rule.1, (rule.0).1);
                // Increment the amount of the newly created pairs by the amount of cases
                // of the original pair that matched the rule.
                match new_polymer.get_mut(&new_pair_a) {
                    None => { new_polymer.insert(new_pair_a, *rule_pair_amount); },
                    Some(new_pair_amount) => *new_pair_amount += *rule_pair_amount
                }
                match new_polymer.get_mut(&new_pair_b) {
                    None => { new_polymer.insert(new_pair_b, *rule_pair_amount); },
                    Some(new_pair_amount) => *new_pair_amount += *rule_pair_amount
                }
                // Decrement the amount of the originally matched pair by the amount
                // that there was originally
                match new_polymer.get_mut(&rule.0) {
                    None => panic!("original pair didn't appear in new polymer."),
                    Some(amount) => *amount -= original_amount
                }
            }
        }
    }
    new_polymer
}

// fn least_and_most_common_element_count(polymer: &str) -> (u32, u32) {
//     let mut count = HashMap::<char, u32>::new();
//     for element in polymer.chars() {
//         match count.get_mut(&element) {
//             None => { count.insert(element, 1); },
//             Some(value) => *value += 1
//         }
//     }
//     // println!("{:?}", count);
//     let max = count.values().max().unwrap().to_owned();
//     let min = count.values().min().unwrap().to_owned();
//     (min, max)
// }

fn least_and_most_common_element_count(
    polymer: &HashMap<(char, char), u64>,
    first: char,
    last: char
) -> (u64, u64) {
    // Count up all individual chars in the pairs
    let mut count = HashMap::<char, u64>::new();
    for pair in polymer {
        let char_a = (pair.0).0;
        let char_b = (pair.0).1;
        match count.get_mut(&char_a) {
            None => { count.insert(char_a, *pair.1); },
            Some(value) => *value += *pair.1
        }
        match count.get_mut(&char_b) {
            None => { count.insert(char_b, *pair.1); },
            Some(value) => *value += *pair.1
        }
    }
    // All chars appear twice as often in the map as they actually do
    // due to pair overlap, except first and last. Compensate by adding
    // 1 for first and last char.
    match count.get_mut(&first) {
        None => panic!("first character didn't appear in polymer"),
        Some(value) => *value += 1
    }
    match count.get_mut(&last) {
        None => panic!("last character didn't appear in polymer"),
        Some(value) => *value += 1
    }
    // Find max and min, and divide by 2 due to the doubling mentioned above
    let max = count.values().max().unwrap().to_owned() / 2;
    let min = count.values().min().unwrap().to_owned() / 2;
    (min, max)
}