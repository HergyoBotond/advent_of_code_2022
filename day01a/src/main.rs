use std::fs;

fn main() {
    let file_path = "input.txt";
    let elfs = read_input(file_path);
    println!("{:?}", elfs);

    let it = elfs.iter().max();

    println!("{:?}", it);

    let result = it;

    match result {
        Some(&x) => println!("{x}"),
        None => println!("something went wrong"),
    }
}

fn read_input(file_path: &str) -> Vec<i32> {
    let mut elfs: Vec<i32> = Vec::new();
    let mut elf = 0;
    let mut num;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        if !line.is_empty() {
            num = line.parse::<i32>().unwrap();
            elf += num;
        } else {
            elfs.push(elf.clone());
            elf = 0;
        }
    }
    elfs
}
