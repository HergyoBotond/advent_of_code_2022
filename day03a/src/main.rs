use std::fs;

#[derive(Debug)]
struct PartNumber {
    number: i32,
    x1: i32,
    x2: i32,
    y: i32,
}

impl PartNumber {
    fn is_adjacent_to_symbol(&self, symbol: &Symbol) -> bool {
        (self.x1 - 1..=self.x2 + 1).contains(&symbol.x) &&
        (self.y - 1..=self.y + 1).contains(&symbol.y)

    }
    
    fn is_adjacent_to_any_number(&self, symbols: &[Symbol]) -> bool {
        symbols.iter().any(|symbol| self.is_adjacent_to_symbol(symbol))
    } 
}

#[derive(Debug)]
struct Symbol {
    x: i32,
    y: i32,
}


fn parse_input(input: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut n: Option<i32> = None;
        let mut length: i32 = 0;

        for (x, &c) in line.as_bytes().iter().chain([b'.'].iter()).enumerate() {
            if c.is_ascii_digit() {
                length += 1;

                let digit = (c - b'0') as i32;

                n = n.map_or(Some(digit), |number| Some(number * 10 + digit));
            }
            else {
                if let Some(number) = n {
                    part_numbers.push(PartNumber { 
                        number,
                        x1: x as i32 - length,
                        x2: x as i32 - 1, 
                        y: y as i32, 
                    });
                }

                n = None;

                length = 0;

               if c != b'.' {
                    symbols.push(Symbol { 
                        x: x as i32, 
                        y: y as i32, 
                    });
                }
            }
        }
    }

    (part_numbers, symbols)
}

fn main() {
    let file_input = fs::read_to_string("input.txt").unwrap();

    let (part_numbers, symbols) = parse_input(&file_input);

    let result: i32 = part_numbers
        .iter()
        .filter(|pn| pn.is_adjacent_to_any_number(&symbols))
        .map(|pn| pn.number)
        .sum();
    
    println!("{:?}", result);
}
