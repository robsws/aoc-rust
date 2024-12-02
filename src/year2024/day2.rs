use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let rows = parse_input(lines);
    let safe_count =
        rows.into_iter()
            .filter(|r| is_safe(r))
            .count();
    println!("{}", safe_count);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let rows = parse_input(lines);
    let safe_count =
        rows.into_iter()
            .filter(|r| is_safe_with_dampener(r))
            .count();
    println!("{}", safe_count);
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<i32>> {
    lines.into_iter()
        .map(|line| {
            line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(sequence: &Vec<i32>) -> bool {
    // numbers are all increasing
    sequence.windows(2)
        .all(|adj_nums| {
            let n = adj_nums[0];
            let m = adj_nums[1];
            let diff = (n-m).abs();
            n > m && diff >= 1 && diff <= 3
        })
    ||
    // or numbers are all decreasing
    sequence.windows(2)
        .all(|adj_nums| {
            let n = adj_nums[0];
            let m = adj_nums[1];
            let diff = (n-m).abs();
            m > n && diff >= 1 && diff <= 3
        })    
}

fn is_safe_with_dampener(sequence: &Vec<i32>) -> bool {
    // check if it's safe without removing any numbers
    if is_safe(sequence) {
        return true;
    }
    (0..sequence.len())
        .any(|i| {
            // build sequence without number at that index
            let dampened =
                sequence[..i].iter()
                    .chain(sequence[i+1..].iter())
                    .map(|n| *n)
                    .collect();
            // check if that's safe
            is_safe(&dampened)
        })
}
