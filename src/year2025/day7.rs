use std::collections::HashMap;

use crate::{data_structs::{Grid}, input_file::read_lines};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let grid = parse_input(lines);
    let mut simulation = TachyonSimulation::new(grid);
    simulation.simulate();
    println!("{}", simulation.split_count);
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let startx = find_start(lines.first().unwrap());
    let grid = parse_input(lines);
    let mut simulation = TachyonSimulation::new(grid);
    let timelines = simulation.simulate_tachyon(startx, 0);
    println!("{}", timelines);
}

fn find_start(first_line: &str) -> usize {
    first_line.char_indices().find(|(_, c)| *c == 'S').unwrap().0
}

fn parse_input(lines: Vec<String>) -> Grid<TachyonCell> {
    // parse all chars into a grid
    Grid::<TachyonCell>::with_elements(
        lines[0].len(),
        lines.len(),
        lines
            .iter()
            .map(|line| {
                line
                    .chars()
                    .map(|c| {
                        match c {
                            'S' => TachyonCell::Start,
                            '.' => TachyonCell::Empty,
                            '^' => TachyonCell::Splitter,
                            x => panic!("Invalid character in input {}", x)
                        }
                    })
            })
            .flatten()
            .collect())
}

#[derive(Clone)]
enum TachyonCell {
    Start,
    Empty,
    Beam,
    Splitter
}

struct TachyonSimulation {
    grid: Grid<TachyonCell>,
    split_count: u32,
    timeline_count_cache: HashMap<(usize, usize), u64>
}

impl TachyonSimulation {

    pub fn new(grid: Grid<TachyonCell>) -> TachyonSimulation {
        TachyonSimulation {
            grid,
            split_count: 0,
            timeline_count_cache: HashMap::<(usize, usize), u64>::new()
        }
    }
    
    pub fn simulate(&mut self) {
        loop {
            let changed = self.simulate_round();
            if !changed {
                break;
            }
        }
    }

    fn simulate_round(&mut self) -> bool {
        // bool return value tells us if something changed
        // during this round
        let mut changed = false;
        for x in 0..self.grid.xsize {
            for y in 0..self.grid.ysize {
                changed = changed || match self.grid.get(x, y) {
                    TachyonCell::Start => self.handle_beam(x, y),
                    TachyonCell::Beam => self.handle_beam(x, y),
                    TachyonCell::Empty => false,
                    TachyonCell::Splitter => self.handle_splitter(x, y),
                }
            } 
        }
        changed
    }

    fn handle_beam(&mut self, x: usize, y: usize) -> bool {
        if y+1 >= self.grid.ysize {
            return false;
        }
        match self.grid.get(x, y+1) {
            TachyonCell::Empty => {
                self.grid.set(x, y+1, TachyonCell::Beam);
                true
            },
            _ => {false}
        }
    }

    fn handle_splitter(&mut self, x: usize, y: usize) -> bool {
        let mut changed = false;
        match self.grid.get(x, y-1) {
            TachyonCell::Beam => {
                if x > 0 {
                    match self.grid.get(x-1, y) {
                        TachyonCell::Empty => {
                            self.grid.set(x-1, y, TachyonCell::Beam);
                            changed = true;
                        },
                        _ => {}
                    }
                }
                if x+1 < self.grid.xsize {
                    match self.grid.get(x+1, y) {
                        TachyonCell::Empty => {
                            self.grid.set(x+1, y, TachyonCell::Beam);
                            changed = true;
                        },
                        _ => {}
                    }

                }
            },
            _ => {}
        }
        if changed {
            self.split_count += 1;
        }
        changed
    }

    pub fn simulate_tachyon(&mut self, startx: usize, starty: usize) -> u64 {
        // recursively tracks a single tachyon through the grid
        // on each split, a new recursive execution is started
        // returns the total number of timelines created
        let x = startx;
        let mut y = starty;
        loop {
            y += 1;
            if y >= self.grid.ysize {
                return 1;
            }
            if let TachyonCell::Splitter = self.grid.get(x, y) {
                return if let Some(splitter_timelines) = self.timeline_count_cache.get(&(x,y)) {
                    *splitter_timelines
                } else {
                    let splitter_timelines = self.simulate_tachyon(x-1, y) + self.simulate_tachyon(x+1, y);
                    self.timeline_count_cache.insert((x,y), splitter_timelines);
                    splitter_timelines
                }
            }
        }
    }
}
