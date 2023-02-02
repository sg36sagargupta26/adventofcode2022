use std::fs;
use std::path::PathBuf;



fn create_stacks(path: PathBuf)-> Vec<Vec<u8>>{
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
    let mut stack : Vec<Vec<u8>> = Vec::new();

    for i in 0..rlen{
        let mut temp : Vec<u8> = Vec::new();
        for j in 0..clen{
            if string_stack[j][i]!=32{
                temp.push(string_stack[j][i])
            }
        }
        stack.push(temp);
    }
    stack

}

pub fn part1(path: PathBuf, _path2: PathBuf) {
    let  stack = create_stacks(path).clone();
    println!("{:?}",stack)
}

    // pub fn part2(path: PathBuf) -> u32{
    //     let overlap = fs::read_to_string(path)
    //         .unwrap()
    //         .lines()
    //         .map(|str|{
    //             let elfs = Vec::from_iter(str.split(',').map(String::from));
    //             let elf0=Vec::from_iter(elfs[0].split('-').map(String::from));
    //             let elf1=Vec::from_iter(elfs[1].split('-').map(String::from));
    //             let e00: i32=elf0[0].parse::<i32>().unwrap();
    //             let e01: i32=elf0[1].parse::<i32>().unwrap();
    //             let e10: i32=elf1[0].parse::<i32>().unwrap();
    //             let e11: i32=elf1[1].parse::<i32>().unwrap();
    //             if e00 <= e10 && e01 >= e10 || e00 <= e11 && e01 >= e11 || e10 <= e00 && e11 >= e00 || e10 <= e01 && e11 >= e01{
    //                 return 1;
    //             }
    //             return 0;
    //         }).sum();
    //     overlap
    // }
