use crate::input_file::read_all_to_string;

/// Print the floor that Santa ends up on when following
/// the instructions in the file at input_file_path.
/// ( => go up a floor.
/// ) => go down a floor.
pub fn part1(input_file_path: &str) {
    let input: String = read_all_to_string(input_file_path);
    let mut floor: i32 = 0;
    for paren in input.chars() {
        match paren {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid character in file.")
        }
    }
    println!("{}", floor);
}

/// Print the index of the command that first takes
/// Santa underground when following
/// the instructions in the file at input_file_path.
/// ( => go up a floor.
/// ) => go down a floor.
pub fn part2(input_file_path: &str) {
    let input: String = read_all_to_string(input_file_path);
    let mut floor: i32 = 0;
    for (i, paren) in input.chars().enumerate() {
        match paren {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid character in file.")
        }
        if floor < 0 {
            println!("{}", i+1);
            break;
        }
    }
}