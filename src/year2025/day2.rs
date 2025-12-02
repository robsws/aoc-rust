use std::collections::HashSet;

use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let ranges = parse_input(lines.first().unwrap()); // only one line
    let max_of_ranges = get_max_of_ranges(&ranges);
    let mut total = 0;
    for i in 1..max_of_ranges {
        let id = make_repeated_num(i);
        if id > max_of_ranges {
            break;
        }
        if ranges.iter().any(|r| {
            r.includes(id)
        }) {
            total += id;
        }
    }
    println!("{}", total);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let ranges = parse_input(lines.first().unwrap()); // only one line
    let max_of_ranges = get_max_of_ranges(&ranges);
    let mut invalids = HashSet::<u64>::new();
    for i in 1..max_of_ranges {
        let ids = make_repeated_nums(i, max_of_ranges);
        if ids.len() == 0 {
            break;
        }
        for id in ids {
            if ranges.iter().any(|r| {
                r.includes(id)
            }) {
                invalids.insert(id);
            }
        }
    }
    println!("{}", invalids.iter().sum::<u64>());
}

fn parse_input(line: &str) -> Vec<NumRange> {
    line.split(',')
        .map(|str_range| {
            match str_range.split_once('-') {
                Some((min, max)) => NumRange{
                    min: min.parse().unwrap(),
                    max: max.parse().unwrap()
                },
                None => panic!("Bad formatting in input")
            }
        })
        .collect()
}

struct NumRange {
    min: u64,
    max: u64
}

impl NumRange {

    pub fn includes(&self, val: u64) -> bool {
        val >= self.min && val <= self.max
    }
}

fn get_max_of_ranges(ranges: &Vec<NumRange>) -> u64 {
    ranges.iter().max_by(|x, y| x.max.cmp(&y.max)).unwrap().max
}

fn make_repeated_num(n: u64) -> u64 {
    let nstr = n.to_string();
    let repeated = format!("{}{}", nstr, nstr);
    repeated.parse().unwrap()
}

fn make_repeated_nums(n: u64, max: u64) -> Vec<u64> {
    let mut nums = Vec::<u64>::new();
    let nstr = n.to_string();
    for i in 2..99 {
        let repeated = nstr.repeat(i).parse().unwrap();
        if repeated > max {
            break;
        }
        nums.push(repeated);
    }
    nums
}
