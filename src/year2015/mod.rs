mod day1;
mod day6;
mod day7;
mod day8;
mod day12;
mod day13;

/// Dispatch the correct function given the day and part
/// given.
pub fn solve(day: u8, part: u8, input_file_path: &str) {
    match (day, part) {
        (1, 1) => day1::part1(input_file_path),
        (1, 2) => day1::part2(input_file_path),
        (6, 1) => day6::part1(input_file_path),
        (6, 2) => day6::part2(input_file_path),
        (7, 1) => day7::part1(input_file_path),
        (7, 2) => day7::part2(input_file_path),
        (8, 1) => day8::part1(input_file_path),
        (8, 2) => day8::part2(input_file_path),
        (12, 1) => day12::part1(input_file_path),
        (12, 2) => day12::part2(input_file_path),
        (13, 1) => day13::part1(input_file_path),
        (13, 2) => day13::part2(input_file_path),
        _ => {
            eprintln!(
                "Solution to 2015 day {} part {} not yet implemented.",
                day, part
            );
        }
    }
}