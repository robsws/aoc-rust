use regex::Regex;
use crate::input_file::read_lines;
use lazy_static::lazy_static;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let reindeer= parse_lines(&lines);
    let furthest_distance = simulate_sprint_race(reindeer);
    println!("{}", furthest_distance);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let reindeer= parse_lines(&lines);
    let winning_score = simulate_points_race(reindeer);
    println!("{}", winning_score);
}

lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(
            r"^\w+ can fly (?P<fly_speed>\d+) km/s for (?P<fly_duration>\d+) seconds, but then must rest for (?P<rest_duration>\d+) seconds.$"
        ).unwrap();
}

fn parse_lines(lines: &Vec<String>) -> Vec<Reindeer> {
    let mut reindeer = Vec::<Reindeer>::new();
    for line in lines {
        let caps = LINE_RE.captures(&line)
            .expect(&format!("Input line did not match expected pattern. {}", line));
        let fly_speed = caps.name("fly_speed").unwrap().as_str();
        let fly_speed = fly_speed.parse::<u32>().expect("Invalid fly speed in line.");
        let fly_duration = caps.name("fly_duration").unwrap().as_str();
        let fly_duration = fly_duration.parse::<u32>().expect("Invalid fly duration in line.");
        let rest_duration = caps.name("rest_duration").unwrap().as_str();
        let rest_duration = rest_duration.parse::<u32>().expect("Invalid rest duration in line.");
        let new_reindeer = Reindeer {
            fly_speed,
            fly_duration,
            rest_duration,
            time_until_state_change: fly_duration,
            flying: true,
            position_km: 0,
            points: 0
        };
        reindeer.push(new_reindeer);
    }
    reindeer
}

fn simulate_sprint_race(reindeer: Vec<Reindeer>) -> u32 {
    let mut reindeer = reindeer;
    for _ in 0..2503 {
        for r in &mut reindeer {
            r.simulate_one_second();
        }
    }
    leading_distance(&reindeer)
}

fn leading_distance(reindeer: &Vec<Reindeer>) -> u32 {
    let mut furthest_distance = 0;
    for r in reindeer {
        if r.position_km > furthest_distance {
            furthest_distance = r.position_km;
        }
    }
    furthest_distance
}

fn simulate_points_race(reindeer: Vec<Reindeer>) -> u32 {
    let mut reindeer = reindeer;
    for _ in 0..2503 {
        for r in &mut reindeer {
            r.simulate_one_second();
        }
        award_points(&mut reindeer);
    }
    let mut winning_score = 0;
    for r in &reindeer {
        if r.points > winning_score {
            winning_score = r.points;
        }
    }
    winning_score
}

fn award_points(reindeer: &mut Vec<Reindeer>) {
    let lead_distance = leading_distance(reindeer);
    for r in reindeer {
        if r.position_km == lead_distance {
            r.points += 1;
        }
    }
}

struct Reindeer {
    fly_speed: u32,
    fly_duration: u32,
    rest_duration: u32,
    flying: bool,
    time_until_state_change: u32,
    position_km: u32,
    points: u32
}

impl Reindeer {
    pub fn simulate_one_second(&mut self) {
        if self.time_until_state_change == 0 {
            self.flying = !self.flying;
            if self.flying {
                self.time_until_state_change = self.fly_duration;
            } else {
                self.time_until_state_change = self.rest_duration;
            }
        }
        if self.flying {
            self.position_km += self.fly_speed;
        }
        self.time_until_state_change -= 1;
    }
}