
use std::path::PathBuf;
// use day01;
// use day02;
use day03;

fn main() {
    /* day1: solutions*/
    // let path1 = PathBuf::from("src/day01/data/input.txt");
    // let path2 = path1.clone();
    // println!("{}",day01::part1(path1));
    // println!("{}",day01::part2(path2));

    /* day2: solutions*/
    // let path1 = PathBuf::from("src/day02/data/input.txt");
    // let path2 = path1.clone();
    // println!("{}", day02::part1(path1));
    // println!("{}", day02::part2(path2));

    /* day3: solutions*/
    let path1 = PathBuf::from("src/day03/data/input.txt");
    // let path2 = path1.clone();
    println!("{}", day03::part1(path1));
    //println!("{}", day02::part2(path2));
}
