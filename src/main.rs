use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        panic!("Usage: {} YEAR DAY PART INPUT_FILE", &args[0]);
    }

    let year: u16 = args[1].parse().expect("Error parsing argument year.");
    let day: u8 = args[2].parse().expect("Error parsing argument day.");
    let part: u8 = args[3].parse().expect("Error parsing argument part.");
    if part != 1 && part != 2 {
        panic!("Part must be 1 or 2.");
    }
    let input_file_path = &args[4];

    match year {
        2015 => {
            year2015::solve(day, part, input_file_path);
        },
        _ => {
            eprintln!("Solutions to year {} not yet implemented.", year);
        }
    }
}

mod year2015 {
    use crate::inputfiles;

    /// Dispatch the correct function given the day and part
    /// given.
    pub fn solve(day: u8, part: u8, input_file_path: &str) {
        match (day, part) {
            (1, 1) => day1part1(input_file_path),
            (1, 2) => day1part2(input_file_path),
            _ => {
                eprintln!(
                    "Solution to 2015 day {} part {} not yet implemented.",
                    day, part
                );
            }
        }
    }

    /// Print the floor that Santa ends up on when following
    /// the instructions in the file at input_file_path.
    /// ( => go up a floor.
    /// ) => go down a floor.
    fn day1part1(input_file_path: &str) {
        let input: String = inputfiles::read_all_to_string(input_file_path);
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
    fn day1part2(input_file_path: &str) {
        let input: String = inputfiles::read_all_to_string(input_file_path);
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
}

mod inputfiles {
    use std::fs;

    /// Read the entire input file into a string.
    pub fn read_all_to_string(
        file_path: &str
    ) -> String {
        return fs::read_to_string(file_path)
        .expect("Could not read the input file.");
    }
}