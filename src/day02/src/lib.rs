use std::{cmp::Ordering, str::FromStr, fs, path::PathBuf};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock
            && other == &Move::Scissors
        {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}
fn matcher( opponent_moves: &str ,my_moves: &str)->u32{
    let opponent_move = opponent_moves.parse::<Move>().unwrap();
    match my_moves {
        "X" => {
            let my_move = match opponent_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            };
            my_move as u32
        }
        "Y" => 3 + opponent_move as u32,
        "Z" => {
            let my_move = match opponent_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            };
            6 + my_move as u32
        }
        _=> {
            panic!("Unexpected response");
        }
    }
}

pub fn part1(path: PathBuf) -> u32{
    let  score = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line|{
            let moves : Vec<Move> = line
                .split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => 3 + moves[1] as u32, // draw
                Some(Ordering::Less) => 6 + moves[1] as u32,  // win
                Some(Ordering::Greater) => moves[1] as u32,   // loss
                None => panic!("moves should be comparable"), //panic
            }
        }).sum();
    score
}

pub fn part2(path: PathBuf) -> u32{
    let  score = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line|{
            let moves : Vec<&str> = line.split(" ").collect();
            matcher(moves[0], moves[1])
        }).sum();
    score
}

