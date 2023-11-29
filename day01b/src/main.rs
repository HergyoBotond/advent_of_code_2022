use std::fs;

fn main() {

    let mut elfs = read_input();

    elfs.sort_unstable();

    println!("{}", elfs.into_iter().rev().take(3).sum::<i32>());
}

fn read_input() -> Vec<i32> {
    const FILE_NAME: &str = "input.txt";
    let mut elfs: Vec<i32> = Vec::new();
    let mut elf = 0;
    let mut num;

    for line in fs::read_to_string(FILE_NAME).unwrap().lines() {
         
        if !line.is_empty() {
            num = line.parse::<i32>().unwrap();
            elf += num;
        }
        else{
            elfs.push(elf.clone());
            elf = 0;
        }
    }
    elfs
}
