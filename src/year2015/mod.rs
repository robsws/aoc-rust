mod day1;
mod day6;

/// Dispatch the correct function given the day and part
/// given.
pub fn solve(day: u8, part: u8, input_file_path: &str) {
    match (day, part) {
        (1, 1) => day1::part1(input_file_path),
        (1, 2) => day1::part2(input_file_path),
        (6, 1) => day6::part1(input_file_path),
        (6, 2) => day6::part2(input_file_path),
        _ => {
            eprintln!(
                "Solution to 2015 day {} part {} not yet implemented.",
                day, part
            );
        }
    }
}