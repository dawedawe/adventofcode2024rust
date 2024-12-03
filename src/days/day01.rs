use std::{collections::HashMap, fs};

const INPUT: &str = "day01input.txt";

pub fn part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    input.lines().for_each(|line| {
        let parts: Vec<_> = line.split_whitespace().collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    });
    left.sort();
    right.sort();
    let pairs = left.into_iter().zip(right);
    let s: i32 = pairs.map(|(l, r)| (l - r).abs()).sum();
    println!("{s}")
}

pub fn part2() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    input.lines().for_each(|line| {
        let parts: Vec<_> = line.split_whitespace().collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    });
    let mut m: HashMap<i32, i32> = HashMap::new();
    right.iter().for_each(|r| {
        let c = m.get(r).unwrap_or(&0);
        m.insert(*r, c + 1);
    });

    let s: i32 = left.iter().map(|l| l * m.get(l).unwrap_or(&0)).sum();
    println!("{}", s)
}
