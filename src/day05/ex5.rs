use std::fs;
use std::path::Path;

pub fn parse_rule(line: &str) -> Vec<u32> {
    line.split('|')
        .map(|s: &str| -> u32 { s.parse::<u32>().unwrap() })
        .collect()
}

pub fn parse_update(line: &str) -> Vec<u32> {
    line.split(',')
        .map(|s: &str| -> u32 { s.parse::<u32>().unwrap() })
        .collect()
}

pub fn parse_input_rules(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .filter(|line| line.contains('|'))
        .map(parse_rule)
        .map(|rule| (rule[0], rule[1]))
        .collect()
}

pub fn parse_input_updates(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter(|line| line.contains(','))
        .map(parse_update)
        .collect()
}

pub fn is_correct(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for first in 0..update.len() {
        for second in first..update.len() {
            let anti_rule = (update[second], update[first]);
            if rules.contains(&anti_rule) {
                return false;
            }
        }
    }
    return true;
}

pub fn calc_deps(
    val: &u32,
    rules: &Vec<(u32, u32)>,
    update: &Vec<u32>,
    result: &Vec<u32>,
) -> usize {
    rules
        .iter()
        .filter(|(_, b)| b == val)
        .filter(|(a, _)| update.contains(a))
        .filter(|(a, _)| !result.contains(a))
        .count()
}

pub fn fix_ordering(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut result = vec![];
    while result.len() != update.len() {
        let deps: Vec<(u32, usize)> = update
            .iter()
            .map(|&val| (val, calc_deps(&val, rules, update, &result)))
            .filter(|(a, _)| !result.contains(a))
            .collect();

        let mut without_deps = deps
            .iter()
            .filter(|(_, b)| *b == 0)
            .map(|(a, _)| *a)
            .collect::<Vec<u32>>();

        result.append(&mut without_deps);
    }

    result
}

pub fn part_one(input: &str) -> u32 {
    let rules = parse_input_rules(input);
    let updates = parse_input_updates(input);

    updates
        .into_iter()
        .filter(|u| is_correct(u, &rules))
        .fold(0, |mut acc, vec| {
            acc += vec[vec.len() / 2];
            acc
        })
}

pub fn part_two(input: &str) -> u32 {
    let rules = parse_input_rules(input);
    let updates = parse_input_updates(input);

    updates
        .into_iter()
        .filter(|u| !is_correct(u, &rules))
        .map(|u| fix_ordering(&u, &rules))
        .fold(0, |mut acc, vec| {
            acc += vec[vec.len() / 2];
            acc
        })
}

pub fn exec(input: &Path) -> () {
    let input: String = fs::read_to_string(input).unwrap();

    let result_one = part_one(&input);
    println!("Result part one: {:?}", result_one);

    let result_two = part_two(&input);
    println!("Result part two: {:?}", result_two);
}
