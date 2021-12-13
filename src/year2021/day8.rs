use crate::input_file::read_lines;

use self::wire_mapping::WireMappingTable;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let displays = parse_lines(lines);
    let mut count = 0;
    for display in displays {
        for output_digit in display.1 {
            if [2, 3, 4, 7].contains(&output_digit.len()) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let displays = parse_lines(lines);
    let mut mapping = WireMappingTable::new();
    let mut modified = true;
    let mut total = 0;
    for display in displays {
        // Deduce wire mappings
        while modified {
            for digit in &display.0 {
                modified |= mapping.update(digit)
            }
            for digit in &display.1 {
                modified |= mapping.update(digit)
            }
        }
        // Calculate output number
        let mut output = 0;
        for (i, digit) in display.1.iter().enumerate() {
            let num = mapping.wires_to_digit(digit);
            let exp = display.1.len() - i - 1;
            output += num * (10usize.pow(exp as u32));
        }
        // Add to total
        total += output;
    }
    println!("{}", total);
}

fn parse_lines(lines: Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    lines.iter().map(|line| {
        let parts = line
            .split('|')
            .map(|p| p.trim().split_whitespace().map(|s| s.to_string()).collect())
            .collect::<Vec<Vec<String>>>();
        (parts[0].clone(), parts[1].clone())
    }).collect()
}

mod wire_mapping {

    use std::{collections::{HashMap, HashSet}, iter::FromIterator};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref WIRES: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        static ref DIGIT_SEGMENTS: [Vec<char>; 10] = [
            vec!['a', 'b', 'c', 'e', 'f', 'g'],
            vec!['c', 'f'],
            vec!['a', 'c', 'd', 'e', 'g'],
            vec!['a', 'c', 'd', 'f', 'g'],
            vec!['b', 'c', 'd', 'f'],
            vec!['a', 'b', 'd', 'f', 'g'],
            vec!['a', 'b', 'd', 'e', 'f', 'g'],
            vec!['a', 'c', 'f'],
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            vec!['a', 'b', 'c', 'd', 'f', 'g'],
        ];
    }

    fn all_wires_except(wires: &Vec<char>) -> Vec<char> {
        WIRES.clone().into_iter().filter(|w| !wires.contains(w)).collect()
    }

    pub struct WireMappingTable {
        table: HashMap<char, HashSet<char>>
    }

    impl WireMappingTable {
        pub fn new() -> WireMappingTable {
            let wires = HashSet::from_iter(WIRES.clone().into_iter());
            WireMappingTable {
                table: HashMap::<char, HashSet<char>>::from([
                    ('a', wires.clone()),
                    ('b', wires.clone()),
                    ('c', wires.clone()),
                    ('d', wires.clone()),
                    ('e', wires.clone()),
                    ('f', wires.clone()),
                    ('g', wires.clone()),
                ])
            }
        }

        pub fn wires_to_digit(&self, seven_segment: &str) -> usize {
            let mut mapped_segments = Vec::new();
            for c in seven_segment.chars() {
                let mapped = self.table.get(&c).unwrap();
                if mapped.len() != 1 {
                    panic!("More than one possibility remaining for wire.");
                }
                let mapped = mapped.iter().next().unwrap();
                mapped_segments.push(*mapped);
            }
            mapped_segments.sort();
            for (digit, segments) in DIGIT_SEGMENTS.iter().enumerate() {
                if mapped_segments == *segments {
                    return digit;
                }
            }
            panic!("No digit found that matches the given segments.");
        }

        pub fn update(&mut self, digit: &str) -> bool {
            match digit.len() {
                0 => self.handle_six_digit(digit),
                2 => self.handle_simple_digit(digit),
                3 => self.handle_simple_digit(digit),
                4 => self.handle_simple_digit(digit),
                6 => self.handle_six_digit(digit),
                9 => self.handle_six_digit(digit),
                _ => false
            }
        }

        fn handle_simple_digit(&mut self, digit: &str) -> bool {
            let segments = &DIGIT_SEGMENTS[digit.len()];
            let inputs: Vec<char> = digit.chars().collect();
            let mut modified = false;
            for wire in WIRES.iter() {
                if inputs.contains(wire) {
                    // rule out everything but segments for the inputs appearing in the digit
                    modified |= self.rule_out_for_input(
                        *wire, 
                        &all_wires_except(segments)
                    );
                } else {
                    // rule out segments as outputs for all other inputs
                    modified |= self.rule_out_for_output(
                        *wire, 
                        segments
                    );
                }
            }
            if modified {
                self.check_for_resolved_mappings();
            }
            modified
        }

        fn handle_six_digit(&mut self, digit: &str) -> bool {
            // find the missing input
            let mut missing_wire = '-';
            for wire in WIRES.iter() {
                if !digit.contains(*wire) {
                    missing_wire = *wire;
                    break;
                }
            }
            // check which wire this must map to
            // d, c, and e relate to the segments missing from 0, 6, and 9 respectively
            let mut mapped_wire = None;
            if self.table.get(&missing_wire).unwrap().contains(&'d') {
                match mapped_wire {
                    None => mapped_wire = Some('d'),
                    Some(_) => return false,
                }
            }
            if self.table.get(&missing_wire).unwrap().contains(&'c') {
                match mapped_wire {
                    None => mapped_wire = Some('c'),
                    Some(_) => return false,
                }
            }
            if self.table.get(&missing_wire).unwrap().contains(&'e') {
                match mapped_wire {
                    None => mapped_wire = Some('e'),
                    Some(_) => return false,
                }
            }
            // if digit is ascertained, rule out other wires from being that digit
            match mapped_wire {
                None => return false,
                Some(_) => ()
            }
            let mapped_wire = mapped_wire.unwrap();
            let modified = self.rule_out_for_input(missing_wire, &all_wires_except(&vec![mapped_wire]));
            if modified {
                self.check_for_resolved_mappings();
            }
            modified
        }

        fn check_for_resolved_mappings(&mut self) {
            // Check if any inputs have one possible output
            for input in WIRES.iter() {
                let outputs = self.table.get(&input).unwrap();
                if outputs.len() == 1 {
                    // Rule out all other inputs from having the same output
                    let output = outputs.iter().next().unwrap().to_owned();
                    self.rule_out_for_output(output, &all_wires_except(&vec![*input]));
                }
            }
            // Check if any outputs have one possible input
            for output in WIRES.iter() {
                let inputs_with_this_output = WIRES.iter().filter(
                    |input| {
                        let outputs = self.table.get(input).unwrap();
                        outputs.contains(output)
                    }
                ).collect::<Vec<&char>>();
                if inputs_with_this_output.len() == 1 {
                    // Rule out all other outputs from having the same input
                    self.rule_out_for_input(
                        *inputs_with_this_output[0], 
                        &all_wires_except(&vec![*output])
                    );
                }
            }
        }

        fn rule_out_for_input(&mut self, input: char, outputs: &Vec<char>) -> bool {
            let record = self.table.get_mut(&input).unwrap();
            let mut modified = false;
            for output in outputs {
                modified |= record.remove(&output);
            }
            modified
        }

        fn rule_out_for_output(&mut self, output: char, inputs: &Vec<char>) -> bool {
            let mut modified = false;
            for input in inputs {
                let record = self.table.get_mut(&input).unwrap();
                modified |= record.remove(&output);
            }
            modified
        }
    }
}