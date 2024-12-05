
use fancy_regex;
use regex;
use std::fs;
use std::path::Path;

pub fn part_one(input: &str) -> u32 {
    let re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let (_, [first, second]) = c.extract();
            first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap()
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let re = fancy_regex::Regex::new(r"(?s)(?<=do\(\))(.*?)(?=don't\(\))").unwrap();
    let mut text = String::from("do()");
    text.push_str(input);
    re.captures_iter(&text)
        .map(|c| {
            let capture = c.unwrap().get(0).unwrap().as_str();
            part_one(capture)
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

pub fn exec(input: &Path) -> () {
    let message: String = fs::read_to_string(input).unwrap();

    let result_one = part_one(&message);
    println!("Result part one: {:?}", result_one);

    let result_two = part_two(&message);
    println!("Result part two: {:?}", result_two);
}
