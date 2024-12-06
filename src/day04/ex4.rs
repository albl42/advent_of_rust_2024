use std::cmp::{max, min};
use std::fs;
use std::path::Path;

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).fold(
        vec![],
        |mut v: Vec<Vec<char>>, e: Vec<char>| {
            v.push(e);
            v
        },
    )
}

pub fn get_horizontal_lines(grid: &Vec<Vec<char>>) -> Vec<String> {
    let mut lines = vec![];
    for x in 0..grid.len() {
        lines.push(grid[x].iter().collect::<String>());
        lines.push(grid[x].iter().rev().collect::<String>());
    }
    lines
}

pub fn get_vertical_lines(grid: &Vec<Vec<char>>) -> Vec<String> {
    let mut lines = vec![];
    for y in 0..grid.len() {
        let mut characters = vec![];
        for x in 0..grid[0].len() {
            characters.push(grid[x][y])
        }
        lines.push(characters.iter().collect::<String>());
        lines.push(characters.iter().rev().collect::<String>());
    }
    lines
}

pub fn get_diagonal_lines(grid: &Vec<Vec<char>>) -> Vec<String> {
    let mut lines = vec![];

    let rows = grid.len();
    let cols = grid[0].len();
    for diagonal in 1..(rows + cols) {
        let start_col = max(0, diagonal as i32 - rows as i32);
        let count = min(min(diagonal as i32, cols as i32 - start_col), rows as i32);

        let mut characters = vec![];
        for y in 0..count as usize {
            let x = min(rows, diagonal) - y - 1;
            let y = start_col as usize + y as usize;
            characters.push(grid[x][y])
        }
        lines.push(characters.iter().collect::<String>());
        lines.push(characters.iter().rev().collect::<String>());
    }

    for diagonal in 1..(rows + cols) {
        let start_col = max(0, diagonal as i32 - rows as i32);
        let count = min(min(diagonal as i32, cols as i32 - start_col), rows as i32);

        let mut characters = vec![];
        for y in 0..count as usize {
            let x = min(rows, diagonal) - y - 1;
            let y = start_col as usize + y as usize;
            characters.push(grid[rows - x - 1][y])
        }
        lines.push(characters.iter().collect::<String>());
        lines.push(characters.iter().rev().collect::<String>());
    }
    lines
}

pub fn get_lines(grid: Vec<Vec<char>>) -> Vec<String> {
    let mut lines = get_horizontal_lines(&grid);
    let mut vert = get_vertical_lines(&grid);
    let mut diag = get_diagonal_lines(&grid);
    lines.append(&mut vert);
    lines.append(&mut diag);
    lines
}

pub fn count_occurences(list: &Vec<String>) -> u32 {
    let re = fancy_regex::Regex::new(r"XMAS").unwrap();
    list.into_iter().fold(0, |mut sum, haystack| {
        let count = re.find_iter(haystack).count() as u32;
        sum += count;
        sum
    })
}

pub fn check_x_mas(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let has_a = grid[x][y] == 'A';
    let has_fist_mas = (grid[x - 1][y - 1] == 'M' && grid[x + 1][y + 1] == 'S')
        || (grid[x - 1][y - 1] == 'S' && grid[x + 1][y + 1] == 'M');
    let has_second_mas = (grid[x - 1][y + 1] == 'M' && grid[x + 1][y - 1] == 'S')
        || (grid[x - 1][y + 1] == 'S' && grid[x + 1][y - 1] == 'M');

    has_a && has_fist_mas && has_second_mas
}

pub fn find_x_mas(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if check_x_mas(grid, x, y) {
                count += 1;
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> u32 {
    let grid = parse_input(input);
    let list = get_lines(grid);
    count_occurences(&list)
}

pub fn part_two(input: &str) -> u32 {
    let grid = parse_input(input);
    find_x_mas(&grid)
}

pub fn exec(input: &Path) -> () {
    let message: String = fs::read_to_string(input).unwrap();

    let result_one = part_one(&message);
    println!("Result part one: {:?}", result_one);

    let result_two = part_two(&message);
    println!("Result part two: {:?}", result_two);
}
