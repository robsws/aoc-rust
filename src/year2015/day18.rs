use crate::{
    data_structs::Grid,
    input_file::read_lines
};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let mut lights = parse_lines(&lines);
    for _ in 0..100 {
        lights = animate_lights(lights, false);
    }
    println!("{}", get_total_lights_on(&lights));
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let mut lights = parse_lines(&lines);
    for _ in 0..100 {
        lights = animate_lights(lights, true);
    }
    println!("{}", get_total_lights_on(&lights));
}

fn parse_lines(lines: &Vec<String>) -> Grid<bool> {
    let mut lights = Grid::<bool>::new(lines[0].len(), lines.len(), false);
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let light = match c {
                '#' => true,
                '.' => false,
                 _ => panic!("Invalid char in line, must be # or .")
            };
            lights.set(x, y, light);
        }
    }
    lights
}

fn animate_lights(lights: Grid<bool>, corners_always_on: bool) -> Grid<bool> {
    let mut next_lights = lights.clone();
    for x in 0..lights.xsize {
        for y in 0..lights.ysize {
            let surrounding_on = get_surrounding_lights_on(&lights, x, y);
            let is_on = *lights.get(x, y);
            if is_on && surrounding_on != 2 && surrounding_on != 3 {
                next_lights.set(x, y, false);
            }
            else if !is_on && surrounding_on == 3 {
                next_lights.set(x, y, true);
            }
        }
    }
    if corners_always_on {
        next_lights.set(0, 0, true);
        next_lights.set(0, lights.ysize-1, true);
        next_lights.set(lights.xsize-1, 0, true);
        next_lights.set(lights.xsize-1, lights.ysize-1, true);
    }
    next_lights
}

fn get_surrounding_lights_on(lights: &Grid<bool>, x: usize, y: usize) -> u32 {
    let mut lights_on = 0;
    for x1 in x.saturating_sub(1)..x+2 {
        if x1 >= lights.xsize {
            continue;
        }
        for y1 in y.saturating_sub(1)..y+2 {
            if y1 >= lights.ysize {
                continue;
            }
            if !(x == x1 && y == y1) && *lights.get(x1, y1) {
                lights_on += 1;
            }
        }
    }
    lights_on
}

fn get_total_lights_on(lights: &Grid<bool>) -> u32 {
    let mut total = 0;
    for light in lights {
        if *light {
            total += 1;
        }
    }
    total
}