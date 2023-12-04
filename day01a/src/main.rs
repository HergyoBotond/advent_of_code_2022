use std::fs;

fn main() {

    let file_name = "input.txt";

    let result = get_result(file_name);

    println!("{}", result);
}

fn get_result(file_name: &str) -> i32 {

    let mut sum: i32 = 0;

    let file_string = fs::read_to_string(file_name).unwrap();

    for line in file_string.lines() {
        let first_num = line.chars().find(|c: &char| c.is_digit(10)).unwrap();
        let last_num = line.chars().rev().find(|c: &char| c.is_digit(10)).unwrap();

        let mut number = String::new();

        number.push(first_num);
        number.push(last_num);

        sum += number.parse::<i32>().unwrap();
    }
    
    sum

}
