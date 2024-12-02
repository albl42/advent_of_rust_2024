use std::{
    env, io,
    path::{Path, PathBuf},
};

mod day01;
mod day02;

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
        _ => println!("To be implemented soon ... "),
    }
}

// cargo run
// cargo run interactive
fn main() {
    let current_day: u8 = 2;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "interactive" {
        interactive_mode();
    } else {
        exec_day(current_day);
    }
}
