use std::fs;
use std::path::Path;

pub fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|s: &str| -> u32 { s.parse::<u32>().unwrap() })
        .collect()
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(parse_line)
        .fold(vec![], |mut v: Vec<Vec<u32>>, e: Vec<u32>| {
            v.push(e);
            v
        })
}

pub fn is_monotonic_dec(level: &Vec<u32>) -> bool {
    level
        .into_iter()
        .zip(level.into_iter().skip(1))
        .map(|(l, r)| -> bool { *l > *r })
        .fold(true, |acc, e| acc && e)
}

pub fn is_monotonic_inc(level: &Vec<u32>) -> bool {
    level
        .into_iter()
        .zip(level.into_iter().skip(1))
        .map(|(l, r)| -> bool { *l < *r })
        .fold(true, |acc, e| acc && e)
}

pub fn at_most_three_apart(level: &Vec<u32>) -> bool {
    level
        .into_iter()
        .zip(level.into_iter().skip(1))
        .map(|(l, r)| -> bool { u32::abs_diff(*l, *r) <= 3 })
        .fold(true, |acc, e| acc && e)
}

pub fn part_one(levels: &Vec<Vec<u32>>) -> u32 {
    levels
        .into_iter()
        .map(|l| at_most_three_apart(l) && (is_monotonic_inc(l) || is_monotonic_dec(l)))
        .fold(0, |mut acc, e| {
            if e {
                acc += 1
            };
            acc
        })
}

pub fn generate_subarrays(vec: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut vecs = vec![vec.clone()];

    for index in 0..vec.len() {
        let mut subvec = vec.clone();
        subvec.remove(index);
        vecs.push(subvec)
    }
    vecs
}

pub fn part_two(levels: &Vec<Vec<u32>>) -> u32 {
    levels.into_iter()
    .map(generate_subarrays)
    .map(|levels: Vec<Vec<u32>>| part_one(&levels))
    .fold(0, |mut acc, e| {
        if e > 0 {
            acc += 1
        };
        acc
    })
}

pub fn exec(input: &Path) -> () {
    let message: String = fs::read_to_string(input).unwrap();
    let vecs = parse_input(&message);

    let result_one = part_one(&vecs);
    println!("Result part one: {:?}", result_one);

    let result_two = part_two(&vecs);
    println!("Result part two: {:?}", result_two);
}
