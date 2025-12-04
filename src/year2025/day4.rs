use crate::{data_structs::Grid, input_file::read_lines};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let grid = parse_input(lines);
    let result = accessible_rolls(&grid);
    println!("{}", result);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let mut grid = parse_input(lines);
    let mut total = 0;
    loop {
        let (new_grid, num_removed) = remove_accessible_rolls(&grid);
        if num_removed == 0 {
            break;
        }
        total += num_removed;
        grid = new_grid;
    }
    println!("{}", total);
}

fn parse_input(lines: Vec<String>) -> Grid<bool> {
    let elements: Vec<bool> =
        lines.iter().map(|line| {
            line.chars().map(|c| {
                match c {
                    '@' => true,
                    '.' => false,
                    _ => panic!("invalid char {} in input", c)
                }
            })
        }).flatten().collect();
    
    Grid::<bool>::with_elements(
        lines[0].len(),
        lines.len(),
        elements
    )
}

fn accessible_rolls(grid: &Grid<bool>) -> u32 {
    let mut total = 0u32;
    for x in 0isize..grid.xsize as isize {
        for y in 0isize..grid.ysize as isize {
            if !*grid.get(x as usize, y as usize) {
                continue;
            }
            let mut neighbours = 0;
            for xoff in -1isize..2isize {
                let curr_x = x as isize + xoff;
                if curr_x < 0 || curr_x >= grid.xsize as isize {
                    continue;
                }
                for yoff in -1..2 {
                    if xoff == 0 && yoff == 0 {
                        continue;
                    }
                    let curr_y = y + yoff;
                    if curr_y < 0 || curr_y >= grid.ysize as isize {
                        continue;
                    }
                    if *grid.get(curr_x as usize, curr_y as usize) {
                        neighbours += 1;
                    }
                }
            }
            if neighbours < 4 {
                total += 1;
            }
        }
    }
    total
}

fn remove_accessible_rolls(grid: &Grid<bool>) -> (Grid<bool>, u32) {
    let mut new_grid = grid.clone();
    let mut total = 0u32;
    for x in 0isize..grid.xsize as isize {
        for y in 0isize..grid.ysize as isize {
            if !*grid.get(x as usize, y as usize) {
                continue;
            }
            let mut neighbours = 0;
            for xoff in -1isize..2isize {
                let curr_x = x as isize + xoff;
                if curr_x < 0 || curr_x >= grid.xsize as isize {
                    continue;
                }
                for yoff in -1..2 {
                    if xoff == 0 && yoff == 0 {
                        continue;
                    }
                    let curr_y = y + yoff;
                    if curr_y < 0 || curr_y >= grid.ysize as isize {
                        continue;
                    }
                    if *grid.get(curr_x as usize, curr_y as usize) {
                        neighbours += 1;
                    }
                }
            }
            if neighbours < 4 {
                total += 1;
                new_grid.set(x as usize, y as usize, false);
            }
        }
    }
    (new_grid, total)
}
