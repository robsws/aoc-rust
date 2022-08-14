use std::env;

mod year2015; mod year2016; mod year2021;
mod input_file;
mod data_structs;
mod binary;

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
        2015 => year2015::solve(day, part, input_file_path),
        2016 => year2016::solve(day, part, input_file_path),
        2021 => year2021::solve(day, part, input_file_path),
        _ => {
            eprintln!("Solutions to year {} not yet implemented.", year);
        }
    }
}