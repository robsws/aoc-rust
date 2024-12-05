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
                        find_string_applying_offset(&input, "MAS", i, west()), //w
                        find_string_applying_offset(&input, "MAS", i, north_west(grid_width)), //nw
                        find_string_applying_offset(&input, "MAS", i, north(grid_width)), //n
                        find_string_applying_offset(&input, "MAS", i, north_east(grid_width)), //ne
                        find_string_applying_offset(&input, "MAS", i, east()), //e
                        find_string_applying_offset(&input, "MAS", i, south_east(grid_width)), //se
                        find_string_applying_offset(&input, "MAS", i, south(grid_width)), //s
                        find_string_applying_offset(&input, "MAS", i, south_west(grid_width)), //sw
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
    let lines: Vec<String> = lines.iter().map(|l| l.clone() + "|").collect();
    let grid_width = lines[0].len() as i32;
    let input = lines.concat();

    let no_of_x_mases: usize =
        input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c == 'A' {

                    let pos = i as i32;
                    let nw_pos = pos + north_west(grid_width);
                    let ne_pos = pos + north_east(grid_width);
                    let se_pos = pos + south_east(grid_width);
                    let sw_pos = pos + south_west(grid_width);

                    if nw_pos >= 0 && nw_pos < input.len() as i32 &&
                        se_pos >= 0 && se_pos < input.len() as i32 &&
                        ne_pos >= 0 && ne_pos < input.len() as i32 &&
                        sw_pos >= 0 && sw_pos < input.len() as i32 {
                            let nw = ascii_char_at(&input, nw_pos);
                            let se = ascii_char_at(&input, se_pos);
                            let ne = ascii_char_at(&input, ne_pos);
                            let sw = ascii_char_at(&input, sw_pos);
                            if ((nw == 'S' && se == 'M') || (nw == 'M' && se == 'S')) &&
                                ((ne == 'S' && sw == 'M') || (ne == 'M' && sw == 'S')) {
                                1
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                } else {
                    0
                }
            })
            .sum();

    println!("{}", no_of_x_mases);
}

fn west() -> i32 { -1 }
fn north_west(width: i32) -> i32 { -1-width }
fn north(width: i32) -> i32 { -width }
fn north_east(width: i32) -> i32 { 1-width }
fn east() -> i32 { 1 }
fn south_east(width: i32) -> i32 { 1+width }
fn south(width: i32) -> i32 { width }
fn south_west(width: i32) -> i32 { -1+width }

fn ascii_char_at(grid: &str, pos: i32) -> char {
    grid.as_bytes()[pos as usize] as char
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
        ascii_char_at(grid, next_pos) == first_char &&
        find_string_applying_offset(grid, &to_find[1..], next_pos as usize, offset)
}
