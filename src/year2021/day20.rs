use std::{collections::HashSet, ops::Add, iter::FromIterator};

use crate::{
  input_file::read_lines,
  binary::bin_to_dec,
  data_structs::SignedCoord
};

pub fn part1(input_file_path: &str) {
  let lines = read_lines(input_file_path);
  let (enhancer, mut lit_pixels) = parse_lines(&lines);
  let top_left = SignedCoord{x: -10, y: -10};
  let bottom_right = SignedCoord{x: (lines[2].len()+9) as isize, y: (lines.len()+9) as isize};
  for _ in 0..2 {
    lit_pixels = enhance(lit_pixels, &enhancer, &top_left, &bottom_right);
    print_lit_pixels(&lit_pixels, &top_left, &bottom_right);
  }
  println!("{}", lit_pixels.iter().filter(|p| p.x != top_left.x && p.x != bottom_right.x).count());
}

pub fn part2(input_file_path: &str) {
  let lines = read_lines(input_file_path);
  let (enhancer, mut lit_pixels) = parse_lines(&lines);
  let top_left = SignedCoord{x: -53, y: -53};
  let bottom_right = SignedCoord{x: (lines[2].len()+52) as isize, y: (lines.len()+52) as isize};
  for i in 0..50 {
    lit_pixels = enhance(lit_pixels, &enhancer, &top_left, &bottom_right);
    if i % 2 == 1 {
      lit_pixels = HashSet::from_iter(lit_pixels.into_iter().filter(|p| p.x != top_left.x && p.x != bottom_right.x));
    }
    // print_lit_pixels(&lit_pixels, &top_left, &bottom_right);
  }
  println!("{}", lit_pixels.iter().filter(|p| p.x != top_left.x && p.x != bottom_right.x).count());
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