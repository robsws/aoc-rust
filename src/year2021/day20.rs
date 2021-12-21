use std::{collections::HashSet, ops::Add};

use crate::{
  input_file::read_lines,
  binary::bin_to_dec,
  data_structs::SignedCoord
};

pub fn part1(input_file_path: &str) {
  let lines = read_lines(input_file_path);
  let (enhancer, mut lit_pixels) = parse_lines(&lines);
  let mut top_left = SignedCoord{x: -1, y: -1};
  let mut bottom_right = SignedCoord{x: lines[2].len() as isize, y: lines.len() as isize};
  for _ in 0..2 {
    lit_pixels = enhance(lit_pixels, &enhancer, &top_left, &bottom_right);
    print_lit_pixels(&lit_pixels, &top_left, &bottom_right);
    top_left.x -= 5;
    top_left.y -= 5;
    bottom_right.x += 5;
    bottom_right.y += 5;
  }
  println!("{}", lit_pixels.len());
}

pub fn part2(input_file_path: &str) {
}

fn parse_lines(lines: &Vec<String>) -> (Vec<bool>, HashSet<SignedCoord>) {
  let enhancer = lines[0].chars().map(|c| c == '#').collect();
  let mut lit_pixels = HashSet::new();
  for i in 2..lines.len() {
    let y = i as isize - 2;
    for (x, c) in lines[i].chars().enumerate() {
      if c == '#' {
        lit_pixels.insert(SignedCoord{x: x as isize, y});
      }
    }
  }
  (enhancer, lit_pixels)
}

fn enhance(
  lit_pixels: HashSet<SignedCoord>,
  enhancer: &Vec<bool>,
  top_left: &SignedCoord,
  bottom_right: &SignedCoord
) -> HashSet<SignedCoord> {
  let mut new_lit_pixels = HashSet::new();
  for x in top_left.x .. bottom_right.x+1 {
    for y in top_left.y .. bottom_right.y+1 {
      let mut binary_number = Vec::new();
      for y1 in y-1 .. y+2 {
        for x1 in x-1 .. x+2 {
          binary_number.push(lit_pixels.contains(&SignedCoord{x: x1, y: y1}));
        }
      }
      let dec_number = bin_to_dec(&binary_number) as usize;
      if enhancer[dec_number] {
        new_lit_pixels.insert(SignedCoord{x, y});
      }
    }
  }
  new_lit_pixels
}

fn print_lit_pixels(lit_pixels: &HashSet<SignedCoord>, top_left: &SignedCoord, bottom_right: &SignedCoord) {
  for y in top_left.y .. bottom_right.y+1 {
    let mut line = String::new();
    for x in top_left.x .. bottom_right.x+1 {
      if lit_pixels.contains(&SignedCoord{x, y}) {
        line = line.add("#");
      } else {
        line = line.add(".");
      }
    }
    println!("{}", line);
  }
  println!("");
}