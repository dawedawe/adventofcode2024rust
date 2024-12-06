use regex::Regex;
use std::fs;

const INPUT: &str = "day03input.txt";

pub fn part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let s = re
        .captures_iter(input.as_str())
        .map(|m| {
            let a = m.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = m.get(2).unwrap().as_str().parse::<i32>().unwrap();
            a * b
        })
        .sum::<i32>();
    println!("{s}")
}

pub fn part2() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;
    let s = re
        .captures_iter(input.as_str())
        .map(|m| match m.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ => {
                if enabled {
                    let a = m.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let b = m.get(3).unwrap().as_str().parse::<i32>().unwrap();
                    a * b
                } else {
                    0
                }
            }
        })
        .sum::<i32>();
    println!("{s}")
}
