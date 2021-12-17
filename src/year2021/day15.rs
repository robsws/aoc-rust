use crate::{input_file::read_lines, data_structs::{Coord, Grid, MinPriorityQueue}};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let grid = parse_lines(lines);
    let lowest_risk = djikstra_path(grid);
    println!("{}", lowest_risk);
}

pub fn part2(input_file_path: &str) {

}

fn parse_lines(lines: Vec<String>) -> Grid<u8> {
    let elements = lines.concat().chars().map(
        |c|
        c.to_digit(10).unwrap_or_else(|| panic!("Unable to parse character to int: {}", c)) as u8
    ).collect();
    Grid::<u8>::with_elements(lines[0].len(), lines.len(), elements)
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