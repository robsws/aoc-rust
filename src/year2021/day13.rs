use std::collections::HashSet;

use itertools::Itertools;

use crate::{input_file::read_lines, data_structs::{Coord, Grid}};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (dots, folds) = parse_lines(lines);
    let dots = do_fold(dots, &folds[0]);
    println!("{}", dots.len());
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (mut dots, folds) = parse_lines(lines);
    for fold in folds {
        dots = do_fold(dots, &fold);
    }
    print_dots(&dots);
}

fn parse_lines(lines: Vec<String>) -> (HashSet<Coord>, Vec<Fold>) {
    let mut on_folds = false;
    let mut dots = HashSet::<Coord>::new();
    let mut folds = Vec::<Fold>::new();
    for line in lines {
        if line == "" {
            on_folds = true;
            continue;
        }
        if !on_folds {
            let parts: Vec<usize> = line.split(',').map(|n| n.parse().expect("Coord digit couldn't be parsed.")).collect();
            dots.insert(Coord {x: parts[0], y: parts[1]});
        } else {
            let parts: Vec<&str> = line.split('=').collect();
            let position = parts[1].parse::<usize>().expect("Could not parse fold axis.");
            let axis = match parts[0] {
                "fold along x" => FoldAxis::X,
                "fold along y" => FoldAxis::Y,
                _ => panic!("Invalid fold.")
            };
            folds.push(Fold {axis, position});
        }
    }
    (dots, folds)
}

struct Fold {
    axis: FoldAxis,
    position: usize
}

enum FoldAxis {
    X,
    Y
}

fn do_fold(dots: HashSet<Coord>, fold: &Fold) -> HashSet<Coord> {
    let mut new_dots = dots.clone();
    match fold.axis {
        FoldAxis::X => {
            for dot in dots {
                if dot.x > fold.position {
                    new_dots.remove(&dot);
                    let new_x = fold.position - (dot.x - fold.position);
                    new_dots.insert(Coord {x: new_x, y: dot.y});
                }
            }
        },
        FoldAxis::Y => {
            for dot in dots {
                if dot.y > fold.position {
                    new_dots.remove(&dot);
                    let new_y = fold.position - (dot.y - fold.position);
                    new_dots.insert(Coord {x: dot.x, y: new_y});
                }
            }
        }
    }
    new_dots
}

fn print_dots(dots: &HashSet<Coord>) {
    let max_x = dots.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let max_y = dots.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    let mut grid = Grid::<char>::new(max_x + 1, max_y + 1, '.');
    for dot in dots {
        grid.set(dot.x, dot.y, '#');
    }
    for y in 0 .. grid.ysize {
        let row = 
            (0 .. grid.xsize).map(|x| grid.get(x, y)).join("");
        println!("{}", row);
    }
}