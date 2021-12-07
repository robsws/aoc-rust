use crate::input_file::read_all_to_string;

pub fn part1(input_file_path: &str) {
    let input = read_all_to_string(input_file_path);
    let fish = parse_input(input);
    let fish = simulate_fish(fish, 80);
    println!("{}", fish);
}

pub fn part2(input_file_path: &str) {
    let input = read_all_to_string(input_file_path);
    let fish = parse_input(input);
    let fish = simulate_fish(fish, 256);
    println!("{}", fish);
}

fn parse_input(input: String) -> Vec<u8> {
    input
        .split(',')
        .map(
            |s|
            s.parse().expect("Failed to parse input number")
        )
        .collect()
}

// fn simulate_fish(fish: Vec<u8>, days: usize) -> Vec<u8> {
//     let mut fish = fish.clone();
//     for d in 0..days {
//         println!("day {}", d);
//         for i in 0..fish.len() {
//             if fish[i] == 0 {
//                 fish[i] = 6;
//                 fish.push(8);
//             } else {
//                 fish[i] -= 1;
//             }
//         }
//     }
//     fish
// }

fn simulate_fish(fish: Vec<u8>, days: usize) -> u64 {
    let mut fish_of_age = [0u64; 9];
    for f in fish {
        fish_of_age[f as usize] += 1
    }
    for _ in 0..days {
        let births = fish_of_age[0];
        for age in 0..8 {
            fish_of_age[age] = fish_of_age[age + 1];
        }
        fish_of_age[8] = births;
        fish_of_age[6] += births;
    }
    fish_of_age.iter().sum()
}