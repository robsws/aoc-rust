use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use num::pow;
use regex::Regex;

use crate::input_file::read_lines;

const debugging: bool = false;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let machines =
        lines.iter().map(|line| Machine::from_input_string(line)).collect_vec();
    let result: u64 =
        machines
            .iter()
            .map(|machine| {
                find_fewest_button_presses_lights(machine)
            })
            .sum();
    println!("{}", result);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let machines =
        lines.iter().map(|line| Machine::from_input_string(line)).collect_vec();
    let result: u64 =
        machines
            .iter()
            .map(|machine| {
                let mut result_cache = HashMap::<String,u64>::new();
                let answer = find_fewest_button_presses_jolts(machine, &mut result_cache, 0);
                println!("{:?} {}", machine.joltages, answer);
                answer
            })
            .sum();
    println!("{}", result);

}

#[derive(Clone)]
struct Machine {
    // represent indicator array and button wiring as binary
    // works for max array of 32 lights
    indicator_goal: u64,
    buttons: Vec<u64>,
    buttons_nums: Vec<Vec<u64>>,
    joltages: Vec<u64>
}

impl Machine {

    fn from_input_string(repr: &str) -> Machine {
        let ind_re = Regex::new(r"\[([.#]+)\]").unwrap();
        let but_re = Regex::new(r"\(((?:(?:\d+),?)+)\)").unwrap();
        let jol_re = Regex::new(r"\{((?:(?:\d+),?)+)\}").unwrap();

        let (indicator_goal, indicator_len) = match ind_re.captures(repr) {
            Some(caps) => {
                let ind_str = caps.get(1).unwrap().as_str();
                let bool_vec = ind_str.chars().map(|c| {
                    match c {
                        '#' => true,
                        '.' => false,
                        x => panic!("bad char in indicator input '{}'", x)
                    }
                }).collect_vec();
                (bool_vec_to_num(&bool_vec), bool_vec.len() as u8)
            },
            None => {
                panic!("failed to parse indicators: {}", repr);
            }
        };
        let buttons =
            but_re.captures_iter(repr)
                .map(|button_caps| {
                    let but_str = button_caps.get(1).unwrap().as_str();
                    let indexes = parse_comma_delim_nums(but_str);
                    let mut bool_vec = Vec::<bool>::new();
                    for i in 0..indicator_len {
                        if indexes.contains(&(i as u64)) {
                            bool_vec.push(true);
                        } else {
                            bool_vec.push(false);
                        }
                    }
                    bool_vec_to_num(&bool_vec)
                })
                .collect_vec();
        let buttons_nums =
            but_re.captures_iter(repr)
                .map(|button_caps| {
                    let but_str = button_caps.get(1).unwrap().as_str();
                    let indexes = parse_comma_delim_nums(but_str);
                    let mut num_vec = Vec::<u64>::new();
                    for i in 0..indicator_len {
                        if indexes.contains(&(i as u64)) {
                            num_vec.push(1);
                        } else {
                            num_vec.push(0);
                        }
                    }
                    num_vec
                })
                .collect_vec();
        let joltages = match jol_re.captures(repr) {
            Some(caps) => {
                parse_comma_delim_nums(
                    caps.get(1).unwrap().as_str())
            },
            None => {
                panic!("failed to parse indicators: {}", repr);
            }
        };
        Machine {
            indicator_goal,
            buttons,
            buttons_nums,
            joltages,
        }
    }
}

fn find_fewest_button_presses_lights(machine: &Machine) -> u64 {
    // breadth-first search on tree of possible presses
    let mut state_queue = VecDeque::<(u64, u64)>::new();
    state_queue.push_back((0, 0));

    // try all button presses, update machine state
    loop {
        let (current_ind_state, current_presses) = state_queue.pop_front()
            .expect("reached end of bfs queue");
        for button in machine.buttons.iter() {
            // as indicators and buttons are represented as bit arrays,
            // XOR does the job of toggling on/off
            let next_ind_state =
                (current_ind_state ^ button, current_presses + 1);
            if next_ind_state.0 == machine.indicator_goal {
                return next_ind_state.1;
            }
            state_queue.push_back(next_ind_state);
        }
    }
}

