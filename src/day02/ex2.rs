/* Learnings and Takeaways:

-> Checking Conditions: 'all' can sometimes be used instead of 'map' and 'fold'
-> Adjacent Comparison: 'window' generates overlapping slices of a certain size
-> Preallocate Vec Capacity: 'Vec::with_capacity'
*/

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
    level.windows(2).all(|w| -> bool { w[0] > w[1] })
}

pub fn is_monotonic_inc(level: &Vec<u32>) -> bool {
    level.windows(2).all(|w| -> bool { w[0] < w[1] })
}

pub fn at_most_three_apart(level: &Vec<u32>) -> bool {
    level
        .windows(2)
        .all(|w| -> bool { u32::abs_diff(w[0], w[1]) <= 3 })
}

pub fn check_level(level: &Vec<u32>) -> bool {
    at_most_three_apart(&level) && (is_monotonic_inc(&level) || is_monotonic_dec(&level))
}

pub fn part_one(levels: &Vec<Vec<u32>>) -> u32 {
    levels.into_iter().filter(|l| check_level(&l)).count() as u32
}

// pub fn generate_subvecs_iterative(vec: &Vec<u32>) -> Vec<Vec<u32>> {
//     let mut vecs = Vec::with_capacity(vec.len() + 1);
//     vecs.push(vec.to_vec());
//     for index in 0..vec.len() {
//         let mut subvec = vec.clone();
//         subvec.remove(index);
//         vecs.push(subvec)
//     }
//     vecs
// }

pub fn generate_subvecs_functional(vec: &Vec<u32>) -> Vec<Vec<u32>> {
    (0..vec.len())
        .map(|i| {
            let mut subvec = vec.to_vec();
            subvec.remove(i);
            subvec
        })
        .chain(std::iter::once(vec.to_vec()))
        .collect()
}

pub fn part_two(levels: &Vec<Vec<u32>>) -> u32 {
    levels
        .into_iter()
        .map(generate_subvecs_functional)
        .filter(|subvec| subvec.iter().any(check_level))
        .count() as u32
}

pub fn exec(input: &Path) -> () {
    let message: String = fs::read_to_string(input).unwrap();
    let vecs = parse_input(&message);

    let result_one = part_one(&vecs);
    println!("Result part one: {:?}", result_one);

    let result_two = part_two(&vecs);
    println!("Result part two: {:?}", result_two);
}
