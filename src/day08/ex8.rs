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

// pub fn parse_map(input: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
//     input
//         .iter()
//         .enumerate()
//         .flat_map(|(x, row)| {
//             row.iter()
//                 .enumerate()
//                 .filter_map(move |(y, &cell)| match cell.is_alphanumeric() {
//                     true => Some((cell, (x, y))),
//                     false => None,
//                 })
//         })
//         .fold(HashMap::new(), |mut acc, (k, v)| {
//             acc.entry(k).or_insert_with(Vec::new).push(v);
//             acc
//         })
// }

pub fn parse_map(input: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut map = HashMap::new();

    for (x, row) in input.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if cell.is_alphanumeric() {
                map.entry(cell).or_insert_with(Vec::new).push((x, y));
            }
        }
    }

    map
}

// pub fn antenna_positions(map: &HashMap<char, Vec<(usize, usize)>>) -> HashSet<(usize, usize)> {
//     map.clone().into_values().flatten().collect()
// }

pub fn antenna_pairs(antennas: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    antennas
        .iter()
        .enumerate()
        .flat_map(|(i, a)| antennas.iter().skip(i + 1).map(|b| (*a, *b)))
        .collect()
}

pub fn try_get_pole(
    pos: &(usize, usize),
    step: &(i32, i32),
    limit: &(usize, usize),
) -> Option<(usize, usize)> {
    let x = pos.0 as i32 + step.0;
    let y = pos.1 as i32 + step.1;
    if x >= 0 && y >= 0 && x < limit.0 as i32 && y < limit.1 as i32 {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

pub fn antinode_primary_poles(
    (a, b): &((usize, usize), (usize, usize)),
    limit: (usize, usize),
) -> Vec<Option<(usize, usize)>> {
    let step_down = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);
    let step_up = (-step_down.0, -step_down.1);

    let north = try_get_pole(a, &step_up, &limit);
    let south = try_get_pole(b, &step_down, &limit);

    vec![north, south]
}

pub fn antinode_all_poles(
    (a, b): &((usize, usize), (usize, usize)),
    limit: (usize, usize),
) -> Vec<Option<(usize, usize)>> {
    let step_down = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);
    let step_up = (-step_down.0, -step_down.1);

    let mut result = vec![];

    // Starting at end positions 'b' in order to also include start position 'a'
    let mut north = try_get_pole(b, &step_up, &limit);
    while let Some(n) = north {
        result.push(Some(n));
        north = try_get_pole(&n, &step_up, &limit);
    }

    // Starting at start positions 'a' in order to also include end position 'b'
    let mut south = try_get_pole(a, &step_down, &limit);
    while let Some(s) = south {
        result.push(Some(s));
        south = try_get_pole(&s, &step_down, &limit);
    }

    result
}

pub fn antinode_positions<G>(
    antennas: &Vec<(usize, usize)>,
    gen_func: &mut G,
) -> Vec<(usize, usize)>
where
    G: FnMut(&((usize, usize), (usize, usize))) -> Vec<Option<(usize, usize)>>,
{
    antenna_pairs(&antennas)
        .iter()
        .flat_map(gen_func)
        .flatten()
        .collect()
}

pub fn valid_antinode_positions<G>(
    antenna_map: &HashMap<char, Vec<(usize, usize)>>,
    gen_func: &mut G,
) -> Vec<(usize, usize)>
where
    G: FnMut(&((usize, usize), (usize, usize))) -> Vec<Option<(usize, usize)>>,
{
    antenna_map
        .values()
        .flat_map(|x| antinode_positions(x, gen_func))
        .collect()
}

pub fn unique_elements<T>(vec: Vec<T>) -> Vec<T>
where
    T: std::hash::Hash + Eq,
{
    vec.into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

pub fn part_one(input: &str) -> u64 {
    let grid = parse_input(input);
    let map = parse_map(&grid);

    // Higher-Ranked Trait Bounds (HRTB) and Dynamic Dispatch
    // Explicit &mut dyn for<'a> FnMut
    // explicit higher-ranked trait bound (for<'a>), ensuring that the closure is compatible with any lifetime.
    let mut gen_func: &mut dyn for<'a> FnMut(
        &'a ((usize, usize), (usize, usize)),
    ) -> Vec<Option<(usize, usize)>> =
        &mut |pair| antinode_primary_poles(pair, (grid.len(), grid[0].len()));

    unique_elements(valid_antinode_positions(&map, &mut gen_func)).len() as u64
}

pub fn part_two(input: &str) -> u64 {
    let grid = parse_input(input);
    let map = parse_map(&grid);

    let mut gen_func: &mut dyn for<'a> FnMut(
        &'a ((usize, usize), (usize, usize)),
    ) -> Vec<Option<(usize, usize)>> =
        &mut |pair| antinode_all_poles(pair, (grid.len(), grid[0].len()));

    unique_elements(valid_antinode_positions(&map, &mut gen_func)).len() as u64
}

pub fn exec(input: &Path) -> () {
    let input: String = fs::read_to_string(input).unwrap();

    let result_one = part_one(&input);
    println!("Result part one: {:?}", result_one);

    let result_two = part_two(&input);
    println!("Result part two: {:?}", result_two);
}
