use crate::input_file::read_lines;

use regex::Regex;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let input = lines.concat();

    let total = sum_evaluated_muls(&input);

    println!("{}", total);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let input = lines.concat();

    // first remove all of the string that appears between "don't()" and "do()"
    let dont_do_regex = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let dont_to_end_regex = Regex::new(r"don't\(\).*$").unwrap();
    let filtered_input = dont_do_regex.replace_all(&input, "");
    let filtered_input = dont_to_end_regex.replace_all(&filtered_input, "");

    let total = sum_evaluated_muls(&filtered_input);
    
    println!("{}", total);
}

fn sum_evaluated_muls(input: &str) -> i32 {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    mul_regex.captures_iter(&input)
        .map(|caps| {
            let lvalue: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let rvalue: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            lvalue * rvalue
        })
        .sum()
}
