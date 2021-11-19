use regex::Regex;
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;

use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (guests, happiness) = parse_lines(&lines);
    let (_, total_happiness) = find_optimal_seating(vec![], guests, &happiness);
    println!("{}", total_happiness);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (mut guests, mut happiness) = parse_lines(&lines);
    // Add myself
    for guest in &guests {
        happiness.insert((guest.to_owned(), "Rob".to_owned()), 0);
        happiness.insert(("Rob".to_owned(), guest.to_owned()), 0);
    }
    guests.push("Rob".to_owned());
    
    let (_, total_happiness) = find_optimal_seating(vec![], guests, &happiness);
    println!("{}", total_happiness);
}

lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(
            r"^(?P<name1>\w+) would (?P<gainorlose>gain|lose) (?P<units>\d+) happiness units by sitting next to (?P<name2>\w+)\.$"
        ).unwrap();
}

fn parse_lines(lines: &Vec<String>) -> (Vec<String>, HashMap<(String, String), i32>) {
    let mut happiness = HashMap::<(String, String), i32>::new();
    let mut guests = HashSet::<String>::new();
    for line in lines {
        let caps = LINE_RE.captures(&line)
            .expect(&format!("Input line did not match expected pattern. {}", line));
        let name1 = caps.name("name1").unwrap().as_str().to_owned();
        guests.insert(name1.clone());
        let name2 = caps.name("name2").unwrap().as_str().to_owned();
        let units = caps.name("units").unwrap().as_str();
        let units = units.parse::<i32>().expect("Could not parse amount of happiness units");
        let gain_or_lose = caps.name("gainorlose").unwrap().as_str();
        let signed_units;
        match gain_or_lose {
            "gain" => signed_units = units,
            "lose" => signed_units = -units,
            _ => panic!("Invalid syntax of line - 'gain' or 'lose' expected after 'would'.")
        }
        happiness.insert((name1, name2), signed_units);
    }
    (guests.into_iter().collect(), happiness)
}

fn find_optimal_seating(
    seated_guests: Vec<String>,
    guests_to_seat: Vec<String>,
    happiness: &HashMap<(String, String), i32>,
) -> (Vec<String>, i32) {
    // Base case - zero guests left to seat
    if guests_to_seat.len() == 0 {
        let final_happiness = calculate_happiness(&seated_guests, happiness);
        return (seated_guests, final_happiness);
    }
    // Recursive case - get the best seating plan
    // from all the permutations of non-seated guests
    let mut best_plan = vec![];
    let mut best_happiness = i32::MIN;
    for (i, guest) in (&guests_to_seat).into_iter().enumerate() {
        // Seat the guest
        let mut seated_guests_plus_guest = seated_guests.clone();
        let mut guests_to_seat_minus_guest = guests_to_seat.clone();
        seated_guests_plus_guest.push(guest.to_owned());
        guests_to_seat_minus_guest.remove(i);
        // Permute the rest of the non-seated guests
        let (best_sub_plan, best_sub_happiness) = 
            find_optimal_seating(seated_guests_plus_guest, guests_to_seat_minus_guest, &happiness);
        if best_sub_happiness > best_happiness {
            best_happiness = best_sub_happiness;
            best_plan = best_sub_plan;
        }
    }
    if best_plan.is_empty() {
        panic!("No plan was found with a higher happiness score than i32::MIN.");
    }
    (best_plan, best_happiness)
}

fn calculate_happiness(seated_guests: &Vec<String>, happiness: &HashMap<(String, String), i32>) -> i32 {
    let mut total_happiness = 0;
    for i in 0..seated_guests.len() {
        // Make the guests wrap around back to the first
        let right_guest_index = if i == seated_guests.len() - 1 {0} else {i+1};
        let left_guest = &seated_guests[i];
        let right_guest = &seated_guests[right_guest_index];
        total_happiness += happiness[&(left_guest.to_owned(), right_guest.to_owned())];
        total_happiness += happiness[&(right_guest.to_owned(), left_guest.to_owned())];
    }
    total_happiness
}