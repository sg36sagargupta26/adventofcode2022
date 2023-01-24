use std::path::PathBuf;
use day01::part1;
fn main() {
   let path = PathBuf::from("src/day01/data/inputPart1.txt");
    println!("{}",part1(path));
}
