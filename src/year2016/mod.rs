mod day7; mod day8; mod day9; mod day10; mod day11;

/// Dispatch the correct function given the day and part
/// given.
pub fn solve(day: u8, part: u8, input_file_path: &str) {
    match (day, part) {
        (1, 1) => println!("Not yet implemented"), //day1::part1(input_file_path),
        (1, 2) => println!("Not yet implemented"), //day1::part2(input_file_path),
        (2, 1) => println!("Not yet implemented"), //day2::part1(input_file_path),
        (2, 2) => println!("Not yet implemented"), //day2::part2(input_file_path),
        (3, 1) => println!("Not yet implemented"), //day3::part1(input_file_path),
        (3, 2) => println!("Not yet implemented"), //day3::part2(input_file_path),
        (4, 1) => println!("Not yet implemented"), //day4::part1(input_file_path),
        (4, 2) => println!("Not yet implemented"), //day4::part2(input_file_path),
        (5, 1) => println!("Not yet implemented"), //day5::part1(input_file_path),
        (5, 2) => println!("Not yet implemented"), //day5::part2(input_file_path),
        (6, 1) => println!("Not yet implemented"), //day6::part1(input_file_path),
        (6, 2) => println!("Not yet implemented"), //day6::part2(input_file_path),
        (7, 1) => day7::part1(input_file_path),
        (7, 2) => day7::part2(input_file_path),
        (8, 1) => day8::part1(input_file_path),
        (8, 2) => day8::part2(input_file_path),
        (9, 1) => day9::part1(input_file_path),
        (9, 2) => day9::part2(input_file_path),
        (10, 1) => day10::part1(input_file_path),
        (10, 2) => day10::part2(input_file_path),
//        (11, 1) => day11::part1(input_file_path),
//        (11, 2) => day11::part2(input_file_path),
        (12, 1) => println!("Not yet implemented."), //day12::part1(input_file_path),
        (12, 2) => println!("Not yet implemented."), //day12::part2(input_file_path),
        (13, 1) => println!("Not yet implemented."), //day13::part1(input_file_path),
        (13, 2) => println!("Not yet implemented."), //day13::part2(input_file_path),
        (14, 1) => println!("Not yet implemented."), //day14::part1(input_file_path),
        (14, 2) => println!("Not yet implemented."), //day14::part2(input_file_path),
        (15, 1) => println!("Not yet implemented."), //day15::part1(input_file_path),
        (15, 2) => println!("Not yet implemented."), //day15::part2(input_file_path),
        (16, 1) => println!("Not yet implemented."), //day16::part1(input_file_path),
        (16, 2) => println!("Not yet implemented."), //day16::part2(input_file_path),
        (17, 1) => println!("Not yet implemented."), //day17::part1(input_file_path),
        (17, 2) => println!("Not yet implemented."), //day17::part2(input_file_path),
        (18, 1) => println!("Not yet implemented."), //day18::part1(input_file_path),
        (18, 2) => println!("Not yet implemented."), //day18::part2(input_file_path),
        (20, 1) => println!("Not yet implemented."), //day20::part1(input_file_path),
        (20, 2) => println!("Not yet implemented."), //day20::part2(input_file_path),
        (21, 1) => println!("Not yet implemented."), //day21::part1(input_file_path),
        (21, 2) => println!("Not yet implemented."), //day21::part2(input_file_path),
        _ => {
            eprintln!(
                "Solution to 2016 day {} part {} not yet implemented.",
                day, part
            );
        }
    }
}
