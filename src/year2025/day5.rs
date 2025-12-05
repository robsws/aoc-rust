use std::collections::HashSet;

use itertools::WhileSome;

use crate::input_file::read_lines;
use crate::data_structs::NumRange;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (ranges, nums) = parse_input(lines);
    let total = nums.iter().filter(|num| {
        ranges.iter().filter(|r| {
            r.includes(**num)
        }).count() > 0
    }).count();
    println!("{}", total);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (mut ranges, _) = parse_input(lines);
    ranges.sort_by(|r,s| {r.min.cmp(&s.min)});
    let mut combined_ranges = Vec::<NumRange>::new();
    let mut deleted_range_ids = HashSet::<usize>::new();
    for i in 0..ranges.len() {
        if deleted_range_ids.contains(&i) {
            continue;
        }
        let mut curr_range = ranges[i].clone();
        for j in i+1..ranges.len() {
            if deleted_range_ids.contains(&j) {
                continue;
            }
            let cmp_range = &ranges[j];
            match curr_range.combine_with(cmp_range) {
                Some(r) => {
                    curr_range = r;
                    deleted_range_ids.insert(j);
                },
                None => {}
            }
        }
        combined_ranges.push(curr_range);
    }
    let id_count: u64 =
        combined_ranges.iter().map(|r| {
            r.member_count()
        }).sum();
    println!("{}", id_count);
}

fn parse_input(lines: Vec<String>) -> (Vec<NumRange>, Vec<u64>) {
    let mut range_mode = true;
    let mut ranges = Vec::<NumRange>::new();
    let mut nums = Vec::<u64>::new();
    for line in lines {
        if line.is_empty() {
            range_mode = false;
            continue;
        }
        if range_mode {
            ranges.push(
                match line.split_once('-') {
                    Some((min, max)) => NumRange{
                        min: min.parse().unwrap(),
                        max: max.parse().unwrap()
                    },
                    None => panic!("Bad formatting in input ranges")
                }
            );
        } else {
            nums.push(line.parse().unwrap());
        }
    }
    (ranges, nums)
}

