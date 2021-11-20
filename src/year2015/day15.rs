use num;
use regex::Regex;
use crate::input_file::read_lines;
use lazy_static::lazy_static;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let ingredients= parse_lines(&lines);
    let best_score = best_cookie_score(&ingredients, None, None);
    println!("{}", best_score);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let ingredients= parse_lines(&lines);
    let best_score = best_cookie_score(&ingredients, None, Some(500));
    println!("{}", best_score);
}

lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(
            r"^\w+: capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavour>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)$"
        ).unwrap();
}

fn parse_lines(lines: &Vec<String>) -> Vec<Ingredient> {
    let mut ingredients = Vec::<Ingredient>::new();
    for line in lines {
        let caps = LINE_RE.captures(&line)
            .expect(&format!("Input line did not match expected pattern. {}", line));
        let capacity = caps.name("capacity").unwrap().as_str();
        let capacity = capacity.parse::<i32>().expect("Invalid capacity in line.");
        let durability = caps.name("durability").unwrap().as_str();
        let durability = durability.parse::<i32>().expect("Invalid durability in line.");
        let flavour = caps.name("flavour").unwrap().as_str();
        let flavour = flavour.parse::<i32>().expect("Invalid flavour in line.");
        let texture = caps.name("texture").unwrap().as_str();
        let texture = texture.parse::<i32>().expect("Invalid texture in line.");
        let calories = caps.name("calories").unwrap().as_str();
        let calories = calories.parse::<i32>().expect("Invalid calories in line.");
        let ingredient = Ingredient {capacity, durability, flavour, texture, calories};
        ingredients.push(ingredient);
    }
    ingredients
}

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    calories: i32,
}

fn best_cookie_score(
    ingredients: &Vec<Ingredient>,
    amounts: Option<Vec<i32>>,
    total_calories: Option<i32>
) -> i32 {
    // Initialise the amounts if this is the top level call
    let mut amounts = match amounts {
        None => vec![-1;ingredients.len()],
        Some(v) => v
    };
    // Figure out what index of the amounts to modify
    // and how many spoons are remaining to allocate
    let mut index = 0;
    let mut spoons_left = 100;
    for i in 0..amounts.len() {
        if amounts[i] == -1 {
            index = i;
            break;
        } else {
            spoons_left -= amounts[i];
        }
    }
    // Base case - only one ingredient left to allocate
    // spoons to. All remaining spoons should be allocated.
    if index == amounts.len() - 1 {
        amounts[index] = spoons_left;
        return cookie_score(ingredients, &amounts, total_calories);
    }
    // Recursive case - iterate through possible
    // amounts of spoons at this index and recurse
    // on the remaining ingredients
    let mut best_score = i32::MIN;
    for i in 0..spoons_left+1 {
        amounts[index] = i;
        let best_sub_score = best_cookie_score(ingredients, Some(amounts.clone()), total_calories);
        if best_sub_score > best_score {
            best_score = best_sub_score;
        }
    }
    best_score
}

fn cookie_score(ingredients: &Vec<Ingredient>, amounts: &Vec<i32>, total_calories: Option<i32>) -> i32 {
    let mut capacity_total = 0;
    let mut durability_total = 0;
    let mut flavour_total = 0;
    let mut texture_total = 0;
    let mut calorie_total = 0;
    for i in 0..ingredients.len() {
        capacity_total += ingredients[i].capacity * amounts[i];
        durability_total += ingredients[i].durability * amounts[i];
        flavour_total += ingredients[i].flavour * amounts[i];
        texture_total += ingredients[i].texture * amounts[i];
        calorie_total += ingredients[i].calories * amounts[i];
    }
    capacity_total = num::clamp::<i32>(capacity_total, 0, i32::MAX);
    durability_total = num::clamp::<i32>(durability_total, 0, i32::MAX);
    flavour_total = num::clamp::<i32>(flavour_total, 0, i32::MAX);
    texture_total = num::clamp::<i32>(texture_total, 0, i32::MAX);
    // Calculate the score 
    let score = capacity_total * durability_total * flavour_total * texture_total;
    // If we're considering calories, only give the real score if
    // the calorie total matches the target.
    match total_calories {
        None => score,
        Some(c) => if calorie_total == c { score } else { i32::MIN }
    }
}