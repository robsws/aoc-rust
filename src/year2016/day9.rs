use crate::input_file::read_lines;
use lazy_static::lazy_static;

use regex::Regex;

// Assumptions:
// - all characters in input are ascii. This is required for byte length operations to work
// - no markers refer to lookahead data that is past the EOF.

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    for line in lines {
        let decompressed = decompress_v1(&line);
        println!("{}", decompressed.len())
    }
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    for line in lines {
        let len = get_decompressed_length_v2(&line);
        println!("{}", len);
    }
}

lazy_static! {
    static ref MARKER_REGEX: Regex = Regex::new(r"^\((?P<lookahead>\d+)x(?P<repeat>\d+)\)|(?P<char>\w)").unwrap();
}

fn decompress_v1(line: &str) -> String {
    let mut decompressed = String::new();
    let mut i = 0usize;
    while i < line.len() {
        let remaining_line_slice = &line[i..];
        match MARKER_REGEX.captures(remaining_line_slice) {
            Some(captures) => {
                // Extract the captures from the regex.
                let full_match: &str = captures.get(0).map_or("", |m| m.as_str());
                let lookahead = captures.name("lookahead").map_or("none", |m| m.as_str());
                let repeat = captures.name("repeat").map_or("none", |m| m.as_str());
                let character = captures.name("char").map_or("none", |m| m.as_str());

                // Process the captured information and decompress if needed.
                if !lookahead.eq("none") {
                    // We have found a marker. Consume the 'data section' of the marker,
                    // decompress it, and add it to the final result.
                    let lookahead: usize = lookahead
                        .parse()
                        .expect("Lookahead value in marker could not be parsed to uint");
                    let repeat: usize = repeat
                        .parse()
                        .expect("Repeat value in marker could not be parsed to uint.");
                    let marker_len = full_match.len();
                    let data_section = &remaining_line_slice[marker_len..marker_len + lookahead];
                    let decompressed_slice = data_section.repeat(repeat);
                    decompressed.push_str(&decompressed_slice);
                    i = i + marker_len + data_section.len();
                } else {
                    // We have found a bog-standard character that isn't compressed.
                    // Add it to the full string as-is.
                    if character.eq("none") {
                        panic!("No marker or character was parsed in regex: {}", full_match);
                    }
                    let character: char = character
                        .parse()
                        .expect("Parsed character value could not be parsed to character.");
                    decompressed.push(character);
                    i = i + 1;
                }
            }
            None => {
                panic!(
                    "Output near {}... was not matched by regex.",
                    &remaining_line_slice[..10]
                );
            }
        }
    }
    decompressed
}

fn get_decompressed_length_v2(compressed: &str) -> usize {
    let mut decompressed_length = 0usize;
    let mut i = 0usize;
    while i < compressed.len() {
        let remaining_line_slice = &compressed[i..];
        match MARKER_REGEX.captures(remaining_line_slice) {
            Some(captures) => {
                // Extract the captures from the regex.
                let full_match: &str = captures.get(0).map_or("", |m| m.as_str());
                let lookahead = captures.name("lookahead").map_or("none", |m| m.as_str());
                let repeat = captures.name("repeat").map_or("none", |m| m.as_str());

                // Process the captured information and decompress if needed.
                if !lookahead.eq("none") {
                    // We have found a marker. Consume the 'data section' of the marker,
                    // decompress it, and add it to the final result.
                    let lookahead: usize = lookahead
                        .parse()
                        .expect("Lookahead value in marker could not be parsed to uint");
                    let repeat: usize = repeat
                        .parse()
                        .expect("Repeat value in marker could not be parsed to uint.");
                    let marker_len = full_match.len();
                    let data_section = &remaining_line_slice[marker_len..marker_len + lookahead];
                    // Recursive call on the data section, as in V2 decompression we also consider markers in there.
                    // Whatever the decompressed length of that is, multiply it by the number of times we have to repeat.
                    decompressed_length += get_decompressed_length_v2(data_section) * repeat;
                    i = i + marker_len + data_section.len();
                } else {
                    // We have found a bog-standard character that isn't compressed.
                    // The length of a single character is always 1.
                    decompressed_length += 1;
                    i = i + 1;
                }
            }
            None => {
                panic!(
                    "Output near {}... was not matched by regex.",
                    &remaining_line_slice[..10]
                );
            }
        }
    }
    decompressed_length
}
