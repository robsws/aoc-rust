use crate::{data_structs::Coord, input_file::read_all_to_string};

pub fn part1(input_file_path: &str) {
    let input = read_all_to_string(input_file_path);
    let pos = parse_input(input);
    let code = find_code(&pos);
    println!("{}", code);
}

fn parse_input(input: String) -> Coord {
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    let mut x = parts[17].chars();
    x.next_back();
    let x = x.as_str().parse().expect("Could not parse input row.");
    let mut y = parts[15].chars();
    y.next_back();
    let y = y.as_str().parse().expect("Could not parse input col.");
    Coord { x, y }
}

fn find_code(pos: &Coord) -> u64 {
    let mut x = 0;
    let mut y = 1;
    let mut start_y = 1;
    let mut code = 20151125;
    loop {
        while y >= 0 {
            code = (code * 252533) % 33554393;
            if pos.x == (x + 1) as usize && pos.y == (y + 1) as usize {
                return code;
            }
            x += 1;
            y -= 1;
        }
        start_y += 1;
        y = start_y;
        x = 0;
    }
}