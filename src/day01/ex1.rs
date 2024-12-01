/* Learnings and Takeaways:

-> Sorting: 'unstable_sort' is typically faster than 'sort', except when the list is partially sorted
-> Side Effects vs Transformations: use 'for_each' for side effects (like sorting in place) instead of 'map'
-> Clone vs in place: 'push' modifies the vector in place (this avoids the use of 'clone' from before)
-> Ownership vs Borrowing:  'iter' = borrowed + immutable | 'into_iter' = owned + immutable | 'iter_mut' = borrowed + mutable
-> Accessing HashMap: 'entry' is used for in place modifications while 'get' returns Option<&T>
-> Option Unwrap: 'unwrap_or(&0) is valid for returning a reference to the value zero
-> Itertools: 'collect_tuple' can be used for direct destructure of the result
-> Fold Accumulater Modification: passing the accumulater as mutable avoid the creation of intermediate objects
-> Error Handling: consider using 'unwrap_or_else' instead of 'unwrap'
*/

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|s: &str| -> u32 { s.parse::<u32>().unwrap() })
        .collect()
}

pub fn parse_input_tuple(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(parse_line)
        .map(|ints: Vec<u32>| -> (u32, u32) { (ints[0], ints[1]) })
        .unzip()
}

pub fn parse_input_vec(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(parse_line).fold(
        vec![Vec::<u32>::new(), Vec::<u32>::new()],
        |mut v: Vec<Vec<u32>>, e: Vec<u32>| {
            v[0].push(e[0]);
            v[1].push(e[1]);
            v
        },
    )
}

pub fn part1(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| u32::abs_diff(*l, *r))
        .sum()
}

pub fn part2(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    let map = right.iter().fold(HashMap::new(), |mut m, x| {
        *m.entry(x).or_insert(0) += 1;
        m
    });

    left.iter().map(|x| x * map.get(x).unwrap_or(&0)).sum()
}

pub fn exec(input: &Path) -> () {
    let message: String = fs::read_to_string(input).unwrap();

    ///////////////////////////////////////////////////////////////////////////////////
    // Tuples approach
    ///////////////////////////////////////////////////////////////////////////////////

    let (mut left, mut right) = parse_input_tuple(&message);

    left.sort_unstable();
    right.sort_unstable();

    println!("Tuples approach:");
    println!("  Part 1: {:?}", part1(&left, &right));
    println!("  Part 2: {:?}", part2(&left, &right));

    ///////////////////////////////////////////////////////////////////////////////////
    // Vector approach
    ///////////////////////////////////////////////////////////////////////////////////

    let mut vecs = parse_input_vec(&message);

    vecs.iter_mut().for_each(|v| v.sort_unstable());

    let (left, right) = vecs.iter().collect_tuple().unwrap();

    println!("Vector approach:");
    println!("  Part 1: {:?}", part1(&left, &right));
    println!("  Part 2: {:?}", part2(&left, &right));
}
