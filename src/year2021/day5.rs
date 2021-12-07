use std::{cmp::{max, min}, collections::HashSet};

use crate::{data_structs::Coord, input_file::read_lines};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let vents = parse_lines(lines);
    let crossings = find_crossings(vents, false);
    println!("{}", crossings.len());
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let vents = parse_lines(lines);
    let crossings = find_crossings(vents, true);
    println!("{}", crossings.len());
}

fn parse_lines(lines: Vec<String>) -> Vec<Vent> {
    let mut vents = Vec::<Vent>::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let start_parts: Vec<&str> = parts[0].split(',').collect();
        let end_parts: Vec<&str> = parts[1].split(',').collect();
        let start = Coord {
            x: start_parts[0].parse().expect("Could not parse x1 in line."),
            y: start_parts[1].parse().expect("Could not parse y1 in line."),
        };
        let end = Coord {
            x: end_parts[0].parse().expect("Could not parse x2 in line."),
            y: end_parts[1].parse().expect("Could not parse y2 in line."),
        };
        vents.push(Vent { start, end });
    }
    vents
}

fn find_crossings(vents: Vec<Vent>, allow_diagonal: bool) -> HashSet<Coord> {
    let mut crossings = HashSet::<Coord>::new();
    let mut spaces = HashSet::<Coord>::new();
    for vent in vents {
        if vent.start.x == vent.end.x {
            for y in min(vent.start.y, vent.end.y) ..= max(vent.start.y, vent.end.y) {
                let space = Coord {x: vent.start.x, y };
                if spaces.contains(&space) {
                    crossings.insert(space);
                } else {
                    spaces.insert(space);
                }
            }
        }
        else if vent.start.y == vent.end.y {
            for x in min(vent.start.x, vent.end.x) ..= max(vent.start.x, vent.end.x) {
                let space = Coord {x, y: vent.start.y };
                if spaces.contains(&space) {
                    crossings.insert(space);
                } else {
                    spaces.insert(space);
                }
            }
        }
        else if allow_diagonal {
            let dist = max(vent.start.x, vent.end.x) - min(vent.start.x, vent.end.x);
            let mut i = 0;
            while i <= dist {
                let mut space = Coord { x: vent.start.x+i, y: vent.start.y+i };
                if vent.start.x > vent.end.x {
                    space.x = vent.start.x - i;
                }
                if vent.start.y > vent.end.y {
                    space.y = vent.start.y - i;
                }
                if spaces.contains(&space) {
                    crossings.insert(space);
                } else {
                    spaces.insert(space);
                }
                i += 1
            }
        }
    }
    crossings
}

struct Vent {
    start: Coord,
    end: Coord
}