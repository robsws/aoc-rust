use itertools::Itertools;

use crate::{data_structs::Grid, input_file::read_lines};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (nums, grids) = parse_lines(lines);
    let (last_num, winner) = find_bingo_winner(nums, grids, false).unwrap();
    let score = calculate_score(&winner, last_num);
    println!("{}", score);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (nums, grids) = parse_lines(lines);
    let (last_num, winner) = find_bingo_winner(nums, grids, true).unwrap();
    let score = calculate_score(&winner, last_num);
    println!("{}", score);
}

fn parse_lines(lines: Vec<String>) -> (Vec<u32>, Vec<Grid<BingoEntry>>) {
    let called_nums = 
        lines[0]
        .split(',')
        .map(|nstr| nstr.parse::<u32>().expect("Called number doesn't parse."))
        .collect_vec();
    let mut grids = Vec::<Grid<BingoEntry>>::new();
    for i in (1..lines.len()).step_by(6) {
        grids.push(parse_bingo_grid(&lines[i+1..i+6]))
    }
    (called_nums, grids)
}

fn parse_bingo_grid(lines: &[String]) -> Grid<BingoEntry> {
    // Chain iterators over the parsed numbers in each line
    // and create a grid with all the numbers.
    let elements: Vec<BingoEntry> = lines.iter().flat_map(
        |line|
        line.trim()
            .split_whitespace()
            .map(|nstr| nstr.parse::<u32>().expect("Called number doesn't parse."))
            .map(|num| BingoEntry { num, marked: false })
    ).collect_vec();
    Grid::<BingoEntry>::with_elements(lines.len(), lines.len(), elements)
}

fn find_bingo_winner(
    numbers_to_call: Vec<u32>,
    mut grids: Vec<Grid<BingoEntry>>,
    find_last: bool
) -> Option<(u32, Grid<BingoEntry>)> {
    let mut finished_grid_indexes = Vec::<usize>::new();
    let mut winner = None;
    for called_num in numbers_to_call {
        let mut i = 0;
        for grid in &mut grids {
            if finished_grid_indexes.contains(&i) {
                i += 1;
                continue;
            }
            mark_number(grid, called_num);
            if has_won(&grid) {
                if find_last {
                    finished_grid_indexes.push(i);
                    winner = Some((called_num, grid.clone()))
                } else {
                    return Some((called_num, grid.clone()))
                }
            }
            i += 1;
        }
    }
    winner
}

fn mark_number(grid: &mut Grid<BingoEntry>, num: u32) {
    for x in 0..grid.xsize {
        for y in 0..grid.ysize {
            let entry = grid.get_mut(x, y);
            if entry.num == num {
                entry.marked = true;
            }
        }
    }
}

fn has_won(grid: &Grid<BingoEntry>) -> bool {
    // check columns
    for x in 0..grid.xsize {
        let won = (0..grid.ysize)
            .map(|y| grid.get(x, y))
            .all(|e| e.marked);
        if won {
            return true;
        }
    }
    // check rows
    for y in 0..grid.ysize {
        let won = (0..grid.xsize)
            .map(|x| grid.get(x, y))
            .all(|e| e.marked);
        if won {
            return true;
        }
    }
    false
}

fn calculate_score(grid: &Grid<BingoEntry>, called_num: u32) -> u32 {
    let score_sum: u32 = grid.into_iter()
        .filter(|e| !e.marked)
        .map(|e| e.num)
        .sum();
    score_sum * called_num
}

#[derive(Clone)]
struct BingoEntry {
    num: u32,
    marked: bool
}