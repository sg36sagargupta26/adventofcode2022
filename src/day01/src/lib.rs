use std::fs;
use std::path::PathBuf;

pub fn part1(path: PathBuf) -> u32{
    if path.exists(){
        println!("path exists");
    }else{
        println!("why")
    }
    let mut calories = fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(|elf_load|{
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        }).collect::<Vec<_>>();
    calories.sort_by(|a,b|b.cmp(a));
    let ans = calories.first().unwrap_or(&0);
    *ans
}