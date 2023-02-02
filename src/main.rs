
use std::path::PathBuf;
// use day01;
// use day02;
// use day03;
use day05;

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
    // let path1 = PathBuf::from("src/day03/data/input.txt");
    // let path2 = path1.clone();
    // println!("{}", day03::part1(path1));
    // println!("{}", day03::part2(path2));

    /* day4: solutions*/
    // let path1 = PathBuf::from("src/day04/data/input.txt");
    // let path2 = path1.clone();
    // println!("{}", day04::part1(path1));
    // println!("{}", day04::part2(path2));

    /* day5: solutions*/
    let path1a = PathBuf::from("src/day05/data/input.txt");
    let path1b = PathBuf::from("src/day05/data/input2.txt");
    //let path2 = path1.clone();
    println!("{}", day05::part1(path1a.clone(),path1b.clone()));
    println!("{}", day05::part2(path1a,path1b));
}
