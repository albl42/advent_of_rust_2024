use std::collections::HashSet;
use std::fs;
use std::path::Path;

type Position = (i32, i32);
type Direction = (i32, i32);
type Guard = (Position, Direction);

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).fold(
        vec![],
        |mut v: Vec<Vec<char>>, e: Vec<char>| {
            v.push(e);
            v
        },
    )
}

pub fn to_direction(direction: &char) -> Direction {
    match direction {
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        '^' => (-1, 0),
        _ => panic!("Direction {:?} is not valid!", direction),
    }
}

pub fn find_start_move(map: &Vec<Vec<char>>) -> Guard {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] != '#' && map[x][y] != '.' {
                let pos = (to_i32(x), to_i32(y));
                let dir = to_direction(&map[x][y]);
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
    is_inside(map, pos) && map[pos.0 as usize][pos.1 as usize] == '#'
}

pub fn step_ahead(state: &Guard) -> Guard {
    let x = state.0 .0 + state.1 .0;
    let y = state.0 .1 + state.1 .1;
    ((x, y), state.1)
}

pub fn turn_right((p, d): &Guard) -> Guard {
    (*p, (d.1, -d.0))
}

pub fn next_move(map: &Vec<Vec<char>>, state: &Guard) -> Guard {
    let next_state = step_ahead(state);
    if is_blocked(map, &next_state.0) {
        turn_right(state)
    } else {
        next_state
    }
}

pub fn generate_moves(map: &Vec<Vec<char>>, start_state: &Guard) -> Vec<Guard> {
    let mut moves = vec![];
    let mut curr_move = start_state.clone();

    while is_inside(&map, &curr_move.0) && !moves.contains(&curr_move) {
        moves.push(curr_move);
        curr_move = next_move(&map, &curr_move);
    }
    moves
}

pub fn count_unique_pos(moves: Vec<Guard>) -> u32 {
    let unique_pos: HashSet<Position> = moves.into_iter().map(|step: Guard| step.0).collect();
    u32::try_from(unique_pos.iter().count()).unwrap()
}

pub fn is_loop(map: &Vec<Vec<char>>, moves: &Vec<Guard>) -> bool {
    let next_move = next_move(map, moves.last().unwrap());
    is_inside(map, &next_move.0)
}

pub fn add_obstacle(map: &Vec<Vec<char>>, obs_pos: Position) -> Vec<Vec<char>> {
    if is_inside(map, &obs_pos) {
        let mut new_map = map.clone();
        new_map[obs_pos.0 as usize][obs_pos.1 as usize] = '#';
        return new_map;
    }
    return map.clone();
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
    let maps_with_obstacle = moves
        .into_iter()
        .fold(HashSet::new(), |mut acc, original_move| {
            let next_pos = step_ahead(&original_move);
            if next_pos.0 != start_move.0 {
                let new_map = add_obstacle(&map, next_pos.0);
                acc.insert(new_map);
            }
            acc
        });
    maps_with_obstacle
        .into_iter()
        .filter(|new_map| {
            let new_path = generate_moves(&new_map, &start_move);
            is_loop(&new_map, &new_path)
        })
        .count() as u32
}

pub fn exec(input: &Path) -> () {
    let input: String = fs::read_to_string(input).unwrap();

    let result_one = part_one(&input);
    println!("Result part one: {:?}", result_one);

    let result_two = part_two(&input);
    println!("Result part two: {:?}", result_two);
}
