use regex::{Captures, Regex};

use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let instruction_regex = Regex::new(r"(?P<dir>[LR])(?P<dist>\d+)").unwrap();
    let mut dial = LoopCounter {
        max: 100,
        count: 50,
        zero_crosses: 0,
    };
    let no_of_zeroes: i32 = lines
        .into_iter()
        .map(|line| match instruction_regex.captures(&line) {
            Some(captures) => {
                let dist = parse_i32_from_regex_capture(&captures, "dist");
                match captures.name("dir").unwrap().as_str() {
                    "L" => dial.add(dist * -1),
                    "R" => dial.add(dist),
                    _ => panic!("Invalid instruction"),
                };
                // println!("- dial {}", dial.value());
                if dial.value() == 0 {
                    1
                } else {
                    0
                }
            }
            None => panic!("Invalid instruction"),
        })
        .sum();
    println!("{}", no_of_zeroes);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let instruction_regex = Regex::new(r"(?P<dir>[LR])(?P<dist>\d+)").unwrap();
    let mut dial = LoopCounter {
        max: 100,
        count: 50,
        zero_crosses: 0,
    };
    for line in lines {
        match instruction_regex.captures(&line) {
            Some(captures) => {
                let dist = parse_i32_from_regex_capture(&captures, "dist");
                match captures.name("dir").unwrap().as_str() {
                    "L" => dial.add(dist * -1),
                    "R" => dial.add(dist),
                    _ => panic!("Invalid instruction"),
                };
            }
            None => panic!("Invalid instruction")
        }
    };
    println!("{}", dial.zero_crosses());
}

fn parse_i32_from_regex_capture(captures: &Captures, capture_name: &str) -> i32 {
    let value = captures.name(capture_name).unwrap().as_str();
    value
        .parse()
        .expect(format!("Invalid {}: {}", capture_name, value).as_str())
}

struct LoopCounter {
    pub max: i32,
    count: i32,
    zero_crosses: i32,
}

impl LoopCounter {
    pub fn add(&mut self, val: i32) {
        self.zero_crosses += (val / self.max).abs();
        if (self.count != 0) && ((val < 0 && (val % self.max).abs() > self.count) || (val > 0 && (val % self.max) > self.max - self.count)) {
            self.zero_crosses += 1
        }

        self.count = (self.count + val) % self.max;
        if self.count == 0 {
            self.zero_crosses += 1
        }
        if self.count < 0 {
            self.count = self.max + self.count;
        }
    }

    pub fn value(&self) -> i32 {
        self.count
    }

    pub fn zero_crosses(&self) -> i32 {
        self.zero_crosses
    }
}
