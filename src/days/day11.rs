use std::{collections::HashMap, fs};

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

fn blink(times: usize, stones: HashMap<String, u64>) -> u64 {
    if times > 0 {
        let mut new_stones: HashMap<String, u64> = HashMap::new();
        for (k, count) in stones {
            let transformed = transform(k.to_string());
            for stone in transformed {
                match new_stones.get(&stone) {
                    Some(existing) => new_stones.insert(stone, existing + count),
                    None => new_stones.insert(stone, count),
                };
            }
        }

        blink(times - 1, new_stones)
    } else {
        stones.values().sum()
    }
}

pub fn parse_input() -> HashMap<String, u64> {
    let input = fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string();

    let mut map = HashMap::new();
    input.split_whitespace().for_each(|s| {
        let key = s.to_string();
        map.entry(key).or_insert(1);
    });
    map
}

pub fn part1() {
    let stones = parse_input();
    let sum = blink(25, stones);
    println!("{}", sum);
}

pub fn part2() {
    let stones = parse_input();
    let sum = blink(75, stones);
    println!("{}", sum);
}
