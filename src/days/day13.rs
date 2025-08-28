use std::fs;

use regex::Regex;

const INPUT: &str = "day13input.txt";

#[derive(Debug)]
struct Machine {
    a_x: u32,
    a_y: u32,
    b_x: u32,
    b_y: u32,
    prize: (u32, u32),
}

fn parse_input() -> Vec<Machine> {
    let re = Regex::new(r"X.(\d+), Y.(\d+)").unwrap();
    let input = fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string();
    input
        .lines()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|lines| {
            let caps = re.captures(lines[0]).unwrap();
            let a_x = caps[1].parse::<u32>().unwrap();
            let a_y = caps[2].parse::<u32>().unwrap();

            let caps = re.captures(lines[1]).unwrap();
            let b_x = caps[1].parse::<u32>().unwrap();
            let b_y = caps[2].parse::<u32>().unwrap();

            let caps = re.captures(lines[2]).unwrap();
            let p_x = caps[1].parse::<u32>().unwrap();
            let p_y = caps[2].parse::<u32>().unwrap();
            Machine {
                a_x,
                a_y,
                b_x,
                b_y,
                prize: (p_x, p_y),
            }
        })
        .collect::<Vec<Machine>>()
}

fn find_lowest(machine: &Machine) -> Option<u32> {
    let mut lowest: Option<u32> = None;
    for a in 0..100 {
        for b in 0..100 {
            let target = (
                a * machine.a_x + b * machine.b_x,
                a * machine.a_y + b * machine.b_y,
            );
            if target == machine.prize {
                let tokens = a * 3 + b;
                match lowest {
                    None => lowest = Some(tokens),
                    Some(x) if x > tokens => lowest = Some(tokens),
                    _ => (),
                }
            }
        }
    }
    lowest
}

pub fn part1() {
    let machines = parse_input();
    let sum: u32 = machines.iter().filter_map(find_lowest).sum();
    print!("{sum}");
}
