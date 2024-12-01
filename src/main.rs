use std::path::PathBuf;

mod day01;

fn main() {
    let mut path = PathBuf::new();
    path.push("src");
    path.push("day01");
    path.push("input.txt");
    day01::list_distance::list_distance(&path);
}