fn find_fewest_button_presses_jolts(
    machine: &Machine, result_cache: &mut HashMap<String,u64>, depth: usize
) -> u64 {
    // Had to look up a solution as could see bfs was intractable
    // https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/

    let indent = " ".repeat(depth*2);
    //println!("{}call : {:?}", indent, machine.joltages);
    
    // Look up result in cache
    let key = machine.joltages.iter().map(|n| n.to_string()).join(",");
    match result_cache.get(&key) {
        Some(result) => return *result,
        None => {}
    }

    // Base case - all joltage goals are 0, return 0
    if machine.joltages.iter().all(|j| *j == 0) {
        return 0;
    }

    // First find all patterns of single button presses
    // that achieve the same parity pattern (i.e. lights)
    // as the joltage requirement
    let joltage_parity_goal =
        machine.joltages.iter()
            .map(|joltage| (joltage % 2) as u64).collect::<Vec<u64>>();
    let matching_patterns_and_new_goals =
        machine.buttons_nums.iter()
            .powerset()
            .map(|buttons_pressed| {
                let mut jolts = vec![0; machine.joltages.len()];
                for button in buttons_pressed.iter() {
                    for i in 0..button.len() {
                        jolts[i] += button[i]
                    }
                }
                (buttons_pressed, jolts)
            })
            .filter(|(_, jolts)| {
                let jolt_parity = jolts.iter()
                    .map(|joltage| joltage % 2).collect_vec();
                jolts.iter().zip(machine.joltages.iter()).all(|(j, k)| *j <= *k) && jolt_parity == joltage_parity_goal
            });

    // Recurse having applied each of these patterns
    // Because the remaining joltages are all even after application,
    // the joltages can be halved and the result from the recursion
    // can be multiplied by 2.
    let mut min_presses = u64::max_value();
    for (pattern, jolts_to_sub) in matching_patterns_and_new_goals {
        if debugging { println!("{}press: {:?} for {:?}", indent, pattern, jolts_to_sub); }
        let dbg = jolts_to_sub.clone();
        let new_goal = machine.joltages.iter().zip(jolts_to_sub).map(|(j, k)| (j - k)/2 as u64).collect_vec();
        if debugging { println!("{}{:?} - {:?} / 2 = {:?}", indent, machine.joltages, dbg, new_goal); }
        let next_machine = Machine {
            indicator_goal: machine.indicator_goal,
            buttons: machine.buttons.clone(),
            buttons_nums: machine.buttons_nums.clone(),
            joltages: new_goal
        };
        let subres = find_fewest_button_presses_jolts(&next_machine, result_cache, depth + 1);
        if subres == u64::max_value() {
            if debugging { println!("{}impossible to reach target", indent)}
            continue;
        }
        let presses = pattern.iter().count() as u64 + (2 * subres);
        if debugging { println!("{}presses: {}", indent, presses); }
        if presses < min_presses {
            min_presses = presses;
        }
    }
    //println!("{}min presses {:?}: {}", indent, machine.joltages, min_presses);
    result_cache.insert(key, min_presses);
    min_presses
    
}

fn parse_comma_delim_nums(repr: &str) -> Vec<u64> {
    repr
        .split(',')
        .map(|s| {
        s.parse().unwrap()
    })
        .collect()
}

fn bool_vec_to_num(bool_vec: &Vec<bool>) -> u64 {
    let mut sum = 0;
    for i in 0..bool_vec.len() {
        let power = bool_vec.len()-i;
        sum += (if bool_vec[i] { 1 } else { 0 }) * pow(2, power)
    }
    sum
}
