use std::fs;
use std::path::Path;

pub fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let equation: Vec<&str> = line.split(':').collect();
    let result = equation[0].parse::<u64>().unwrap();
    let operators = equation[1]
        .split_whitespace()
        .map(|s: &str| -> u64 { s.parse::<u64>().unwrap() })
        .collect();
    (result, operators)
}

pub fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.lines().map(parse_line).collect()
}

pub fn add(a: u64, b: u64) -> u64 {
    a + b
}

pub fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

pub fn concat(a: u64, b: u64) -> u64 {
    let result = format!("{a}{b}");
    result.parse::<u64>().unwrap()
}

pub fn is_solvable_one(goal: u64, sum: u64, values: &Vec<u64>) -> bool {
    if values.len() == 0 || sum > goal {
        return goal == sum;
    } else {
        let mut remaining_vals = values.clone();
        let cur_val = remaining_vals.remove(0);
        is_solvable_one(goal, add(sum, cur_val), &remaining_vals)
            || is_solvable_one(goal, multiply(sum, cur_val), &remaining_vals)
    }
}

pub fn has_solution_one(eq: &(u64, Vec<u64>)) -> bool {
    is_solvable_one(eq.0, 0, &eq.1)
}

pub fn is_solvable_two(goal: u64, sum: u64, values: &Vec<u64>) -> bool {
    if values.len() == 0 || sum > goal {
        return goal == sum;
    } else {
        let mut remaining_vals = values.clone();
        let cur_val = remaining_vals.remove(0);
        is_solvable_two(goal, add(sum, cur_val), &remaining_vals)
            || is_solvable_two(goal, multiply(sum, cur_val), &remaining_vals)
            || is_solvable_two(goal, concat(sum, cur_val), &remaining_vals)
    }
}

pub fn has_solution_two(eq: &(u64, Vec<u64>)) -> bool {
    is_solvable_two(eq.0, 0, &eq.1)
}

pub fn part_one(input: &str) -> u64 {
    let equations = parse_input(input);
    equations
        .into_iter()
        .filter(has_solution_one)
        .map(|(a, _)| a)
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

pub fn part_two(input: &str) -> u64 {
    let equations = parse_input(input);
    equations
        .into_iter()
        .filter(has_solution_two)
        .map(|(a, _)| a)
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

pub fn exec(input: &Path) -> () {
    let input: String = fs::read_to_string(input).unwrap();

    let result_one = part_one(&input);
    println!("Result part one: {:?}", result_one);

    let result_two = part_two(&input);
    println!("Result part two: {:?}", result_two);
}
