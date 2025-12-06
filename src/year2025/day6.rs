use crate::{data_structs::Grid, input_file::read_lines};

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let grid = parse_input_human(lines);
    let result: u64 =
        grid_to_human_problems(&grid)
            .iter()
            .map(|p| p.solve())
            .sum();
    println!("{}", result);
}

fn parse_input_human(lines: Vec<String>) -> Grid<String> {
    // create a grid from the inputs
    let elements: Vec<String> =
        lines
            .iter()
            .map(|line| {
                line
                    .split_whitespace()
                    .map(|s| s.to_string())
            })
            .flatten()
            .collect();
    Grid::<String>::with_elements(
        lines[0].split_whitespace().count(),
        lines.len(),
        elements)
}

fn grid_to_human_problems(grid: &Grid<String>) -> Vec<Problem> {
    // create problem from each column
    (0..grid.xsize).map(|x| {
        Problem {
            operands: (0..grid.ysize-1).map(|y| {
                grid.get(x,y).parse().unwrap()
            }).collect(),
            operator: match grid.get(x,grid.ysize-1).as_str() {
                "*" => Operator::Product,
                "+" => Operator::Sum,
                _ => panic!("Invalid operator")
            }
        }
    }).collect()
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let grid = parse_input_cephalopod(lines);
    let result: u64 =
        grid_to_cephalopod_problems(&grid)
            .iter()
            .map(|p| p.solve())
            .sum();
    println!("{}", result);
}

fn parse_input_cephalopod(lines: Vec<String>) -> Grid<char> {
    // create a grid from the inputs
    let elements: Vec<char> =
        lines
            .iter()
            .map(|line| {
                line.chars()
            })
            .flatten()
            .collect();
    Grid::<char>::with_elements(
        lines[0].len(),
        lines.len(),
        elements)
}

fn grid_to_cephalopod_problems(grid: &Grid<char>) -> Vec<Problem> {
    let mut problems = Vec::<Problem>::new();
    let mut operands =  Vec::<u64>::new();
    let mut operator = Operator::Sum;
    for x in 0..grid.xsize {
        // convert grid of chars to strings for each column
        let num_str: String =
            (0..grid.ysize-1).map(|y| {
                grid.get(x,y)
            }).collect::<String>().trim().to_string();
        if !num_str.is_empty() {
            operands.push(num_str.trim().parse().unwrap());
            match grid.get(x,grid.ysize-1) {
                '*' => operator = Operator::Product,
                '+' => operator = Operator::Sum,
                ' ' => {},
                c => panic!("Invalid char in operator spot {}", c)
            };
        } else {
            //println!("{:?} {:?}", operator, &operands);
            problems.push(
                Problem {
                    operands,
                    operator: operator.clone()
                });
            operands = Vec::<u64>::new();
        }
    }
    problems.push(
        Problem {
            operands,
            operator: operator.clone()
        });
    problems
}

#[derive(Clone, Debug)]
enum Operator {
    Product,
    Sum
}

struct Problem {
    operands: Vec<u64>,
    operator: Operator
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operator {
            Operator::Product => self.operands.iter().product(),
            Operator::Sum => self.operands.iter().sum(),
        }
    }
}
