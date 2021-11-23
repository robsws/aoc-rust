use core::panic;
use std::cmp::max;

use regex::Regex;

use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let boss = parse_lines(lines);
    let item_combos = get_item_combos();
    let mut lowest_cost_to_win: i32 = -1;
    for combo in item_combos {
        let player = Fighter {
            health: 100,
            damage: combo.damage,
            armour: combo.armour
        };
        let win = battle(player.clone(), boss.clone());
        if win {
            lowest_cost_to_win = combo.cost;
            break;
        }
    }
    if lowest_cost_to_win == -1 {
        panic!("Did not find a single combo of items that won.");
    }
    println!("{}", lowest_cost_to_win);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let boss = parse_lines(lines);
    let item_combos = get_item_combos();
    let mut highest_cost_to_lose: i32 = -1;
    for combo in item_combos {
        let player = Fighter {
            health: 100,
            damage: combo.damage,
            armour: combo.armour
        };
        let win = battle(player.clone(), boss.clone());
        if !win {
            highest_cost_to_lose = combo.cost;
        }
    }
    if highest_cost_to_lose == -1 {
        panic!("Did not find a single combo of items that lost.");
    }
    println!("{}", highest_cost_to_lose);
}

fn parse_lines(lines: Vec<String>) -> Fighter {
    let number_regex = Regex::new(r"\d+").unwrap();
    if lines.len() != 3 {
        panic!("Wrong number of lines in input file.");
    }
    Fighter {
        health: number_regex.find(&lines[0])
            .unwrap().as_str().parse::<i32>().expect("Hit points not numeric"),
        damage: number_regex.find(&lines[1])
            .unwrap().as_str().parse::<i32>().expect("Damage not numeric"),
        armour: number_regex.find(&lines[2])
            .unwrap().as_str().parse::<i32>().expect("Armour not numeric"),
    }
}

fn get_item_combos() -> Vec<Item> {
    let weapons = vec![
        Item {cost: 8, damage: 4, armour: 0},
        Item {cost: 10, damage: 5, armour: 0},
        Item {cost: 25, damage: 6, armour: 0},
        Item {cost: 40, damage: 7, armour: 0},
        Item {cost: 74, damage: 8, armour: 0},
    ];
    // Represent each valid combo as a single item struct
    let mut combos = Vec::<Item>::new();
    // Exactly one weapon must be picked
    let armour_rings_combos = get_armour_rings_combos();
    for weapon in &weapons {
        for combo in &armour_rings_combos {
            combos.push(add_items(weapon, combo));
        }
    }
    combos.sort_by(|c1, c2| c1.cost.cmp(&c2.cost));
    combos
}

fn get_armour_rings_combos() -> Vec<Item> {
    let armours = vec![
        Item {cost: 0, damage: 0, armour: 0},
        Item {cost: 13, damage: 0, armour: 1},
        Item {cost: 31, damage: 0, armour: 2},
        Item {cost: 53, damage: 0, armour: 3},
        Item {cost: 75, damage: 0, armour: 4},
        Item {cost: 102, damage: 0, armour: 5},
    ];
    let mut combos = Vec::<Item>::new();
    let rings_combos = get_rings_combos();
    for armour in &armours {
        for combo in &rings_combos {
            combos.push(add_items(armour, combo));
        }
    }
    combos
}

fn get_rings_combos() -> Vec<Item> {
    let rings = vec![
        Item {cost: 0, damage: 0, armour: 0},
        Item {cost: 25, damage: 1, armour: 0},
        Item {cost: 50, damage: 2, armour: 0},
        Item {cost: 100, damage: 3, armour: 0},
        Item {cost: 20, damage: 0, armour: 1},
        Item {cost: 40, damage: 0, armour: 2},
        Item {cost: 80, damage: 0, armour: 3},
    ];
    let mut combos = Vec::<Item>::new();
    for ring1 in &rings {
        for ring2 in &rings {
            if *ring1 != *ring2 {
                combos.push(add_items(ring1, ring2));
            }
        }
    }
    combos
}

#[derive(Clone)]
struct Fighter {
    health: i32,
    damage: i32,
    armour: i32
}

#[derive(PartialEq)]
struct Item {
    cost: i32,
    damage: i32,
    armour: i32
}

fn add_items(item1: &Item, item2: &Item) -> Item {
    Item {
        cost: item1.cost + item2.cost,
        damage: item1.damage + item2.damage,
        armour: item1.armour + item2.armour
    }
}

fn battle(player: Fighter, boss: Fighter) -> bool {
    let player_damage = max(player.damage - boss.armour, 1) as f32;
    let boss_damage = max(boss.damage - player.armour, 1) as f32;
    let player_turns_to_win = (boss.health as f32 / player_damage).ceil();
    let boss_turns_to_win = (player.health as f32 / boss_damage).ceil();
    player_turns_to_win <= boss_turns_to_win
}