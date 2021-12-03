use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    if lines.is_empty() {
        panic!("No data in input file.");
    }
    let common_bits = most_common_bits(&lines);
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..common_bits.len() {
        let exp = (common_bits.len() - i - 1) as u32;
        if common_bits[i] > 0 {
            // part of gamma
            gamma += 2i32.pow(exp);
        } else {
            // part of epsilon
            epsilon += 2i32.pow(exp);
        }
    }
    println!("{}", gamma * epsilon);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    if lines.is_empty() {
        panic!("No data in input file.");
    }
    let oxygen_rating = calculate_rating(&lines, true);
    let co2_rating = calculate_rating(&lines, false);
    let safety_rating = oxygen_rating * co2_rating;
    println!("{}", safety_rating);
}

fn most_common_bits(numbers: &Vec<String>) -> Vec<i32> {
    let mut counts = vec![0; numbers[0].len()];
    for num in numbers {
        let bits: Vec<char> = num.chars().collect();
        if bits.len() != counts.len() {
            panic!("Lines have unequal amounts of bits.");
        }
        for i in 0..bits.len() {
            match bits[i] {
                '0' => counts[i] -= 1,
                '1' => counts[i] += 1,
                _ => panic!("String is not a binary number.")
            }
        }
    }
    counts

}

fn calculate_rating(numbers: &Vec<String>, use_most_common: bool) -> u32 {
    let mut candidates = numbers.clone();
    for i in 0..numbers[0].len() {
        // if one left, we're done
        if candidates.len() == 1 {
            break;
        }
        if candidates.len() < 1 {
            panic!("No candidate left after filter.")
        }
        // find most common bit
        let mut ones = 0;
        let mut zeros = 0;
        for c in &candidates {
            let chars: Vec<char> = c.chars().collect();
            if chars[i] == '1' {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        let bit = if ones >= zeros {
            if use_most_common { '1' } else { '0' }
        } else {
            if use_most_common { '0' } else { '1' }
        };
        // filter candidates by bit
        candidates = candidates.into_iter()
            .filter(|c| {
                // Make sure current bit is the same as most/least common
                let chars: Vec<char> = c.chars().collect();
                chars[i] == bit
            }).collect();
    }
    binstr_to_int(&candidates[0])
}

fn binstr_to_int(binstr: &str) -> u32 {
    let mut int = 0;
    for (i, c) in binstr.chars().enumerate() {
        let exp = (binstr.len() - i - 1) as u32;
        if c == '1' {
            int += 2u32.pow(exp);
        }
    }
    int
}