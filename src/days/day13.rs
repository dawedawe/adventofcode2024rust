use std::{fs, ops::Sub};

use regex::Regex;

const INPUT: &str = "day13input.txt";

#[derive(Debug)]
struct Machine {
    a_x: u64,
    a_y: u64,
    b_x: u64,
    b_y: u64,
    prize: (u64, u64),
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
            let a_x = caps[1].parse::<u64>().unwrap();
            let a_y = caps[2].parse::<u64>().unwrap();

            let caps = re.captures(lines[1]).unwrap();
            let b_x = caps[1].parse::<u64>().unwrap();
            let b_y = caps[2].parse::<u64>().unwrap();

            let caps = re.captures(lines[2]).unwrap();
            let p_x = caps[1].parse::<u64>().unwrap();
            let p_y = caps[2].parse::<u64>().unwrap();

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

fn find_lowest(machine: &Machine) -> Option<u64> {
    let mut lowest: Option<u64> = None;
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
    let sum: u64 = machines.iter().filter_map(find_lowest).sum();
    print!("{sum}");
}

#[derive(Debug)]
struct Equation {
    factor_a: i64,
    factor_b: i64,
    result: i64,
}

impl Equation {
    fn mul(&self, rhs: i64) -> Self {
        Equation {
            factor_a: self.factor_a * rhs,
            factor_b: self.factor_b * rhs,
            result: self.result * rhs,
        }
    }
}

impl Sub for Equation {
    type Output = Equation;

    fn sub(self, rhs: Self) -> Self::Output {
        Equation {
            factor_a: self.factor_a - rhs.factor_a,
            factor_b: self.factor_b - rhs.factor_b,
            result: self.result - rhs.result,
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else if a <= b {
        let b = b % a;
        gcd(a, b)
    } else {
        gcd(b, a)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    let p = a * b;
    let c = gcd(a, b);
    assert!((p % c) == 0);
    p / c
}

fn find_solution(machine: &Machine) -> Option<(i64, i64)> {
    let equation_for_x = Equation {
        factor_a: machine.a_x as i64,
        factor_b: machine.b_x as i64,
        result: machine.prize.0 as i64,
    };
    let equation_for_y = Equation {
        factor_a: machine.a_y as i64,
        factor_b: machine.b_y as i64,
        result: machine.prize.1 as i64,
    };

    let factors_to_eliminate_b = {
        let r = lcm(
            equation_for_x.factor_b as u64,
            equation_for_y.factor_b as u64,
        );
        let r = Some(r);
        match r {
            None => None,
            Some(r) => {
                let fac1 = r as i64 / equation_for_x.factor_b;
                let fac2 = r as i64 / equation_for_y.factor_b;
                Some((fac1, fac2))
            }
        }
    };

    let a = match factors_to_eliminate_b {
        Some(factors_for_b) => {
            let equation_for_b1 = equation_for_x.mul(factors_for_b.0);
            let equation_for_b2 = equation_for_y.mul(factors_for_b.1);
            let equation_for_b = equation_for_b1 - equation_for_b2;
            assert!(equation_for_b.factor_b == 0);
            if equation_for_b.result % equation_for_b.factor_a == 0 {
                let a = equation_for_b.result / equation_for_b.factor_a;
                Some(a)
            } else {
                None
            }
        }
        None => None,
    };

    let factors_to_eliminate_a = {
        let r = lcm(
            equation_for_x.factor_a as u64,
            equation_for_y.factor_a as u64,
        );
        let r = Some(r);
        match r {
            None => None,
            Some(r) => {
                let fac1 = r as i64 / equation_for_x.factor_a;
                let fac2 = r as i64 / equation_for_y.factor_a;
                Some((fac1, fac2))
            }
        }
    };

    let b = match factors_to_eliminate_a {
        None => None,
        Some(factors_to_eliminate_a) => {
            let equation_for_a1 = equation_for_x.mul(factors_to_eliminate_a.0);
            let equation_for_a2 = equation_for_y.mul(factors_to_eliminate_a.1);
            let equation_for_a = equation_for_a1 - equation_for_a2;
            assert!(equation_for_a.factor_a == 0);

            if equation_for_a.result % equation_for_a.factor_b == 0 {
                let b = equation_for_a.result / equation_for_a.factor_b;
                Some(b)
            } else {
                None
            }
        }
    };

    match (a, b) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

fn calc_tokens(machine: &Machine) -> Option<i64> {
    find_solution(machine).map(|(a, b)| a * 3 + b)
}

pub fn part2() {
    let machines = parse_input();
    let machines: Vec<Machine> = machines
        .iter()
        .map(|m| Machine {
            a_x: m.a_x,
            a_y: m.a_y,
            b_x: m.b_x,
            b_y: m.b_y,
            prize: (m.prize.0 + 10000000000000, m.prize.1 + 10000000000000),
        })
        .collect();
    let sum: i64 = machines.iter().filter_map(calc_tokens).sum();
    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_with_zeroes() {
        assert_eq!(gcd(23, 0), 23);
        assert_eq!(gcd(0, 42), 42);
    }

    #[test]
    fn lcm_with_zeroes() {
        assert_eq!(lcm(23, 0), 0);
        assert_eq!(lcm(0, 42), 0);
    }

    #[test]
    fn lcm1() {
        assert_eq!(lcm(94, 37), 3478);
        assert_eq!(lcm(22, 67), 1474);
    }

    #[test]
    fn find_solution_test1() {
        let m = Machine {
            a_x: 94,
            a_y: 34,
            b_x: 22,
            b_y: 67,
            prize: (8400, 5400),
        };
        let ab = find_solution(&m);
        assert_eq!(ab, Some((80, 40)))
    }

    #[test]
    fn find_solution_test2() {
        let m = Machine {
            a_x: 26,
            a_y: 66,
            b_x: 67,
            b_y: 21,
            prize: (12748, 12176),
        };
        let ab = find_solution(&m);
        assert_eq!(ab, None)
    }
    #[test]
    fn find_solution_test3() {
        let m = Machine {
            a_x: 17,
            a_y: 86,
            b_x: 84,
            b_y: 37,
            prize: (7870, 6450),
        };
        let ab = find_solution(&m);
        assert_eq!(ab, Some((38, 86)))
    }
}
