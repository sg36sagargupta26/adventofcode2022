use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;



pub fn part1(path: PathBuf) -> i32{
    let mut i = 0;
    let mut dequeue: VecDeque<char>= Vec::new().into();
    let mut flag: bool = true;
    fs::read_to_string(path)
        .unwrap()
        .chars()
        .for_each(|c|{
            if flag{
                i = i + 1;
                dequeue.push_back(c);
                if dequeue.len() == 4 {
                    if dequeue[0] == dequeue[1] || dequeue[0] == dequeue[2] || dequeue[0] == dequeue[3]
                        || dequeue[1] == dequeue[2] || dequeue[1] == dequeue[3]
                        || dequeue[2] == dequeue[3] {
                        dequeue.pop_front();
                    } else {
                        flag = false
                    }
                }
            }
        });
    i
}

pub fn part2(path: PathBuf) -> i32{
    let mut i = 0;
    let mut dequeue: VecDeque<char>= Vec::new().into();
    let mut flag: bool = true;
    fs::read_to_string(path)
        .unwrap()
        .chars()
        .for_each(|c|{
            if flag{
                i = i + 1;
                dequeue.push_back(c);
                if dequeue.len() == 14 {
                    if check_unique(dequeue.clone()){
                        dequeue.pop_front();
                    }
                    else {
                        flag = false
                    }
                }
            }
        });
    i
}
 fn check_unique(deque: VecDeque<char>) -> bool{
     let mut all_unique = false;
     let mut seen = [false; 26];

     for  e in deque {
         let c = (e as u8 - b'a') as usize;
         if seen[c] {
             all_unique=true;
             break;
         }
         seen[c] = true;
     }

     all_unique
 }
