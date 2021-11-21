use std::{collections::HashMap, ptr::read};

use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (molecule, replacements) = parse_lines(&lines);
}

fn parse_lines(lines: &Vec<String>) -> (String, HashMap<String, Vec<String>>) {
    let mut replacements = HashMap::<String, Vec<String>>::new();
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
        let entry = replacements.get_mut(parts[0]);
        match entry {
            None => {
                replacements.insert(parts[0].to_string(), vec![parts[1].to_string()]);
            },
            Some(v) => v.push(parts[1].to_string()),
        }
    }
    (molecule, replacements)
}