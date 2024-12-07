use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Instant;

type Position = (i32, i32);
type Direction = (i32, i32);
type Guard = (Position, Direction);

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn parse_direction(direction: &char) -> (i32, i32) {
    match direction {
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        '^' => (-1, 0),
        _ => (0, 0),
    }
}

pub fn find_start_move(map: &Vec<Vec<char>>) -> Guard {
    for (x, row) in map.into_iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if cell != '#' && cell != '.' {
                let pos = (to_i32(x), to_i32(y));
                let dir = parse_direction(&cell);
                return (pos, dir);
            }
        }
    }
    panic!("Input does not have a starting postition!")
}

pub fn to_i32(size: usize) -> i32 {
    i32::try_from(size).unwrap()
}

pub fn is_inside(map: &Vec<Vec<char>>, pos: &Position) -> bool {
    pos.0 >= 0 && pos.0 < to_i32(map.len()) && pos.1 >= 0 && pos.1 < to_i32(map[0].len())
}

pub fn is_blocked(map: &Vec<Vec<char>>, pos: &Position) -> bool {
    map[pos.0 as usize][pos.1 as usize] == '#'
}

pub fn next_position(state: &Guard) -> Position {
    (state.0 .0 + state.1 .0, state.0 .1 + state.1 .1)
}

pub fn turn_right(dir: &Direction) -> Direction {
    (dir.1, -dir.0)
}

pub fn advance_guard(map: &Vec<Vec<char>>, state: &Guard) -> Guard {
    let next_pos = next_position(state);
    if is_inside(map, &next_pos) && is_blocked(map, &next_pos) {
        (state.0, turn_right(&state.1))
    } else {
        (next_pos, state.1)
    }
}

pub fn generate_moves(map: &Vec<Vec<char>>, start_state: &Guard) -> Vec<Guard> {
    // Using a HashSet here reduces the complexity from O(n^2) to O(n)
    // The execution time therefor was reduced from 300 seconds to 17 seconds
    // Since the order of the moves is important we need an additional vector

    let mut moves = HashSet::new();
    let mut result = vec![];
    let mut curr_move = start_state.clone();

    while is_inside(&map, &curr_move.0) && moves.insert(curr_move.clone()) {
        result.push(curr_move.clone());
        curr_move = advance_guard(&map, &curr_move);
    }
    result
}

pub fn count_unique_pos(moves: Vec<Guard>) -> u32 {
    moves
        .into_iter()
        .map(|step: Guard| step.0)
        .collect::<HashSet<_>>()
        .into_iter()
        .count() as u32
}

pub fn make_map_add_obst(map: &Vec<Vec<char>>, obs_pos: Position) -> Vec<Vec<char>> {
    if is_inside(map, &obs_pos) {
        let mut new_map = map.clone();
        new_map[obs_pos.0 as usize][obs_pos.1 as usize] = '#';
        return new_map;
    }
    return map.clone();
}

pub fn detect_loop(map: &Vec<Vec<char>>, start: &Guard) -> bool {
    let mut moves = HashSet::new();
    let mut curr_move = start.clone();

    while is_inside(&map, &curr_move.0) && moves.insert(curr_move.clone()) {
        curr_move = advance_guard(&map, &curr_move);
    }
    is_inside(&map, &curr_move.0)
}

pub fn part_one(input: &str) -> u32 {
    let map = parse_input(input);
    let start_move = find_start_move(&map);
    let moves = generate_moves(&map, &start_move);
    count_unique_pos(moves)
}

pub fn part_two(input: &str) -> u32 {
    let map = parse_input(input);
    let start_move = find_start_move(&map);
    let moves = generate_moves(&map, &start_move);
    moves
        .into_iter()
        .map(|state| make_map_add_obst(&map, state.0))
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|new_map| detect_loop(&new_map, &start_move))
        .count() as u32
}

pub fn exec(input: &Path) -> () {
    let input: String = fs::read_to_string(input).unwrap();

    let result_one = part_one(&input);
    println!("Result part one: {:?}", result_one);

    let start = Instant::now();
    let result_two = part_two(&input);
    let elapsed = start.elapsed();
    println!("Result part two: {:?}", result_two);
    println!("Part two took: {} seconds", elapsed.as_secs_f64());
}
