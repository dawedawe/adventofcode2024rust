use std::fs;

const INPUT: &str = "day07input.txt";

struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

fn parse_line(line: &str) -> Equation {
    let numbers = line
        .split([':', ' '])
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>();
    let (a, b) = numbers.split_at(1);
    Equation {
        result: a[0],
        numbers: b.to_vec(),
    }
}

fn is_solvable(eq: &Equation) -> bool {
    let a = *eq.numbers.first().unwrap();
    let acc = vec![a];
    let results = eq.numbers.iter().skip(1).fold(acc, |acc, x| {
        let mut next_acc = Vec::new();
        for a in acc {
            next_acc.push(a + x);
            next_acc.push(a * x);
        }
        next_acc
    });

    results.contains(&eq.result)
}

pub fn part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let sum = input
        .lines()
        .map(parse_line)
        .filter(is_solvable)
        .map(|eq| eq.result)
        .sum::<i64>();
    println!("{}", sum);
}
