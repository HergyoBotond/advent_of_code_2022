use std::{fs, char};

#[derive(Debug)]
struct Card {
    card_num: i32,
    winning_nums: Vec<i32>,
    my_nums: Vec<i32>,
}

impl Card {
    fn calc_winning(&self) -> i32 {
        let mut winnings = 0;
        let points: i32 = 2;

        for num in self.my_nums.iter() {
            if self.winning_nums.contains(&num){
                winnings += 1;
            }
        }

        if winnings > 0 {
            return points.pow(winnings - 1);
        }
        0
    }
}

fn main() {
    let file_input = fs::read_to_string("input.txt").unwrap();

    let cards: Vec<Card> = parse_file(&file_input);

    let points: i32 = cards.iter().map(|card| card.calc_winning()).sum();

    println!("{:?}", points);
}

fn parse_file(file: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];

    for (y, line) in file.lines().enumerate() {
        let nums = line.split_once(": ").unwrap().1;

        let winning: Vec<&str> = nums
            .split_once(" | ")
            .unwrap().0
            .split_whitespace()
            .filter(|num| num.parse::<i32>().is_ok())
            .collect();
        
        let mine: Vec<&str> = nums
            .split_once(" | ")
            .unwrap().1
            .split_whitespace()
            .filter(|num| num.parse::<i32>().is_ok())
            .collect();

        let card: Card = Card { 
            card_num: (y + 1) as i32,
            winning_nums: winning.iter().map(|num| num.parse::<i32>().unwrap()).collect(),
            my_nums: mine.iter().map(|num| num.parse::<i32>().unwrap()).collect(),
        };
        cards.push(card);
    }
    cards
}

