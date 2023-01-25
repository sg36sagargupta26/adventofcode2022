use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;
use std::{path::PathBuf, fs};


lazy_static! {
    static ref SCORES: HashMap<char, usize> = ('a'..='z').chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();
}

fn priority_sum  (fh: &str, sh: &str) -> usize{
    let hashset1: HashSet<char> = fh.chars().collect();
    let hashset2: HashSet<char> = sh.chars().collect();
    let sum = hashset2.intersection(&hashset1)
        .enumerate()
        .map(|(_i,a)|SCORES.get(&a).unwrap())
        .sum::<usize>();
    sum
}

pub fn part1(path: PathBuf) -> u32{
    let  score : u32= fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line|{
            let half = line.len() / 2;
            let first_half = &line[..half];
            let second_half = &line[half..];
            (priority_sum(first_half,second_half) )as u32
        }).sum();
    score
}


