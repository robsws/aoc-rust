use std::cmp::{min, max};

use regex::Regex;

use crate::{input_file::read_all_to_string, data_structs::SignedCoord};

pub fn part1(input_file_path: &str) {
  let input = read_all_to_string(input_file_path);
  let goal = parse_input(&input);
  let peak = get_highest_trajectory(&goal);
  println!("{}", peak);
}

pub fn part2(input_file_path: &str) {
  let input = read_all_to_string(input_file_path);
  let goal = parse_input(&input);
  let trajs = get_possible_trajectories(&goal);
  let amount = trajs.len();
  // for traj in trajs.into_iter().sorted() {
  //   println!("{:?}", traj);
  // }
  println!("{}", amount);
}

fn parse_input(input: &str) -> Region {
  let regex = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
  let values: Vec<isize> = regex.captures(input).unwrap().iter().skip(1).map(|s|
    s.unwrap().as_str().parse().expect("Could not parse input coordinates.")
  ).collect();
  Region {
    top_left: SignedCoord {
      x: min(values[0], values[1]),
      y: max(values[2], values[3]),
    },
    bottom_right: SignedCoord {
      x: max(values[0], values[1]),
      y: min(values[2], values[3]),
    }
  }
}

struct Region {
  top_left: SignedCoord,
  bottom_right: SignedCoord
}

fn get_highest_trajectory(goal: &Region) -> isize {
  // loop through y velocities and simulate
  let mut overall_highest = isize::MIN;
  for initial_dy in 0..1000 {
    let mut y = 0;
    let mut dy = initial_dy;
    let mut peak = 0;
    let mut touches_goal = false;
    // Simulate flight on x axis and see whether the probe touches the goal region
    while y >= goal.bottom_right.y {
      if y > peak {
        peak = y
      }
      if y <= goal.top_left.y {
        touches_goal = true;
      }
      y += dy;
      dy -= 1;
    }
    if touches_goal {
      overall_highest = peak;
    }
  }
  overall_highest
}

fn get_possible_trajectories(goal: &Region) -> Vec<SignedCoord> {
  let possible_dxs = get_possible_dxs(goal);
  // loop through y velocities and simulate
  let mut possible_trajs = Vec::<SignedCoord>::new();
  for initial_dy in -500..500 {
    for initial_dx in &possible_dxs {
      let mut x = 0;
      let mut y = 0;
      let mut dx = *initial_dx;
      let mut dy = initial_dy;
      let mut touches_goal = false;
      // Simulate flight and see whether the probe touches the goal region
      while !touches_goal && y >= goal.bottom_right.y {
        if y <= goal.top_left.y && x >= goal.top_left.x && x <= goal.bottom_right.x {
          touches_goal = true;
        }
        x += dx;
        y += dy;
        dy -= 1;
        dx = if dx < 0 {
          min(0, dx + 1)
        } else if dx > 0 {
          max(0, dx - 1)
        } else {
          0
        }
      }
      if touches_goal {
        possible_trajs.push(SignedCoord{x: *initial_dx, y: initial_dy});
      }
    }
  }
  possible_trajs
}

fn get_possible_dxs(goal: &Region) -> Vec<isize> {
  // Get all possible x values for the initial trajectory
  let mut possible_dxs = Vec::<isize>::new();
  if goal.top_left.x > 0 {
    // Fire probe to the right
    for initial_dx in 0..goal.bottom_right.x + 1 {
      let mut x = 0;
      let mut dx = initial_dx;
      let mut touches_goal = false;
      // Simulate flight on x axis and see whether the probe touches the goal region
      while !touches_goal && x <= goal.bottom_right.x && dx >= 0 {
        if x >= goal.top_left.x {
          touches_goal = true;
        }
        x += dx;
        dx -= 1;
      }
      if touches_goal {
        possible_dxs.push(initial_dx);
      }
    }
  }
  else if goal.bottom_right.x < 0 {
    // Fire probe to the left
    for initial_dx in 0..goal.top_left.x+1 {
      let mut x = 0;
      let mut dx = initial_dx;
      let mut touches_goal = false;
      // Simulate flight on x axis and see whether the probe touches the goal region
      while !touches_goal && x >= goal.top_left.x && dx <= 0 {
        if x <= goal.bottom_right.x {
          touches_goal = true;
        }
        x += dx;
        dx += 1;
      }
      if touches_goal {
        possible_dxs.push(initial_dx);
      }
    }
  }
  else {
    panic!("Did not implement for case where box is directly above or below");
  }
  possible_dxs
}