use std::{cmp::{max, min}};

use itertools::Itertools;

use crate::{data_structs::{Coord, manhattan_dist}, input_file::read_lines};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let coords = parse_input(lines);

    // get all pairs of coords and find the largest manhattan distance
    // this will equate to the largest area if a rectangle is made
    let mut pairs = Vec::<(Coord, Coord)>::new();
    for (i, c1) in coords.iter().enumerate() {
        for (_, c2) in coords.iter().enumerate().skip_while(|(k,_)| k <= &i) {
            pairs.push((c1.clone(), c2.clone()));
        }
    }
    let sorted_by_largest_manhattan =
        pairs
            .iter()
            .sorted_by(|(a1, a2), (b1, b2)| {
                manhattan_dist(&b1, &b2).cmp(&manhattan_dist(&a1, &a2))
            })
            .collect_vec();
    let largest_corners = sorted_by_largest_manhattan.first().unwrap();
    println!("{:?}", largest_corners);
    let largest_area =
        (max(largest_corners.0.x, largest_corners.1.x) - min(largest_corners.0.x, largest_corners.1.x) + 1) *
            (max(largest_corners.0.y, largest_corners.1.y) - min(largest_corners.0.y, largest_corners.1.y) + 1);
    println!("{}", largest_area);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let coords = parse_input(lines);

    // get all pairs of coords and find the largest manhattan distance
    // this will equate to the largest area if a rectangle is made
    let mut pairs = Vec::<(Coord, Coord)>::new();
    for (i, c1) in coords.iter().enumerate() {
        for (_, c2) in coords.iter().enumerate().skip_while(|(k,_)| k <= &i) {
            pairs.push((c1.clone(), c2.clone()));
        }
    }
    let sorted_by_area =
        pairs
            .iter()
            .sorted_by(|rect_corners1, rect_corners2| {
                area_of_rect(rect_corners2).cmp(&area_of_rect(rect_corners1))
            })
            .collect_vec();

    let mut outline_points = Vec::<_>::new();
    for (edge_start, edge_end) in coords.iter().zip(coords.iter().cycle().skip(1)) {
        if edge_start.x == edge_end.x {
            for y in min(edge_start.y, edge_end.y)..max(edge_start.y, edge_end.y)+1 {
                outline_points.push(Coord {x: edge_start.x, y});
            }
        } else if edge_start.y == edge_end.y {
            for x in min(edge_start.x,edge_end.x)..max(edge_start.x, edge_end.x)+1 {
                outline_points.push(Coord {x, y: edge_start.y});
            }
        }
    }
    let outline_points = outline_points;

    // find the first pair that does not have an intersecting line
    for rect_corners in sorted_by_area {
        let intersect = outline_points.iter().any(|p| point_within_rect(p, rect_corners));
        if !intersect {
            // no intersection means this is our largest rect
            println!("{:?}", rect_corners);
            let largest_area =
                (max(rect_corners.0.x, rect_corners.1.x) - min(rect_corners.0.x, rect_corners.1.x) + 1) *
                    (max(rect_corners.0.y, rect_corners.1.y) - min(rect_corners.0.y, rect_corners.1.y) + 1);
            println!("{}", largest_area);
            break;
        }
    }
}

fn area_of_rect(rect_corners: &(Coord, Coord)) -> usize {
    (max(rect_corners.0.x, rect_corners.1.x) - min(rect_corners.0.x, rect_corners.1.x) + 1) *
        (max(rect_corners.0.y, rect_corners.1.y) - min(rect_corners.0.y, rect_corners.1.y) + 1)
}

fn parse_input(lines: Vec<String>) -> Vec<Coord> {
    lines
        .iter()
        .map(|line| {
            match line.split_once(',') {
                Some((x, y)) => Coord {
                        x: x.parse().unwrap(),
                        y: y.parse().unwrap()
                    },
                None => panic!("Bad formatting in input")
            }
        })
        .collect()
}

fn point_within_rect(point: &Coord, rect_corners: &(Coord, Coord)) -> bool {
    let ubound = min(rect_corners.0.y, rect_corners.1.y);
    let dbound = max(rect_corners.0.y, rect_corners.1.y);
    let lbound = min(rect_corners.0.x, rect_corners.1.x);
    let rbound = max(rect_corners.0.x, rect_corners.1.x);
    
    point.x > lbound && point.x < rbound && point.y > ubound && point.y < dbound
}
