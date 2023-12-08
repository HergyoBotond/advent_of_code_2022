use std::fs;

#[derive(Debug)]
struct Card {
    card_num: i32,
    winning_nums: Vec<i32>,
    my_nums: Vec<i32>,
    wins: u32,
    calls: u32,
}

impl Card {
    fn calc_winning(&mut self){
        let mut winnings = 0;

        for num in self.my_nums.iter() {
            if self.winning_nums.contains(&num){
                winnings += 1;
            }
        }
        self.wins += winnings;
    }
}

fn main() {
    let file_input = fs::read_to_string("input.txt").unwrap();

    let cards: Vec<Card> = parse_file(&file_input);

    let result = calc_points(cards);

    println!("{:?}", result);

}

fn calc_points(mut cards: Vec<Card>) -> u32 {
    let mut sum = 0;
    for i in 0..cards.len() {
        let end = (cards[i].wins + 1) as usize;
        for j in 1..end {
            let mut location = i+j;

            if location > cards.len(){
                location = cards.len();
            }

            cards[location].calls += cards[i].calls;
        }

        sum += cards[i].calls;
    }

    sum
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

        let mut card: Card = Card { 
            card_num: (y + 1) as i32,
            winning_nums: winning.iter().map(|num| num.parse::<i32>().unwrap()).collect(),
            my_nums: mine.iter().map(|num| num.parse::<i32>().unwrap()).collect(),
            wins: 0,
            calls: 1,
        };
        card.calc_winning();
        cards.push(card);
    }
    cards
}

