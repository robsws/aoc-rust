use std::collections::HashSet;
use itertools::Itertools;
use num::{integer::sqrt, pow};

use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let coords = parse_input(lines);

    let mut pairs = Vec::<(Coord3, Coord3)>::new();
    for (i, c1) in coords.iter().enumerate() {
        for (_, c2) in coords.iter().enumerate().skip_while(|(k,_)| k <= &i) {
            pairs.push((c1.clone(), c2.clone()));
        }
    }
    let pairs = pairs.iter().sorted_by(|(a1, a2), (b1, b2)| {
        a1.dist_to(&a2).cmp(&b1.dist_to(&b2))
    }).collect_vec();

    let mut circuits = Vec::<HashSet<Coord3>>::new();
    // connect the 1000 closest together junction boxes
    for cxn in pairs.iter().take(1000) {
        circuits = add_connection_to_circuit(&circuits, &cxn.0, &cxn.1);
    }

    let result: u64 = circuits
        .iter()
        // sort in descending order of size of circuit
        .sorted_by(|a, b| {
            b.len().cmp(&a.len())
        })
        // get the biggest three circuits
        .take(3)
        // multiply the sizes
        .map(|c| {println!("{}", c.len()); c.len() as u64})
        .product();

    println!("{}", result);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let coords = parse_input(lines);

    let mut pairs = Vec::<(Coord3, Coord3)>::new();
    for (i, c1) in coords.iter().enumerate() {
        for (_, c2) in coords.iter().enumerate().skip_while(|(k,_)| k <= &i) {
            pairs.push((c1.clone(), c2.clone()));
        }
    }
    let pairs = pairs.iter().sorted_by(|(a1, a2), (b1, b2)| {
        a1.dist_to(&a2).cmp(&b1.dist_to(&b2))
    }).collect_vec();

    let mut circuits = Vec::<HashSet<Coord3>>::new();
    // connect the 1000 closest together junction boxes
    for cxn in pairs.iter() {
        circuits = add_connection_to_circuit(&circuits, &cxn.0, &cxn.1);
        // if everything is connected in one big circuit, return x coords multiplied
        if circuits.len() == 1 && circuits.first().unwrap().len() == coords.len() {
            println!("{}", cxn.0.x * cxn.1.x);
            break;
        }
    }
}

fn parse_input(lines: Vec<String>) -> Vec<Coord3> {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.splitn(3, ",");
            Coord3 {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
                z: parts.next().unwrap().parse().unwrap()
            }
        })
        .collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coord3 {
    x: i64,
    y: i64,
    z: i64
}

impl Coord3 {

    fn dist_to(&self, other: &Coord3) -> i64 {
        let vec = self.sub(other);
        sqrt(pow(vec.x, 2) + pow(vec.y, 2) + pow(vec.z,2))
    }

    fn sub(&self, other: &Coord3) -> Coord3 {
        Coord3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

fn add_connection_to_circuit(circuits: &Vec<HashSet<Coord3>>, box1: &Coord3, box2: &Coord3) -> Vec<HashSet<Coord3>> {
    let mut box1_circuit_idx: Option<usize> = None;
    let mut box2_circuit_idx: Option<usize> = None;
    
    // try and find boxes in existing circuits
    for i in 0..circuits.len() {
        let circuit = &circuits[i];
        if circuit.contains(box1) {
            box1_circuit_idx = Some(i);
        }
        if circuit.contains(box2) {
            box2_circuit_idx = Some(i);
        }
        match (box1_circuit_idx, box2_circuit_idx) {
            (Some(_), Some(_)) => break,
            _ => {}
        }
    }
    
    let mut new_circuits = circuits.clone();
    match (box1_circuit_idx, box2_circuit_idx) {
        (None, None) => {
            // create a new circuit
            new_circuits.push(
                HashSet::from([box1.to_owned(), box2.to_owned()])
            );
        },
        (Some(box1i), None) => {
            // add box2 to circuit where box1 was found
            new_circuits[box1i].insert(box2.clone());
        },
        (None, Some(box2i)) => {
            // add box2 to circuit where box1 was found
            new_circuits[box2i].insert(box1.clone());
        },
        (Some(box1i), Some(box2i)) => {
            // combine circuits where box1 and box2 were found
            if box1i != box2i {
                new_circuits[box1i].extend(circuits[box2i].clone());
                new_circuits.swap_remove(box2i);
            }
        }
    }
    new_circuits
}
