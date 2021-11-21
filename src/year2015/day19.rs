use std::collections::{
    HashMap,
    HashSet
};

use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (molecule, replacements, _) = parse_lines(&lines);
    let count = count_distinct_replacements(&molecule, &replacements);
    println!("{}", count);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (molecule, _, reductions) = parse_lines(&lines);
    let count = count_reductions_to_e(&molecule, &reductions);
    println!("{}", count);
}

fn parse_lines(lines: &Vec<String>) -> (String, HashMap<String, Vec<String>>, Vec<(String, String)>) {
    let mut replacements = HashMap::<String, Vec<String>>::new();
    let mut reductions = Vec::<(String, String)>::new();
    let mut read_all_replacements = false;
    let mut molecule: String = String::new();
    for line in lines {
        if line == "" {
            read_all_replacements = true;
            continue;
        }
        if read_all_replacements {
            molecule = line.to_string();
            continue;
        }
        let parts = line.split(" => ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            panic!("Invalid syntax in replacement line.");
        }
        reductions.push((parts[1].to_string(), parts[0].to_string()));
        let entry = replacements.get_mut(parts[0]);
        match entry {
            None => {
                replacements.insert(parts[0].to_string(), vec![parts[1].to_string()]);
            },
            Some(v) => v.push(parts[1].to_string()),
        }
    }
    reductions.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    (molecule, replacements, reductions)
}

fn count_distinct_replacements(molecule: &str, replacements: &HashMap<String, Vec<String>>) -> usize {
    let mut resulting_molecules = HashSet::<String>::new();
    let mut i: usize = 0;
    let molecule_chars = molecule.chars().collect::<Vec<char>>();
    while i < molecule_chars.len() {
        // Try looking up a 1-character element
        let single_element = molecule_chars[i].to_string();
        match replacements.get(&single_element) {
            Some(subs) => {
                // Found it in the valid replacements - replace it and
                // add it to the set.
                for sub in subs {
                    let new_molecule = format!("{}{}{}", &molecule[..i], sub, &molecule[i+1..]);
                    resulting_molecules.insert(new_molecule);
                }
                i += 1;
            },
            None => {
                // Didn't find a 1-character element
                // Try looking up a 2-character element
                let double_element = single_element.clone() + &molecule_chars[i+1].to_string();
                match replacements.get(&double_element) {
                    Some(subs) => {
                        // Found it in the valid replacements - replace it and
                        // add it to the set.
                        for sub in subs {
                            let new_molecule = format!("{}{}{}", &molecule[..i], sub, &molecule[i+2..]);
                            resulting_molecules.insert(new_molecule);
                        }
                        i += 2;
                    },
                    None => {
                        // Can't do any replacements here, move on
                        i += 1;
                    }
                }
            }
        }
    }
    resulting_molecules.len()
}

fn count_reductions_to_e(start_molecule: &str, reductions: &Vec<(String, String)>) -> usize {
    let mut molecule = start_molecule.to_string();
    let mut reduction_count = 0;
    while molecule != "e" {
        for (pattern, result) in reductions {
            if molecule.contains(pattern) {
                molecule = molecule.replacen(pattern, result, 1);
                reduction_count += 1;
                break;
            }
        }
    }
    reduction_count
}