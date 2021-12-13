mod day1; mod day2; mod day3; mod day4; mod day5;
mod day6; mod day7; mod day8; mod day9;

/// Dispatch the correct function given the day and part
/// given.
pub fn solve(day: u8, part: u8, input_file_path: &str) {
    match (day, part) {
        (1, 1) => day1::part1(input_file_path),
        (1, 2) => day1::part2(input_file_path),
        (2, 1) => day2::part1(input_file_path),
        (2, 2) => day2::part2(input_file_path),
        (3, 1) => day3::part1(input_file_path),
        (3, 2) => day3::part2(input_file_path),
        (4, 1) => day4::part1(input_file_path),
        (4, 2) => day4::part2(input_file_path),
        (5, 1) => day5::part1(input_file_path),
        (5, 2) => day5::part2(input_file_path),
        (6, 1) => day6::part1(input_file_path),
        (6, 2) => day6::part2(input_file_path),
        (7, 1) => day7::part1(input_file_path),
        (7, 2) => day7::part2(input_file_path),
        (8, 1) => day8::part1(input_file_path),
        (8, 2) => day8::part2(input_file_path),
        (9, 1) => day9::part1(input_file_path),
        (9, 2) => day9::part2(input_file_path),
        _ => {
            eprintln!(
                "Solution to 2015 day {} part {} not yet implemented.",
                day, part
            );
        }
    }
}