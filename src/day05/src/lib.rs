use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;


struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from_line(line: &str) -> Self {
        let mut split = line.split(|c| c == ' ');
        let count = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap();
        let to = split.nth(1).unwrap().parse::<usize>().unwrap();
        Self { count, from, to }
    }
}



fn create_stacks(path: PathBuf)-> Vec<VecDeque<u8>>{
    let  string_stack : Vec<Vec<u8>> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| {
            let temp = s.as_bytes().to_vec();
            let mut chars : Vec<u8> = Vec::new();
            for (i, u) in temp.iter().enumerate(){
                if i!=0 &&(i-1)%4==0{
                    chars.push(*u);
                }
            }
            chars
        })
        .collect();
    let clen= string_stack.len();
    let rlen = string_stack[0].len();
    let mut stack : Vec<VecDeque<u8>> = Vec::new();

    for i in 0..rlen{
        let mut temp : VecDeque<u8> = Vec::new().into();
        for j in 0..clen{
            if string_stack[j][i]!=32{
                temp.push_back(string_stack[j][i])
            }
        }
        stack.push(temp);
    }
    stack

}

pub fn part1(path: PathBuf, path2: PathBuf) -> String {
    let mut stack = create_stacks(path).clone();
    fs::read_to_string(path2)
        .unwrap()
        .lines()
        .for_each(|str|{
            let m = Move::from_line(str);
            for _ in 0..m.count {
                let elem = stack[m.from - 1].pop_front().unwrap();
                stack[m.to - 1].push_front(elem);
            }
        });
    let mut res = String::with_capacity(stack.len());
    stack
        .iter()
        .for_each(|s| res.push(*s.front().unwrap() as char));
    res
}

pub fn part2(path: PathBuf, path2: PathBuf) -> String {
    let mut stack = create_stacks(path).clone();
    fs::read_to_string(path2)
        .unwrap()
        .lines()
        .for_each(|str|{
            let m = Move::from_line(str);
            let mut n : VecDeque<u8> = Vec::new().into();
            for _ in 0..m.count {
                let elem = stack[m.from - 1].pop_front().unwrap();
                n.push_front(elem);
            }
            for _ in 0..m.count {
                let elem = n.pop_front().unwrap();
                stack[m.to - 1].push_front(elem);
            }

        });
    let mut res = String::with_capacity(stack.len());
    stack
        .iter()
        .for_each(|s| res.push(*s.front().unwrap() as char));
    res
}
