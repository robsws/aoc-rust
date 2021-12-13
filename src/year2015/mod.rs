mod day8;  mod day19; mod day16; 
mod day12; mod day20; mod day15; mod day24; mod day25;
mod day21; mod day23; mod day14; mod day17; mod day13;
mod day6;  mod day18; mod day7;  mod day1;  mod day22;

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
        (14, 1) => day14::part1(input_file_path),
        (14, 2) => day14::part2(input_file_path),
        (15, 1) => day15::part1(input_file_path),
        (15, 2) => day15::part2(input_file_path),
        (16, 1) => day16::part1(input_file_path),
        (16, 2) => day16::part2(input_file_path),
        (17, 1) => day17::part1(input_file_path),
        (17, 2) => day17::part2(input_file_path),
        (18, 1) => day18::part1(input_file_path),
        (18, 2) => day18::part2(input_file_path),
        (19, 1) => day19::part1(input_file_path),
        (19, 2) => day19::part2(input_file_path),
        (20, 1) => day20::part1(input_file_path),
        (20, 2) => day20::part2(input_file_path),
        (21, 1) => day21::part1(input_file_path),
        (21, 2) => day21::part2(input_file_path),
        (22, 1) => day22::part1(input_file_path),
        (22, 2) => day22::part2(input_file_path),
        (23, 1) => day23::part1(input_file_path),
        (23, 2) => day23::part2(input_file_path),
        (24, 1) => day24::part1(input_file_path),
        (24, 2) => day24::part2(input_file_path),
        (25, 1) => day25::part1(input_file_path),
        // (25, 2) => day25::part2(input_file_path),
        _ => {
            eprintln!(
                "Solution to 2015 day {} part {} not yet implemented.",
                day, part
            );
        }
    }
}