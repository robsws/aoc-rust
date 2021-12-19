use crate::{input_file::read_lines, data_structs::{Coord, Grid, MinPriorityQueue}};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let grid = parse_lines(lines);
    let lowest_risk = djikstra_path(grid);
    println!("{}", lowest_risk);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let grid = parse_lines(lines);
    let grid = expand_grid(&grid);
    let lowest_risk = djikstra_path(grid);
    println!("{}", lowest_risk);
}

fn parse_lines(lines: Vec<String>) -> Grid<u8> {
    let elements = lines.concat().chars().map(
        |c|
        c.to_digit(10).unwrap_or_else(|| panic!("Unable to parse character to int: {}", c)) as u8
    ).collect();
    Grid::<u8>::with_elements(lines[0].len(), lines.len(), elements)
}

fn expand_grid(grid: &Grid<u8>) -> Grid<u8> {
    let mut new_grid = Grid::new(grid.xsize * 5, grid.ysize * 5, 0);
    for tile_x in 0..5 {
        for tile_y in 0..5 {
            let sub_grid = raise_risk(&grid, tile_x + tile_y);
            splice_grid(
                &mut new_grid, 
                &sub_grid, 
                tile_x as usize * grid.xsize, 
                tile_y as usize * grid.ysize
            );
        }
    }
    new_grid
}

fn raise_risk(grid: &Grid<u8>, amount: u8) -> Grid<u8> {
    let mut new_grid = Grid::new(grid.xsize, grid.ysize, 0);
    for x in 0..grid.xsize {
        for y in 0..grid.ysize {
            let new_risk = (grid.get(x, y) + amount - 1) % 9 + 1;
            new_grid.set(x, y, new_risk);
        }
    }
    new_grid
}

fn splice_grid(big_grid: &mut Grid<u8>, small_grid: &Grid<u8>, offset_x: usize, offset_y: usize) {
    for x in 0..small_grid.xsize {
        for y in 0..small_grid.ysize {
            big_grid.set(x + offset_x, y + offset_y, *small_grid.get(x, y));
        }
    }
}

fn djikstra_path(grid: Grid<u8>) -> u32 {
    let start = Coord{x: 0, y: 0};
    let goal = Coord{x: grid.xsize - 1, y: grid.ysize - 1};

    // Build a priority queue of all squares in the grid
    let mut unvisited_queue = MinPriorityQueue::new();
    for x in 0..grid.xsize {
        for y in 0..grid.ysize {
            unvisited_queue.push(Coord{x, y}, u32::MAX);
        }
    }
    unvisited_queue.change_priority(&start, 0);

    loop {
        let (current, current_risk) = unvisited_queue.pop().unwrap();
        if current == goal {
            return current_risk;
        }
        // Check the space to the left of the current
        if current.x > 0 {
            let left = Coord {x: current.x - 1, y: current.y};
            update_risk(&mut unvisited_queue, &grid, current_risk, &left);
        }
        // Check the space to the above the current
        if current.y > 0 {
            let up = Coord {x: current.x, y: current.y - 1};
            update_risk(&mut unvisited_queue, &grid, current_risk, &up);
        }
        // Check the space to the right of the current
        if current.x < grid.xsize - 1 {
            let right = Coord {x: current.x + 1, y: current.y};
            update_risk(&mut unvisited_queue, &grid, current_risk, &right);
        }
        // Check the space below the current
        if current.y < grid.ysize - 1 {
            let down = Coord {x: current.x, y: current.y + 1};
            update_risk(&mut unvisited_queue, &grid, current_risk, &down);
        }
    }
}

fn update_risk(
    unvisited_queue: &mut MinPriorityQueue<Coord>,
    grid: &Grid<u8>,
    current_risk: u32,
    next: &Coord
) {
    match unvisited_queue.get(next) {
        None => (),
        Some((_, risk)) => {
            let next_risk = grid.get(next.x, next.y);
            let total_risk = current_risk + (*next_risk as u32);
            if total_risk < risk {
                unvisited_queue.change_priority(&next, total_risk);
            }
        }
    }
}