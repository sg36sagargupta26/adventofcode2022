use std::path::PathBuf;
use day01::{part1, part2};
fn main() {
    let path1 = PathBuf::from("src/day01/data/inputPart1.txt");
    let path2 = PathBuf::from("src/day01/data/inputPart1.txt");
    println!("{}",part1(path1));
    println!("{}",part2(path2));
}
