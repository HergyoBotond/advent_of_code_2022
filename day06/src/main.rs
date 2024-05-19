
fn read_input(time: &mut Vec<i64>, dis: &mut Vec<i64>) {
    let data = std::fs::read_to_string("input2.txt").expect("Error reading file");

    for line in data.lines() {
        let mut split = line.split_whitespace();

        if split.next().unwrap() == "Time:" {
            for num in split.collect::<Vec<&str>>().iter() {
                time.push(num.parse::<i64>().unwrap());
            }
        } else {
            for num in split.collect::<Vec<&str>>().iter() {
                dis.push(num.parse::<i64>().unwrap());
            }
        }
    }
}

fn main() {
    let mut time: Vec<i64> = Vec::new();
    let mut dis: Vec<i64> = Vec::new();
    let test_time = vec![71530];
    let test_dis = vec![940200];

    read_input(&mut time, &mut dis);
    let result = part_one(time.clone(), dis.clone());

    println!("Times: {:?}", time);
    println!("Distances: {:?}", dis);
    println!("Result: {}", result);
}

fn part_one(time: Vec<i64>, dis: Vec<i64>) -> i64 {
    let mut results: Vec<i64> = Vec::new();
    let rounds = time.len();

    for i in 0..rounds {
        let mut ways = 0;
        let mid: i64 = time[i] / 2 as i64;

        for j in mid..time[i] {
            let distance: i64 = (time[i] - j) * j;
            
            if distance > dis[i] {
                ways += 1;
            } else {
                break;
            }
        }

        for j in (0..mid).rev() {
            let distance: i64 = (time[i] - j) * j;
            
            if distance > dis[i] {
                ways += 1;
            } else {
                break;
            }
        }

        results.push(ways);
    }

    let mut sum = 1;

    for i in 0..results.len() {
        sum *= results[i];
    }

    sum     
}



