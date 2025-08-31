use regex::Regex;
use std::fs;

const INPUT: &str = "day14input.txt";

#[derive(Debug)]
struct Robot {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

impl Robot {
    fn step(&mut self) {
        self.x_pos = (self.x_pos + self.x_vel).rem_euclid(WIDTH);
        self.y_pos = (self.y_pos + self.y_vel).rem_euclid(HEIGHT);
    }
}

fn steps(robots: &mut [Robot], n: u32, show_step: bool) {
    for s in 1..=n {
        robots.iter_mut().for_each(|r| r.step());
        if show_step {
            println!("{s}");
            show(robots);
        }
    }
}

fn safety_factor(robots: Vec<Robot>) -> usize {
    let middle_x = WIDTH / 2;
    let middle_y = HEIGHT / 2;
    let q1 = robots
        .iter()
        .filter(|r| r.x_pos < middle_x && r.y_pos < middle_y)
        .count();
    let q2 = robots
        .iter()
        .filter(|r| r.x_pos > middle_x && r.y_pos < middle_y)
        .count();
    let q3 = robots
        .iter()
        .filter(|r| r.x_pos < middle_x && r.y_pos > middle_y)
        .count();
    let q4 = robots
        .iter()
        .filter(|r| r.x_pos > middle_x && r.y_pos > middle_y)
        .count();
    q1 * q2 * q3 * q4
}

fn parse_line(s: &str) -> Robot {
    let re = Regex::new(r"p=(\d+),(\d+) v=([-]?\d+),([-]?\d+)").unwrap();
    let caps = re.captures(s).unwrap();
    let x_pos = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let y_pos = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let x_vel = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let y_vel = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
    Robot {
        x_pos,
        y_pos,
        x_vel,
        y_vel,
    }
}

fn parse_input() -> Vec<Robot> {
    let input = fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string();
    input.lines().map(parse_line).collect()
}

pub fn part1() {
    let mut robots = parse_input();
    steps(&mut robots, 100, false);
    let f = safety_factor(robots);
    println!("{f}")
}

fn show(robots: &[Robot]) {
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let c = if robots.iter().any(|r| r.x_pos == col && r.y_pos == row) {
                "#"
            } else {
                "."
            };
            print!("{c}");
        }
        println!();
    }
}

pub fn part2() {
    let mut robots = parse_input();
    steps(&mut robots, 7858, true);
}
