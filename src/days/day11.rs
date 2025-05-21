use std::fs;

const INPUT: &str = "day11input.txt";

fn transform(stone: String) -> Vec<String> {
    match stone {
        n if n == *"0" => vec![String::from("1")],
        n if n.len() % 2 == 0 => {
            let (left, right) = n.split_at(n.len() / 2);
            let left = left.trim_start_matches("0").to_string();
            let right = right.parse::<i64>().unwrap().to_string();
            vec![left, right]
        }
        n => {
            let n = n.parse::<i64>().unwrap() * 2024;
            let n = n.to_string();
            vec![n]
        }
    }
}

fn blink(times: usize, stones: Vec<String>) -> Vec<String> {
    if times == 0 {
        stones
    } else {
        let stones = stones.into_iter().flat_map(transform).collect();
        blink(times - 1, stones)
    }
}

pub fn part1() {
    let input = fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string();

    let stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    let stones = blink(25, stones);
    println!("{}", stones.len());
}
