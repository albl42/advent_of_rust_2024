use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
//use std::{cmp::Eq, hash::Hash, iter::FromIterator};

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// pub struct AntennaMap<K: Eq + Hash, V>(HashMap<K, Vec<V>>);

// impl<K: Eq + Hash, V> FromIterator<(K, V)> for AntennaMap<K, V> {
//     fn from_iter<I>(tuples: I) -> Self
//     where
//         I: IntoIterator<Item = (K, V)>,
//     {
//         let mut m = HashMap::new();
//         for (k, v) in tuples {
//             m.entry(k).or_insert_with(Vec::new).push(v)
//         }
//         Self(m)
//     }
// }

pub fn parse_map(input: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    input
        .into_iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(y, &cell)| match cell.is_alphanumeric() {
                    true => Some((cell, (x, y))),
                    false => None,
                })
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_insert_with(Vec::new).push(v);
            acc
        })
}

// pub fn antenna_positions(map: &HashMap<char, Vec<(usize, usize)>>) -> HashSet<(usize, usize)> {
//     map.clone().into_values().flatten().collect()
// }

pub fn antenna_pairs(antennas: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    antennas
        .iter()
        .flat_map(|a| {
            antennas.iter().filter_map(move |b| match a != b {
                true => Some((*a, *b)),
                false => None,
            })
        })
        .collect()
}

pub fn try_get_pole(a: i32, b: i32, limit: &(usize, usize)) -> Option<(usize, usize)> {
    match a >= 0 && b >= 0 && a < limit.0 as i32 && b < limit.1 as i32 {
        true => Some((a as usize, b as usize)),
        false => None,
    }
}

pub fn antinode_primary_poles(
    (a, b): &((usize, usize), (usize, usize)),
    limit: &(usize, usize),
) -> Vec<Option<(usize, usize)>> {
    let diff = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);
    let north = try_get_pole(a.0 as i32 - diff.0, a.1 as i32 - diff.1, limit);
    let south = try_get_pole(b.0 as i32 + diff.0, b.1 as i32 + diff.1, limit);
    vec![north, south]
}

pub fn antinode_all_poles(
    (a, b): &((usize, usize), (usize, usize)),
    limit: &(usize, usize),
) -> Vec<Option<(usize, usize)>> {
    let diff = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);

    let mut result = vec![];
    let mut north = try_get_pole(b.0 as i32 - diff.0, b.1 as i32 - diff.1, limit);
    while north.is_some() {
        result.push(north);
        north = try_get_pole(
            north.unwrap().0 as i32 - diff.0,
            north.unwrap().1 as i32 - diff.1,
            limit,
        );
    }
    let mut south = try_get_pole(a.0 as i32 + diff.0, a.1 as i32 + diff.1, limit);
    while south.is_some() {
        result.push(south);
        south = try_get_pole(
            south.unwrap().0 as i32 + diff.0,
            south.unwrap().1 as i32 + diff.1,
            limit,
        );
    }
    result
}

pub fn antinode_primary_positions(
    antennas: &Vec<(usize, usize)>,
    limit: &(usize, usize),
) -> Vec<(usize, usize)> {
    let pairs = antenna_pairs(&antennas);
    pairs
        .iter()
        .flat_map(|x| antinode_primary_poles(x, limit))
        .flatten()
        .collect()
}

pub fn antinode_all_positions(
    antennas: &Vec<(usize, usize)>,
    limit: &(usize, usize),
) -> Vec<(usize, usize)> {
    let pairs = antenna_pairs(&antennas);
    pairs
        .iter()
        .flat_map(|x| antinode_all_poles(x, limit))
        .flatten()
        .collect()
}

pub fn valid_primary_antinode_positions(
    antenna_map: &HashMap<char, Vec<(usize, usize)>>,
    limit: &(usize, usize),
) -> Vec<(usize, usize)> {
    antenna_map
        .values()
        .flat_map(|x| antinode_primary_positions(x, limit))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

pub fn valid_all_antinode_positions(
    antenna_map: &HashMap<char, Vec<(usize, usize)>>,
    limit: &(usize, usize),
) -> Vec<(usize, usize)> {
    antenna_map
        .values()
        .flat_map(|x| antinode_all_positions(x, limit))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

pub fn part_one(input: &str) -> u64 {
    let grid = parse_input(input);
    let map = parse_map(&grid);
    valid_primary_antinode_positions(&map, &(grid.len(), grid[0].len())).len() as u64
}

pub fn part_two(input: &str) -> u64 {
    let grid = parse_input(input);
    let map = parse_map(&grid);
    valid_all_antinode_positions(&map, &(grid.len(), grid[0].len())).len() as u64
}

pub fn exec(input: &Path) -> () {
    let input: String = fs::read_to_string(input).unwrap();

    let result_one = part_one(&input);
    println!("Result part one: {:?}", result_one);

    let result_two = part_two(&input);
    println!("Result part two: {:?}", result_two);
}
