use std::{collections::HashMap};

use regex::Regex;
use crate::input_file::read_lines;
use lazy_static::lazy_static;

lazy_static! {
    static ref TARGET_SUE: [(&'static str, u32); 10] = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1)
    ];
}

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let sues = parse_lines(&lines);
    let sue_index = match_sue(sues, false);
    match sue_index {
        Some(i) => println!("{}", i),
        None => panic!("No sue found that matches the target.")
    }
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let sues = parse_lines(&lines);
    let sue_index = match_sue(sues, true);
    match sue_index {
        Some(i) => println!("{}", i),
        None => panic!("No sue found that matches the target.")
    }
}

lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(
            r"^Sue \d+: (?P<key1>\w+): (?P<val1>\d+), (?P<key2>\w+): (?P<val2>\d+), (?P<key3>\w+): (?P<val3>\d+)$"
        ).unwrap();
}

fn parse_lines(lines: &Vec<String>) -> Vec<HashMap<String, u32>> {
    let mut sues = Vec::<HashMap<String, u32>>::new();
    for line in lines {
        let mut properties = HashMap::<String, u32>::new();
        let caps = LINE_RE.captures(&line)
            .expect(&format!("Input line did not match expected pattern. {}", line));
        for i in 1..4 {
            let regex_key_tag = format!("key{}", i);
            let regex_val_tag = format!("val{}", i);
            let key = caps.name(&regex_key_tag).unwrap().as_str();
            let value = caps.name(&regex_val_tag).unwrap().as_str();
            let value = value.parse::<u32>().expect("Invalid numeric value in line.");
            properties.insert(key.to_string(), value);
        }
        sues.push(properties);
    }
    sues
}

fn match_sue(sues: Vec<HashMap<String, u32>>, inequalities: bool) -> Option<usize> {
    for (i, sue) in sues.into_iter().enumerate() {
        let mut is_match = true;
        for (key, value) in TARGET_SUE.iter() {
            let keystr = *key;
            match sue.get(keystr) {
                None => (),
                Some(v) => {
                    if (
                        inequalities &&
                        (keystr == "cats" || keystr == "trees") &&
                        *v <= *value
                    ) || (
                        inequalities &&
                        (keystr == "pomeranians" || keystr == "goldfish") &&
                        *v >= *value
                    ) || (
                        (
                            !inequalities ||
                            !(keystr == "cats" || keystr == "trees" || keystr == "pomeranians" || keystr == "goldfish")
                        ) &&
                        *v != *value
                    ) {
                        is_match = false;
                        break;
                    }
                }
            }
        }
        if is_match {
            return Some(i + 1);
        }
    }
    None
}