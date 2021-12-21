use std::{cmp::{min, max}, collections::HashMap};
use lazy_static::lazy_static;

use crate::{
  input_file::read_lines
};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (p1_pos, p2_pos) = parse_lines(&lines);
    let die = (1..101).cycle();
    let (loser_score, rolls) = play_determinist_dice(p1_pos, p2_pos, die);
    println!("{}", loser_score * rolls);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let (p1_pos, p2_pos) = parse_lines(&lines);
    let (p1_wins, p2_wins) = take_dirac_turn(p1_pos, p2_pos, 0, 0, true);
    println!("{}", max(p1_wins, p2_wins));
}

fn parse_lines(lines: &Vec<String>) -> (u8, u8) {
    (
        lines[0].split(' ').last().unwrap().parse().expect("Could not parse starting pos"),
        lines[1].split(' ').last().unwrap().parse().expect("Could not parse starting pos"),
    )
}

fn play_determinist_dice<I>(p1_start_pos: u8, p2_start_pos: u8, mut die: I) -> (u32, u32)
    where I: Iterator<Item=u32>
{
    let mut p1_score = 0u32;
    let mut p2_score = 0u32;
    let mut p1_pos = p1_start_pos as u32;
    let mut p2_pos = p2_start_pos as u32;
    let mut rolls = 0;
    while p1_score < 1000 && p2_score < 1000 {
        let roll_total = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
        rolls += 3;
        p1_pos = (p1_pos + roll_total - 1) % 10 + 1;
        p1_score += p1_pos;
        if p1_score >= 1000 {
            break;
        }
        let roll_total = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
        rolls += 3;
        p2_pos = (p2_pos + roll_total - 1) % 10 + 1;
        p2_score += p2_pos;
    }
    (min(p1_score, p2_score), rolls)
}

lazy_static! {
    static ref ROLL_DISTRIBUTION: HashMap<u8, u8> = HashMap::from([
        (3, 1), //[1,1,1]
        (4, 3), //[2,1,1], [1,2,1], [1,1,2]
        (5, 6), //[1,2,2], [2,1,2], [2,2,1], [3,1,1], [1,3,1], [1,1,3]
        (6, 7), //[1,2,3], [2,3,1], [3,2,1], [3,1,2], [2,1,3], [1,3,2], [2,2,2], 
        (7, 6),
        (8, 3),
        (9, 1),
    ]);        
}

fn take_dirac_turn(
    p1_pos: u8,
    p2_pos: u8,
    p1_score: u8,
    p2_score: u8,
    p1_turn: bool
) -> (u64, u64) {
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for roll in 3..10 {
        if p1_turn {
            let p1_pos = (p1_pos + roll - 1) % 10 + 1;
            let p1_score = p1_score + p1_pos;
            if p1_score >= 21 {
                p1_wins += ROLL_DISTRIBUTION[&roll] as u64;
            } else {
                let next_result = take_dirac_turn(p1_pos, p2_pos, p1_score, p2_score, !p1_turn);
                p1_wins += next_result.0 * ROLL_DISTRIBUTION[&roll] as u64;
                p2_wins += next_result.1 * ROLL_DISTRIBUTION[&roll] as u64;
            }
        } else {
            let p2_pos = (p2_pos + roll - 1) % 10 + 1;
            let p2_score = p2_score + p2_pos;
            if p2_score >= 21 {
                p2_wins += ROLL_DISTRIBUTION[&roll] as u64;
            } else {
                let next_result = take_dirac_turn(p1_pos, p2_pos, p1_score, p2_score, !p1_turn);
                p1_wins += next_result.0 * ROLL_DISTRIBUTION[&roll] as u64;
                p2_wins += next_result.1 * ROLL_DISTRIBUTION[&roll] as u64;
            }
        }
    }
    (p1_wins, p2_wins)
}