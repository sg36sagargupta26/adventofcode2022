use std::fs;
use std::path::PathBuf;

pub fn part1(path: PathBuf) -> u32{
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

pub fn part2(path: PathBuf) -> u32{
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
    let sum = calories.iter().take(3).sum::<u32>();
    sum
}
