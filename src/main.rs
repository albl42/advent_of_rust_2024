use std::{
    env, io,
    path::{Path, PathBuf},
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn input_file_path(day_number: u16, file_name: &str) -> PathBuf {
    Path::new("src")
        .join(format!("day{:02}", day_number))
        .join(file_name)
}

fn interactive_mode() {
    println!("Choose a number between 1 and 24:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<u8>() {
        Ok(num) if num >= 1 && num <= 24 => {
            exec_day(num);
        }
        _ => {
            println!("Invalid input. Please enter a number between 1 and 24.");
        }
    }
}

fn exec_day(day: u8) {
    match day {
        1 => day01::ex1::exec(&input_file_path(1, "input.txt")),
        2 => day02::ex2::exec(&input_file_path(2, "input.txt")),
        3 => day03::ex3::exec(&input_file_path(3, "input.txt")),
        4 => day04::ex4::exec(&input_file_path(4, "input.txt")),
        5 => day05::ex5::exec(&input_file_path(5, "input.txt")),
        6 => day06::ex6::exec(&input_file_path(6, "input.txt")),
        7 => day07::ex7::exec(&input_file_path(7, "input.txt")),
        8 => day08::ex8::exec(&input_file_path(8, "input.txt")),
        _ => println!("To be implemented soon ... "),
    }
}

// cargo run
// cargo run interactive
fn main() {
    let current_day: u8 = 8;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "interactive" {
        interactive_mode();
    } else {
        exec_day(current_day);
    }
}
