mod day1; mod day2;

/// Dispatch the correct function given the day and part
/// given.
pub fn solve(day: u8, part: u8, input_file_path: &str) {
    match (day, part) {
        (1, 1) => day1::part1(input_file_path),
        (1, 2) => day1::part2(input_file_path),
        (2, 1) => day2::part1(input_file_path),
        (2, 2) => day2::part2(input_file_path),
        _ => {
            eprintln!(
                "Solution to 2025 day {} part {} not yet implemented.",
                day, part
            );
        }
    }
}
