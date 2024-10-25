use crate::input_file::read_lines;

use regex::{Regex, Captures};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let commands = parse_lines(&lines);
    let mut screen = screen::Screen::new(50, 6);
    for command in commands {
        screen = screen.do_command(command)
    }
    println!("{}", screen.count_on())
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let commands = parse_lines(&lines);
    let mut screen = screen::Screen::new(50, 6);
    for command in commands {
        screen = screen.do_command(command)
    }
    println!("{}", screen)

}

fn parse_usize_from_regex_capture(captures: &Captures, capture_name: &str) -> usize {
    let value =
        captures
            .name(capture_name)
            .unwrap()
            .as_str();
    value.parse().expect(format!("Invalid {}: {}", capture_name, value).as_str())
}

fn parse_lines(lines: &Vec<String>) -> Vec<screen::Command> {
    let rect_regex = Regex::new(r"rect (?P<width>\d+)x(?P<height>\d+)").unwrap();
    let rotate_row_regex = Regex::new(r"rotate row y=(?P<row_y>\d+) by (?P<amount>\d+)").unwrap();
    let rotate_col_regex = Regex::new(r"rotate column x=(?P<col_x>\d+) by (?P<amount>\d+)").unwrap();

    let mut commands = Vec::<screen::Command>::new();
    for line in lines {
        match rect_regex.captures(&line) {
            Some(captures) => {
                let width = parse_usize_from_regex_capture(&captures, "width");
                let height = parse_usize_from_regex_capture(&captures, "height");
                commands.push(screen::Command::Rect(width, height));
                continue;
            },
            None => ()
        }
        match rotate_row_regex.captures(&line) {
            Some(captures) => {
                let row_y = parse_usize_from_regex_capture(&captures, "row_y");
                let amount = parse_usize_from_regex_capture(&captures, "amount");
                commands.push(screen::Command::RotateRow(row_y, amount));
                continue;
            },
            None => ()
        }
        match rotate_col_regex.captures(&line) {
            Some(captures) => {
                let col_x = parse_usize_from_regex_capture(&captures, "col_x");
                let amount = parse_usize_from_regex_capture(&captures, "amount");
                commands.push(screen::Command::RotateCol(col_x, amount));
            },
            None => {
                panic!("This line of input is not of any accepted pattern: {}", &line)
            }
        }
    }
    commands
}

mod screen {

    use crate::data_structs::Grid;
    
    use std::fmt;

    #[derive(Clone)]
    pub struct Screen {
        pixels: Grid<bool>,
    }

    pub enum Command {
        Rect(usize, usize),
        RotateRow(usize, usize),
        RotateCol(usize, usize)
    }

    impl Screen {
        pub fn new(width: usize, height: usize) -> Screen {
            let pixels = Grid::<bool>::new(width, height, false);
            Screen { pixels }
        }

        pub fn do_command(&self, command: Command) -> Screen {
            match command {
                Command::Rect(width, height) => self.draw_rect_top_left(width, height),
                Command::RotateRow(row_y, offset) => self.rotate_row(row_y, offset),
                Command::RotateCol(col_x, offset) => self.rotate_column(col_x, offset)
            }
        }

        pub fn width(&self) -> usize {
            return self.pixels.xsize
        }

        pub fn height(&self) -> usize {
            return self.pixels.ysize
        }

        pub fn count_on(&self) -> usize {
            self.pixels.into_iter().map(|pixel| if *pixel {1} else {0}).sum()
        }

        fn draw_rect_top_left(&self, width: usize, height: usize) -> Screen {
            let mut screen_prime = self.clone();
            for x in 0..width {
                for y in 0..height {
                    screen_prime.pixels.set(x, y, true);
                }
            }
            screen_prime
        }

        fn rotate_row(&self, y: usize, offset: usize) -> Screen {
            let mut screen_prime = self.clone();
            for x in 0..self.width() {
                let new_x = (x + offset) % self.width();
                let val = self.pixels.get(x, y);
                screen_prime.pixels.set(new_x, y, *val);
            }
            screen_prime
        }

        fn rotate_column(&self, x: usize, offset: usize) -> Screen {
            let mut screen_prime = self.clone();
            for y in 0..self.height() {
                let new_y = (y + offset) % self.height();
                let val = self.pixels.get(x, y);
                screen_prime.pixels.set(x, new_y, *val);
            }
            screen_prime            
        }

    }

    impl fmt::Display for Screen {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut repr = "".to_string();
            for y in 0..self.height(){
                for x in 0..self.width() {
                    repr.push_str(if *self.pixels.get(x, y) { "#" } else { "." })
                }
                repr.push_str("\n")
            }
            write!(f, "{}", repr)
        }
    }

}
