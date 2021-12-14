use crate::{input_file::read_lines};

use self::cave_system::CaveSystem;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let system = CaveSystem::from_string_repr(lines);
    let path_count = cave_system::count_paths(&system, false);
    println!("{}", path_count);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let system = CaveSystem::from_string_repr(lines);
    let path_count = cave_system::count_paths(&system, true);
    println!("{}", path_count);
}

mod cave_system {
    use std::collections::HashMap;
    use regex::Regex;

    #[derive(Clone)]
    pub struct CaveSystem {
        caves: HashMap<String, Cave>,
        visited_one_twice: bool
    }

    #[derive(Clone)]
    struct Cave {
        cave_type: CaveType,
        connections: Vec<String>,
        visited: bool
    }

    #[derive(Clone, PartialEq)]
    enum CaveType {
        Large,
        Small
    }

    impl CaveSystem {
        pub fn from_string_repr(lines: Vec<String>) -> CaveSystem {
            let mut caves_by_id = HashMap::<String, Cave>::new();
            for line in lines {
                let cave_ids = line.split('-').collect::<Vec<&str>>();
                // Add caves that haven't been seen yet
                for cave_id in cave_ids.iter() {
                    if !caves_by_id.contains_key(*cave_id) {
                        caves_by_id.insert(cave_id.to_string(), Cave::new(cave_id));
                    }
                }
                // Connect the caves
                let cave_a = caves_by_id.get_mut(cave_ids[0]).unwrap();
                cave_a.connections.push(cave_ids[1].to_string());
                let cave_b = caves_by_id.get_mut(cave_ids[1]).unwrap();
                cave_b.connections.push(cave_ids[0].to_string());
            }
            CaveSystem { caves: caves_by_id, visited_one_twice: false }
        }
    }

    pub fn count_paths(system: &CaveSystem, allow_double: bool) -> u32 {
        if allow_double {
            count_paths_allowing_double_rec(system.clone(), "start")
        } else {
            count_paths_rec(system.clone(), "start")
        }
    }

    fn count_paths_rec(mut system: CaveSystem, from_cave_id: &str) -> u32 {
        if from_cave_id == "end" {
            // Base case: reached the end of the cave system
            1
        } else {
            // Recursive case: visit all connected caves
            // Mark cave as visited
            let current_cave = system.caves.get_mut(from_cave_id).unwrap();
            current_cave.visited = true;
            // Loop through connected caves
            let current_cave = system.caves.get(from_cave_id).unwrap();
            let mut path_count = 0;
            for next_cave_id in current_cave.connections.iter() {
                let next_cave = system.caves.get(next_cave_id).unwrap();
                // Only explore paths that lead to unvisited small caves or
                // large caves (which can be visited multiple times).
                if !next_cave.visited || next_cave.cave_type == CaveType::Large {
                    path_count += count_paths_rec(system.clone(), &next_cave_id);
                }
            }
            path_count
        }
    }

    fn count_paths_allowing_double_rec(mut system: CaveSystem, from_cave_id: &str) -> u32 {
        if from_cave_id == "end" {
            // Base case: reached the end of the cave system
            1
        } else {
            // Recursive case: visit all connected caves
            // Mark cave as visited
            let current_cave = system.caves.get_mut(from_cave_id).unwrap();
            current_cave.visited = true;
            // Loop through connected caves
            let current_cave = system.caves.get(from_cave_id).unwrap();
            let mut path_count = 0;
            for next_cave_id in current_cave.connections.iter() {
                let next_cave = system.caves.get(next_cave_id).unwrap();
                // Only explore paths that lead to:
                // - large caves
                // - small caves that are completely unvisited
                // - small caves that are visited, but only if we haven't already
                //   visited a small cave twice on this path and it's not the start cave.
                if
                    next_cave.cave_type == CaveType::Large ||
                    !next_cave.visited ||
                    (!system.visited_one_twice && next_cave_id != "start")
                {
                    let mut next_system = system.clone();
                    // Make sure we can't visit a small cave twice again on this path.
                    if next_cave.visited && next_cave.cave_type == CaveType::Small {
                        next_system.visited_one_twice = true;
                    }
                    path_count += count_paths_allowing_double_rec(next_system, &next_cave_id);
                }
            }
            path_count
        }
    }

    impl Cave {
        fn new(id: &str) -> Cave {
            let upper_re = Regex::new(r"[A-Z]+").unwrap();
            let t = match id {
                uc if upper_re.is_match(uc) => CaveType::Large,
                _ => CaveType::Small
            };
            Cave {
                cave_type: t,
                connections: Vec::<String>::new(),
                visited: false
            }
        }
    }
}