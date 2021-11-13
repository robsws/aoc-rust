use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read the entire input file into a string.
pub fn read_all_to_string(
    file_path: &str
) -> String {
    return fs::read_to_string(file_path)
    .expect("Could not read the input file.");
}

/// Read the entire input file into a vector, split
/// by lines.
pub fn read_lines(
    file_path: &str
) -> Vec<String> {
    let file: File = File::open(file_path)
        .expect("Problem opening input file.");
    let reader = BufReader::new(file);
    let mut lines = Vec::<String>::new();
    for line in reader.lines() {
        let line = line.expect("Issue reading line from input file.");
        lines.push(line);
    }
    return lines;
}