use itertools::Itertools;

use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let score: u32 = lines.iter().map(|l| corruption_score(l)).sum();
    println!("{}", score);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let scores = lines.iter().map(
        |l| autocomplete_score(l)
    ).filter(
        |s| *s != None // corrupt scores are None
    ).map(
        |o| o.unwrap()
    ).sorted().collect::<Vec<u64>>();
    let middle_index = scores.len() / 2;
    println!("{}", scores[middle_index]);
}

fn corruption_score(line: &str) -> u32 {
    let mut stack = Vec::<char>::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(c),
            '[' => stack.push(c),
            '{' => stack.push(c),
            '<' => stack.push(c),
            ')' => match stack.pop() {
                None => break,
                Some(d) => if d != '(' { return 3 }
            },
            ']' => match stack.pop() {
                None => break,
                Some(d) => if d != '[' { return 57 }
            },
            '}' => match stack.pop() {
                None => break,
                Some(d) => if d != '{' { return 1197 }
            },
            '>' => match stack.pop() {
                None => break,
                Some(d) => if d != '<' { return 25137 }
            },
            _ => panic!("Illegal character found")
        }
    }
    0
}

fn autocomplete_score(line: &str) -> Option<u64> {
    let mut stack = Vec::<char>::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(c),
            '[' => stack.push(c),
            '{' => stack.push(c),
            '<' => stack.push(c),
            ')' => match stack.pop() {
                None => break,
                Some(d) => if d != '(' { return None }
            },
            ']' => match stack.pop() {
                None => break,
                Some(d) => if d != '[' { return None }
            },
            '}' => match stack.pop() {
                None => break,
                Some(d) => if d != '{' { return None }
            },
            '>' => match stack.pop() {
                None => break,
                Some(d) => if d != '<' { return None }
            },
            _ => panic!("Illegal character found")
        }
    }
    stack.reverse();
    let mut score = 0;
    for c in stack {
        score *= 5;
        score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Illegal character found")
        }
    }
    Some(score)
}