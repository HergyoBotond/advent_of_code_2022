use std::{collections::HashMap, fs, path::PrefixComponent};

fn main() {
    let file_input = "example.txt";

    let _part_one = part1(file_input);
    let part_two = part2(file_input);

    println!("solution: {:?}", part_two);
}

fn part1(file: &str) -> Vec<i64> {
    let file_content = fs::read_to_string(file).unwrap();
    let mut seeds: Vec<i64> = Vec::new();
    let mut destination_start = Vec::new();
    let mut range = Vec::new();
    let mut source_start = Vec::new();
    let mut line_count = 0;
    let last_line = file_content.lines().last().unwrap();

    let mut change: HashMap<i64, i64> = HashMap::new();

    for (i, line) in file_content.lines().enumerate() {
        if i == 0usize {
            seeds = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect();
        } else {
            if line.starts_with(|c: char| c.is_ascii_digit()) {
                let ids: Vec<i64> = line
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect();

                destination_start.push(ids[0]);
                source_start.push(ids[1]);
                range.push(ids[2]);

                line_count += 1;

                //println!("{:?}", line);
            }

            if line.starts_with(|c: char| !c.is_ascii_digit()) || line == last_line {
                for x in 0..line_count {
                    for (j, seed) in seeds.clone().iter().enumerate() {
                        if *seed > source_start[x] && *seed < source_start[x] + range[x] {
                            let diff = seed.clone() - source_start[x];

                            let dest_num: i64 = destination_start[x].clone() + diff;

                            if !change.contains_key(&seeds[j]) {
                                change.insert(dest_num, seeds[j]);
                                seeds[j] = dest_num;
                            }
                        }
                    }
                }

                //println!("{:?}", change);

                line_count = 0;
                destination_start = Vec::new();
                range = Vec::new();
                source_start = Vec::new();
                change = HashMap::new();
            }
        }
    }

    seeds
}

fn part2(file: &str) -> i64 {
    let file_content = fs::read_to_string(file).unwrap();
    let mut seed_range: Vec<(i64, i64)> = Vec::new();
    let mut destination_start = Vec::new();
    let mut range = Vec::new();
    let mut source_start = Vec::new();
    let mut line_count = 0;
    let last_line = file_content.lines().last().unwrap();

    let mut change: HashMap<i64, i64> = HashMap::new();

    for (i, line) in file_content.lines().enumerate() {
        if i == 0usize {
            let nums: Vec<i64> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect();

            let mut start: i64 = 0;
            let mut end: i64 = 0;

            let mut ranges: Vec<(i64, i64)> = Vec::new();

            for (z, num) in nums.iter().enumerate() {
                if z % 2 == 0 {
                    start = *num;
                } else {
                    end = start + *num;
                    ranges.push((start, end));
                }
            }
            seed_range = ranges;
        } else {
            if line.starts_with(|c: char| c.is_ascii_digit()) {
                let ids: Vec<i64> = line
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect();

                destination_start.push(ids[0]);
                source_start.push(ids[1]);
                range.push(ids[2]);

                line_count += 1;

                //println!("{:?}", line);
            }

            if line.starts_with(|c: char| !c.is_ascii_digit()) || line == last_line {
                let mut new_ranges: Vec<(i64, i64)> = seed_range.clone();

                for x in 0..line_count {
                    let map_range = source_start[x]..source_start[x] + range[x];

                    println!("{:?}", map_range);

                    for (j, seed) in seed_range.clone().iter().enumerate() {
                        let end = seed.0 + seed.1;
                        let start = seed.0;
                        if map_range.contains(&seed.0) || map_range.contains(&seed.1) {
                            if map_range.contains(&start) && !map_range.contains(&end) {
                                let diff = start - map_range.start;
                                let new_range =
                                    (destination_start[x] + diff, destination_start[x] + range[x]);

                                new_ranges[j].1 = map_range.end + 1;

                                if !new_ranges.contains(&new_range) {
                                    new_ranges.push(new_range);
                                }
                            } else if !map_range.contains(&start) && map_range.contains(&end) {
                                let diff = end - map_range.start;
                                let true_end = destination_start[x] + diff;
                                let new_range = (destination_start[x], true_end);

                                new_ranges[j].0 = map_range.start - 1;

                                if !new_ranges.contains(&new_range) {
                                    new_ranges.push(new_range);
                                }
                            }
                        }
                    }
                }

                println!("{:?}", new_ranges);

                seed_range = new_ranges;

                line_count = 0;
                destination_start = Vec::new();
                range = Vec::new();
                source_start = Vec::new();
                change = HashMap::new();
            }
        }
    }

    let starts = seed_range.iter().map(|s| s.0).min().unwrap();

    starts
}
