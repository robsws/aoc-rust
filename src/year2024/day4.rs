use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let lines: Vec<String> = lines.iter().map(|l| l.clone() + "|").collect();
    let grid_width = lines[0].len() as i32;
    let input = lines.concat();

    let no_of_xmases: usize =
        input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c == 'X' {
                    [
                        find_string_applying_offset(&input, "MAS", i, -1), //w
                        find_string_applying_offset(&input, "MAS", i, -1 - grid_width), //nw
                        find_string_applying_offset(&input, "MAS", i, -grid_width), //n
                        find_string_applying_offset(&input, "MAS", i, 1 - grid_width), //ne
                        find_string_applying_offset(&input, "MAS", i, 1), //e
                        find_string_applying_offset(&input, "MAS", i, 1 + grid_width), //se
                        find_string_applying_offset(&input, "MAS", i, 0 + grid_width), //s
                        find_string_applying_offset(&input, "MAS", i, -1 + grid_width), //sw
                    ]
                        .iter()
                        .filter(|result| **result)
                        .count()
                } else {
                    0
                }
            })
            .sum();
    
    println!("{}", no_of_xmases);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
}

fn find_string_applying_offset(grid: &str, to_find: &str, pos: usize, offset: i32) -> bool {
    // check that if we apply the given offset repeatedly, we find
    // the string given.

    if to_find.len() == 0 {
        return true;
    }
    
    let first_char = to_find.chars().next().unwrap();
    let next_pos = pos as i32 + offset;

    next_pos >= 0 &&
        next_pos < grid.len() as i32 &&
        // assume ascii for speed
        grid.as_bytes()[next_pos as usize] as char == first_char &&
        find_string_applying_offset(grid, &to_find[1..], next_pos as usize, offset)
}
