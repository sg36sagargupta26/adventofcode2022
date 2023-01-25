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

fn group_priority_sum( a: &str, b: &str, c: &str) ->usize{
    let hashset_a: HashSet<char> = a.chars().collect();
    let hashset_b: HashSet<char> = b.chars().collect();
    let hashset_c: HashSet<char> = c.chars().collect();

    let mut sets: Vec<HashSet<char>> = vec![];

    sets.push(hashset_c);
    sets.push(hashset_b);
    sets.push(hashset_a);
    let mut iter = sets.iter();
    let base = iter.next().unwrap().clone();
    let mut  final_set = iter.fold(base, |acc, set| acc.intersection(set).map(|x| x.clone()).collect());
    let sum = final_set.drain().map(|a|SCORES.get(&a).unwrap()).sum();
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

pub fn part2(path: PathBuf) -> u32{
    let  score= fs::read_to_string(path)
        .unwrap()
        .lines().collect::<Vec<_>>()
        .chunks(3).
        map(|set| group_priority_sum(set[0],set[1],set[2]) as u32)
        .sum();
    score
}
