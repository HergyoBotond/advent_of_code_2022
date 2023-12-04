use std::fs;

#[derive(Debug)]
struct Game{
    num: u32,
    green: u32,
    red: u32,
    blue: u32,
}

fn main() {
    let file_input = "input.txt";

    let result = get_result(file_input);

    println!("{}", result);
}

fn get_result(file_input: &str) -> u32 {

    let file_string = fs::read_to_string(file_input).unwrap();
    
    let mut games: Vec<Game> = Vec::new();

    for line in file_string.lines() {

        let mut game: Game = Game { num: 0, green: 0, red: 0, blue: 0 };

        let (line, sets) = line.split_once(':').unwrap();

        game.num = line
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        game = get_cubes(sets, game);

        games.push(game);
    }

    let mut result = 0;

    for game in games {
        let sum = game.blue * game.red * game.green;

        result += sum;
    }

    result
}

fn get_cubes(cubes: &str, mut game: Game) -> Game {

    for set in cubes.split(';').map(str::trim) {
            for cube in set.split(',').map(str::trim) {
                let (count, color) = cube.split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();

                match color {
                    "blue" => game.blue = u32::max(game.blue, count),
                    "green" => game.green = u32::max(game.green, count),
                    "red" => game.red = u32::max(game.red, count),
                    _ => panic!("unexpected color"),
                }
            }
        }

    game
}
