use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
  let lines = read_lines(input_file_path);
  let snail_nums = parse_lines(&lines);
  let mut total = snail_nums[0].clone();
  for i in 1..snail_nums.len() {
    total = add_snailfish_numbers(&total, &snail_nums[i]);
  }
  println!("{}", magnitude(&total));
}

pub fn part2(input_file_path: &str) {
  let lines = read_lines(input_file_path);
  let snail_nums = parse_lines(&lines);
  let mut best_magnitude = 0;
  for i in 0..snail_nums.len() {
    for j in 0..snail_nums.len() {
      if i != j {
        let num1 = &snail_nums[i];
        let num2 = &snail_nums[j];
        let total = add_snailfish_numbers(num1, num2);
        let mag = magnitude(&total);
        if mag > best_magnitude {
          best_magnitude = mag
        }
      }
    }
  }
  println!("{}", best_magnitude);
}

fn parse_lines(lines: &Vec<String>) -> Vec<Vec<SnailfishToken>> {
  let mut snail_nums = Vec::<Vec<SnailfishToken>>::new();
  for line in lines {
    let tokens = parse_snailfish_number(line);
    snail_nums.push(tokens);
  }
  snail_nums
}

fn parse_snailfish_number(numstr: &str) -> Vec<SnailfishToken> {
  let mut tokens = Vec::<SnailfishToken>::new();
  let mut nest_level = 0;
  let mut current_number = String::new();
  for c in numstr.chars() {
    // Deal with numbers that can build over multiple chars
    match c {
      '[' | ']' | ',' => {
        if current_number.len() > 0 {
          let value = current_number.parse().unwrap();
          tokens.push(SnailfishToken{value, nest_level});
        }
        current_number = String::new();
      },
      _ => ()
    }
    // Deal with pairs and single digits
    match c {
      '[' => nest_level += 1,
      ']' => nest_level -= 1,
      ',' => continue,
      digit => current_number += &digit.to_string()
    }
  }
  tokens
}

#[derive(Clone)]
struct SnailfishToken {
  value: u32,
  nest_level: i32
}

fn add_snailfish_numbers(num1: &Vec<SnailfishToken>, num2: &Vec<SnailfishToken>) -> Vec<SnailfishToken> {
  let mut number = num1.clone();
  number.extend(num2.clone());
  for mut token in &mut number {
    token.nest_level += 1;
  }
  let mut modified = true;
  while modified {
    modified = false;
    let (exploded, explode_modified) = explode(&number);
    number = exploded;
    modified |= explode_modified;
    if modified {
      continue;
    }
    let (splitted, split_modified) = split(&number);
    modified |= split_modified;
    number = splitted;
  }
  number
}

fn explode(num: &Vec<SnailfishToken>) -> (Vec<SnailfishToken>, bool) {
  let mut modified = false;
  let mut new_num = Vec::<SnailfishToken>::new();
  let mut i = 0;
  while i < num.len() {
    let token = &num[i];
    if !modified && token.nest_level > 4 {
      if i > 0 {
        let last_index = new_num.len()-1;
        new_num[last_index].value += token.value;
      }
      new_num.push(SnailfishToken{value: 0, nest_level: token.nest_level-1});
      if i < num.len() - 2 {
        new_num.push(
          SnailfishToken{
            value: num[i+1].value + num[i+2].value,
            nest_level: num[i+2].nest_level
          }
        );
      }
      modified = true;
      i += 2;
    } else {
      new_num.push(token.clone());
    }
    i += 1;
  }
  (new_num, modified)
}

fn split(num: &Vec<SnailfishToken>) -> (Vec<SnailfishToken>, bool) {
  let mut modified = false;
  let mut new_num = Vec::<SnailfishToken>::new();
  let mut i = 0;
  while i < num.len() {
    let token = &num[i];
    if !modified && token.value > 9 {
      modified = true;
      new_num.push(SnailfishToken{value: num::integer::div_floor(token.value, 2), nest_level: token.nest_level+1});
      new_num.push(SnailfishToken{value: num::integer::div_ceil(token.value, 2), nest_level: token.nest_level+1});
    } else {
      new_num.push(token.clone());
    }
    i += 1;
  }
  (new_num, modified)
}

fn magnitude(values: &Vec<SnailfishToken>) -> u32 {
  let mut reduced = values.clone();
  loop {
    if reduced.len() == 1 {
      break;
    }
    for i in 0..reduced.len()-1 {
      if reduced[i].nest_level == reduced[i+1].nest_level {
        let new_value = 3 * reduced[i].value + 2 * reduced[i+1].value;
        let new_nest_level = reduced[i].nest_level - 1;
        reduced[i] = SnailfishToken{value: new_value, nest_level: new_nest_level};
        reduced.remove(i+1);
        break;
      }
    }
  }
  reduced[0].value
}