use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let containers = parse_lines(&lines);
    let containers_len = containers.len();
    let combos = container_combos_for_amount(&containers, 150, containers_len);
    println!("{}", combos);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let containers = parse_lines(&lines);
    let containers_len = containers.len();
    let mut combos = 0;
    // Find the amount of combos for the smallest possible number of containers
    for limit in 1..containers_len {
        combos = container_combos_for_amount(&containers, 150, limit);
        if combos > 0 {
            break;
        }
    }
    println!("{}", combos);
}

fn parse_lines(lines: &Vec<String>) -> Vec<i32> {
    let mut containers = Vec::<i32>::new();
    for line in lines {
        containers.push(line.parse::<i32>().expect("Line does not parse as integer."))
    }
    containers
}

fn container_combos_for_amount(containers: &Vec<i32>, total: i32, container_limit: usize) -> i32 {
    // Base case - total is negative, so we have overshot the target
    // or depth limit has been reached, so stop
    if total < 0 || container_limit == 0 {
        return 0;
    }
    // Base case - total is 0, which means we hit the target exactly
    if total == 0 {
        return 1;
    }
    // Recursive case - loop through containers, and recurse on
    // remaining containers after each one
    let mut combos = 0;
    for i in 0..containers.len() {
        let containers_to_recurse = containers[i+1..].to_vec();
        combos += container_combos_for_amount(
            &containers_to_recurse, 
            total - containers[i], 
            container_limit - 1
        );
    }
    combos
}