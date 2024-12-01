use crate::input_file::read_lines;

use std::{collections::HashMap, iter::zip};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (left_list, right_list) = parse_input(lines);
    let total = total_distance(left_list, right_list);
    println!("{}", total)
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (left_list, right_list) = parse_input(lines);
    let histogram = generate_histogram(&right_list);
    let total_similarity: i32 =
        left_list.into_iter()
            .map(|x| {
                match histogram.get(&x) {
                    Some(n) => x * (*n as i32),
                    None => 0,
                }
            })
            .sum();
    println!("{}", total_similarity);
}

fn parse_input(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    lines.into_iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            // construct a pair from the first two elements of the split.
            // will panic if there's any less than two.
            (
                parts.next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
                parts.next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
            )
        })
        // unzip converts an iterator of pairs into two containers,
        // one containing all the left elements, and one containing
        // all of the right elements
        .unzip()
}

fn total_distance(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> i32 {
    left_list.sort();
    right_list.sort();
    zip(left_list, right_list)
        .map(|(l, r)| (r-l).abs())
        .sum()
}

/// generate a histogram mapping of numbers to how often they appear in the list.
fn generate_histogram(list: &Vec<i32>) -> HashMap<i32,usize> {
    let mut histogram = HashMap::<i32,usize>::new();
    for x in list {
        match histogram.get_mut(x) {
            Some(n) => {
                *n += 1;
            },
            None => {
                histogram.insert(*x, 1);
            },
        }
    }
    histogram
}
