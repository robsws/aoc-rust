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
    /// Dispatch the correct function given the day and part
    /// given.
    pub fn solve(day: u8, part: u8, input_file_path: &str) {
        match (day, part) {
            (1, 1) => day1::part1(input_file_path),
            (1, 2) => day1::part2(input_file_path),
            (6, 1) => day6::part1(input_file_path),
            _ => {
                eprintln!(
                    "Solution to 2015 day {} part {} not yet implemented.",
                    day, part
                );
            }
        }
    }

    mod day1 {
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
    }

    mod day6 {
        use regex::Regex;
        use lazy_static::lazy_static;
        use crate::input_file::read_lines;
        use crate::data_structs::{Coord, Grid};

        /// Represents a single command for switching on/off
        /// lights in the light matrix.
        enum Command {
            Toggle(Coord, Coord),
            TurnOn(Coord, Coord),
            TurnOff(Coord, Coord)
        }

        /// Set a matrix of lights on or off depending on
        /// instructions in the input file, and then calculate
        /// how many lights are lit.
        pub fn part1(input_file_path: &str) {
            let instructions = read_lines(input_file_path);
            let mut lights: Grid<bool> =
                Grid::new(1000, 1000, false);
            // Execute the instructions in the file.
            for instruction in instructions {
                match parse_instruction(&instruction) {
                    Command::Toggle(from, to) => toggle_lights(&mut lights, from, to),
                    Command::TurnOn(from, to) => turn_on_lights(&mut lights, from, to),
                    Command::TurnOff(from, to) => turn_off_lights(&mut lights, from, to),
                }
            }
            // Count the lights switched on.
            let on_count = count_lights_on(&lights);
            println!("{}", on_count);
        }

        // Set up regex as static so it's not recompiled every
        // iteration.
        lazy_static! {
            static ref INSTR_RE: Regex =
                Regex::new(r"^(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        }

        /// Parse a single instruction from the input file into
        /// a Command object.
        fn parse_instruction(line: &str) -> Command {
            match INSTR_RE.captures(&line) {
                None => panic!("Line in input file is not of expected format."),
                Some(caps) => {
                    let command_str = caps.get(1).expect("No command found in line.").as_str();
                    // Convert captures to the correct format
                    let x1 = caps.get(2)
                        .expect("No x1 found in line.")
                        .as_str()
                        .parse::<usize>()
                        .expect("x1 was not a positive integer in line.");
                    let y1 = caps.get(3)
                        .expect("No y1 found in line.")
                        .as_str()
                        .parse::<usize>()
                        .expect("y1 was not a positive integer in line.");
                    let x2 = caps.get(4)
                        .expect("No x2 found in line.")
                        .as_str()
                        .parse::<usize>()
                        .expect("x2 was not a positive integer in line.");
                    let y2 = caps.get(5)
                        .expect("No y2 found in line.")
                        .as_str()
                        .parse::<usize>()
                        .expect("y2 was not a positive integer in line.");
                    return match command_str {
                        "toggle" =>
                            Command::Toggle(
                                Coord {x: x1, y: y1},
                                Coord {x: x2, y: y2}
                            ),
                        "turn on" =>
                            Command::TurnOn(
                                Coord {x: x1, y: y1},
                                Coord {x: x2, y: y2}
                            ),
                        "turn off" =>
                            Command::TurnOff(
                                Coord {x: x1, y: y1},
                                Coord {x: x2, y: y2}
                            ),
                        _ => panic!("Invalid command found in instruction.")
                    }
                }
            }
        }

        /// Given a range of coordinates into the light matrix,
        /// switch off those that are on and switch on those that
        /// are off.
        fn toggle_lights(grid: &mut Grid<bool>, start: Coord, end: Coord) {
            for x in start.x .. end.x+1 {
                for y in start.y .. end.y+1 {
                    grid.set(x, y, !grid.get(x, y));
                }
            }
        }

        /// Given a range of coordinates into the light matrix,
        /// switch off those lights. 
        fn turn_off_lights(grid: &mut Grid<bool>, start: Coord, end: Coord) {
            for x in start.x .. end.x+1 {
                for y in start.y .. end.y+1 {
                    grid.set(x, y, false);
                }
            }
        }

        /// Given a range of coordinates into the light matrix,
        /// switch on those lights. 
        fn turn_on_lights(grid: &mut Grid<bool>, start: Coord, end: Coord) {
            for x in start.x .. end.x+1 {
                for y in start.y .. end.y+1 {
                    grid.set(x, y, true);
                }
            }
        }

        /// Count how many lights are on in the given light matrix.
        fn count_lights_on(grid: &Grid<bool>) -> u32 {
            let mut count: u32 = 0;
            for light in grid {
                if *light {
                    count += 1;
                }
            }
            return count;
        }
    }
}

mod input_file {
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    /// Read the entire input file into a string.
    pub fn read_all_to_string(
        file_path: &str
    ) -> String {
        return fs::read_to_string(file_path)
        .expect("Could not read the input file.");
    }

    /// Read the entire input file into a vector, split
    /// by lines.
    pub fn read_lines(
        file_path: &str
    ) -> Vec<String> {
        let file: File = File::open(file_path)
            .expect("Problem opening input file.");
        let reader = BufReader::new(file);
        let mut lines = Vec::<String>::new();
        for line in reader.lines() {
            let line = line.expect("Issue reading line from input file.");
            lines.push(line);
        }
        return lines;
    }
}

mod data_structs {

    pub struct Coord {
        pub x: usize,
        pub y: usize
    }

    pub struct Grid<T: Clone> {
        xsize: usize,
        elements: Vec<T>
    }

    impl<'a, T: Clone> Grid<T> {
        /// Create a new grid of size xsize x ysize
        /// filled with the element 'fill_with'.
        pub fn new(
            xsize: usize,
            ysize: usize,
            fill_with: T
        ) -> Grid<T> {
            let elements = vec![fill_with; xsize * ysize];
            Grid{xsize, elements}
        }

        /// Read the element at x,y in the grid
        pub fn get(
            &self,
            x: usize,
            y: usize
        ) -> &T {
            self.check_bounds(x, y);
            let index = self.calc_index(x, y);
            return &self.elements[index];
        }

        /// Set the element at x,y to the given value
        pub fn set(
            &mut self,
            x: usize,
            y: usize,
            val: T
        ) {
            self.check_bounds(x, y);
            let index = self.calc_index(x, y);
            self.elements[index] = val;
        }

        /// Make sure that the given x, y coords are
        /// within the bounds of the grid.
        fn check_bounds(
            &self,
            x: usize,
            y: usize
        ) {
            if x * y > self.elements.len() {
                panic!("Index {},{} out of bounds of grid.", x, y);
            }
        }

        /// Calculate the index of the element in the
        /// elements vector given the x and y coords.
        fn calc_index(
            &self,
            x: usize,
            y: usize
        ) -> usize {
            y*self.xsize + x
        }
    }

    // The lifetime specifier here makes sure that
    // the elements reference living inside the iterator
    // does not outlive the grid.
    impl<'a, T: Clone> IntoIterator for &'a Grid<T> {
        type Item = &'a T;
        type IntoIter = GridIterator<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            GridIterator {
                elements: &self.elements,
                index: 0
            }
        }
    }

    pub struct GridIterator<'a, T> {
        elements: &'a Vec<T>,
        index: usize
    }

    impl<'a, T> Iterator for GridIterator<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.elements.len() {
                let item = &self.elements[self.index];
                self.index += 1;
                return Some(item);
            } else {
                return None;
            }
        }
    }
}