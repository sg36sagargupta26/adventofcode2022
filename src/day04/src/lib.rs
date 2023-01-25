use std::fs;
use std::path::PathBuf;

pub fn part1(path: PathBuf) -> u32{
    let overlap = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|str|{
            let elfs = Vec::from_iter(str.split(',').map(String::from));
            let elf0=Vec::from_iter(elfs[0].split('-').map(String::from));
            let elf1=Vec::from_iter(elfs[1].split('-').map(String::from));
            let e00: i32=elf0[0].parse::<i32>().unwrap();
            let e01: i32=elf0[1].parse::<i32>().unwrap();
            let e10: i32=elf1[0].parse::<i32>().unwrap();
            let e11: i32=elf1[1].parse::<i32>().unwrap();
            if ((e10<= e00) && (e11>= e01) && (e00<=e11) && (e11>=e10)) ||
                ((e10>= e00) && (e11<= e01) && (e10<=e01) && (e01>=e10)){
                return 1;
            }
            return 0;
        }).sum();
    overlap
}

pub fn part2(path: PathBuf) -> u32{
    let overlap = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|str|{
            let elfs = Vec::from_iter(str.split(',').map(String::from));
            let elf0=Vec::from_iter(elfs[0].split('-').map(String::from));
            let elf1=Vec::from_iter(elfs[1].split('-').map(String::from));
            let e00: i32=elf0[0].parse::<i32>().unwrap();
            let e01: i32=elf0[1].parse::<i32>().unwrap();
            let e10: i32=elf1[0].parse::<i32>().unwrap();
            let e11: i32=elf1[1].parse::<i32>().unwrap();
            if e00 <= e10 && e01 >= e10 || e00 <= e11 && e01 >= e11 || e10 <= e00 && e11 >= e00 || e10 <= e01 && e11 >= e01{
                return 1;
            }
            return 0;
        }).sum();
    overlap
}
