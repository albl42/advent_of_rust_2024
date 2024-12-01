use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::path::Path;

pub fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .map(|ints: Vec<u32>| -> (u32, u32) { (ints[0], ints[1]) })
        .unzip()
        // .fold((Vec::<u32>::new(), Vec::<u32>::new()), |(l, r), e: Vec<u32>| { l.push(e[0]); r.push(e[1]); (l, r) })
        //.fold(
        //    Vec::<Vec<u32>>::from([Vec::<u32>::new(), Vec::<u32>::new()]),
        //    |v: Vec<Vec<u32>>, e: Vec<u32>| {
        //        Vec::<Vec<u32>>::from([
        //            v[0].clone().into_iter().chain([e[0]]).collect(),
        //            v[1].clone().into_iter().chain([e[1]]).collect(),
        //        ])
        //    },
        //)
        //.iter_mut()
        //.map(|v: &mut Vec<u32>| { v.sort(); v})
        //.collect()
}

pub fn part1(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    // TODO: We don't want to dereference
    zip(left, right).map(|(l, r)| u32::abs_diff(*l, *r)).sum()
}

pub fn part2(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    let mut map: HashMap<u32, u32> = HashMap::new();

    for element in right {
        *map.entry(*element).or_default() += 1;
    }

    //println!("{:?}", map);

    left.iter().map(|v| v * *map.entry(*v).or_default()).sum()
}

pub fn list_distance(input: &Path) -> () {
    let message: String = fs::read_to_string(input).unwrap();
    let (mut left, mut right) = parse_input(&message);

    left.sort();
    right.sort();

    println!("Part 1: {:?}", part1(&left, &right));
    println!("Part 2: {:?}", part2(&left, &right));
}
