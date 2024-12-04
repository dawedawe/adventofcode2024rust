use std::fs;

const INPUT: &str = "day02input.txt";

fn is_increasing_safe(numbers: &[i32]) -> bool {
    numbers.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
}

fn is_decreasing_safe(numbers: &[i32]) -> bool {
    numbers.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3)
}

fn is_safe(numbers: &[i32]) -> bool {
    is_increasing_safe(numbers) || is_decreasing_safe(numbers)
}

pub fn part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let safe_reports = input
        .lines()
        .filter(|line| {
            let numbers = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            is_safe(&numbers)
        })
        .count();
    println!("{safe_reports}");
}

pub fn part2() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let safe_reports = input
        .lines()
        .filter(|line| {
            let numbers = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            if is_safe(&numbers) {
                true
            } else {
                for i in 0..numbers.len() {
                    let mut numbers2 = numbers.clone();
                    numbers2.remove(i);
                    if is_safe(&numbers2) {
                        return true;
                    }
                }
                false
            }
        })
        .count();
    println!("{safe_reports}");
}
