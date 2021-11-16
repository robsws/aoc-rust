use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let strings = read_lines(input_file_path);
    let mut total = 0;
    for s in strings {
        total += s.len() - in_memory_len(&s);
    }
    println!("{}", total);
}

pub fn part2(input_file_path: &str) {
    let strings = read_lines(input_file_path);
    let mut total = 0;
    for s in strings {
        total += encoded_len(&s) - s.len();
    }
    println!("{}", total);
}

fn in_memory_len(sequence: &str) -> usize {
    let mut total: usize = 0;
    let chars = sequence.chars().collect::<Vec<char>>();
    let mut i = 1;
    while i < chars.len() - 1 {
        match chars[i] {
            '\\' => {
                match chars[i+1] {
                    '\\' => i += 2,
                    '"' => i += 2,
                    'x' => i += 4,
                    _ => panic!("Unsupported escape sequence.")
                }
            },
            _ => i += 1
        };
        total += 1;
    }
    return total;
}

fn encoded_len(sequence: &str) -> usize {
    // Total starts on 2 - Extra speech marks around the side
    let mut total: usize = 2;
    let chars = sequence.chars().collect::<Vec<char>>();
    for c in chars {
        match c {
            '"' => total += 2,
            '\\' => total += 2,
            _ => total += 1
        };
    }
    return total;
}