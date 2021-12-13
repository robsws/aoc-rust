use std::collections::HashSet;

use crate::{input_file::read_lines, data_structs::{Grid, Coord}};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let grid = parse_lines(lines);
    let low_points = low_points(&grid);
    let risk: u32 = low_points.iter().map(|p| *grid.get(p.x, p.y) as u32 + 1).sum();
    println!("{}", risk);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let grid = parse_lines(lines);
    let basins = get_basins(&grid);
    let basin_mult = basins[0].len() * basins[1].len() * basins[2].len();
    println!("{}", basin_mult);
}

fn parse_lines(lines: Vec<String>) -> Grid<u8> {
    let elements = lines.concat().chars().map(
        |c|
        c.to_digit(10).unwrap_or_else(|| panic!("Unable to parse character to int: {}", c)) as u8
    ).collect();
    Grid::<u8>::with_elements(lines[0].len(), lines.len(), elements)
}

fn low_points(grid: &Grid<u8>) -> Vec<Coord> {
    let mut low_points = Vec::<Coord>::new();
    for x in 0..grid.xsize {
        for y in 0..grid.ysize {
            let current = grid.get(x, y);
            if
                // check left is lower
                (x == 0 || grid.get(x-1, y) > current) &&
                // check up is lower
                (y == 0 || grid.get(x, y-1) > current) &&
                // check right is lower
                (x == grid.xsize - 1 || grid.get(x+1, y) > current) &&
                // check down is lower
                (y == grid.ysize - 1 || grid.get(x, y+1) > current)
            {
                low_points.push(Coord { x, y });
            }
        }
    }
    low_points
}

fn get_basins(grid: &Grid<u8>) -> Vec<HashSet<Coord>> {
    let low_points = low_points(&grid);
    let mut basins = Vec::<HashSet<Coord>>::new();
    for point in low_points {
        basins.push(get_points_in_basin(grid, &point))
    }
    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    basins
}

fn get_points_in_basin(grid: &Grid<u8>, coord: &Coord) -> HashSet<Coord> {
    let mut all_basin_coords = HashSet::<Coord>::new();
    all_basin_coords.insert(coord.to_owned());
    let mut unresolved_coords = HashSet::<Coord>::new();
    unresolved_coords.insert(coord.to_owned());

    while unresolved_coords.len() > 0 {
        let current_coord = unresolved_coords.iter().next().unwrap().to_owned();
        unresolved_coords.remove(&current_coord);
        let current_height = grid.get(current_coord.x, current_coord.y);
        if *current_height == 9 {
            continue;
        }
        all_basin_coords.insert(current_coord.to_owned());
        // check left
        if
            current_coord.x > 0 &&
            grid.get(current_coord.x - 1, current_coord.y) > current_height
        {
            unresolved_coords.insert(Coord{x: current_coord.x - 1, y: current_coord.y});
        }
        // check upwards
        if
            current_coord.y > 0 &&
            grid.get(current_coord.x, current_coord.y - 1) > current_height
        {
            unresolved_coords.insert(Coord{x: current_coord.x, y: current_coord.y - 1});
        }
        // check right
        if
            current_coord.x < grid.xsize - 1 &&
            grid.get(current_coord.x + 1, current_coord.y) > current_height
        {
            unresolved_coords.insert(Coord{x: current_coord.x + 1, y: current_coord.y});
        }
        // check upwards
        if
            current_coord.y < grid.ysize - 1 &&
            grid.get(current_coord.x, current_coord.y + 1) > current_height
        {
            unresolved_coords.insert(Coord{x: current_coord.x, y: current_coord.y + 1});
        }
    }
    all_basin_coords
}