use std::{fs, ops::Div};

const INPUT: &str = "day17input.txt";

struct State {
    registers: [u32; 3],
    program: Vec<u32>,
    ip: usize,
}

type OperationResult = (Option<u32>, bool);

impl State {
    fn operand_value(&self, operand: u32) -> u32 {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[operand as usize - 4],
            _ => panic!("reserved operand"),
        }
    }

    // opcode 0
    fn adv(&mut self, operand: u32) -> OperationResult {
        let op = self.operand_value(operand);
        let numerator = self.registers[0];
        let denominator = u32::pow(2, op);
        self.registers[0] = numerator.div(denominator);
        (None, false)
    }

    // opcode 1
    fn bxl(&mut self, operand: u32) -> OperationResult {
        self.registers[1] ^= operand;
        (None, false)
    }

    // opcode 2
    fn bst(&mut self, operand: u32) -> OperationResult {
        let op = self.operand_value(operand);
        self.registers[1] = op % 8;
        (None, false)
    }

    // opcode 3
    fn jnz(&mut self, operand: u32) -> OperationResult {
        if self.registers[0] != 0 {
            self.ip = operand as usize;
            (None, true)
        } else {
            (None, false)
        }
    }

    // opcode 4
    fn bxc(&mut self, _operand: u32) -> OperationResult {
        self.registers[1] ^= self.registers[2];
        (None, false)
    }

    // opcode 5
    fn out(&mut self, operand: u32) -> OperationResult {
        let op = self.operand_value(operand);
        let r = Some(op % 8);
        (r, false)
    }

    // opcode 6
    fn bdv(&mut self, operand: u32) -> OperationResult {
        let op = self.operand_value(operand);
        let numerator = self.registers[0];
        let denominator = u32::pow(2, op);
        self.registers[1] = numerator.div(denominator);
        (None, false)
    }

    // opcode 7
    fn cdv(&mut self, operand: u32) -> OperationResult {
        let op = self.operand_value(operand);
        let numerator = self.registers[0];
        let denominator = u32::pow(2, op);
        self.registers[2] = numerator.div(denominator);
        (None, false)
    }

    fn do_cycle(&mut self) -> Option<u32> {
        let opcode = self.program[self.ip];
        let operand = self.program[self.ip + 1];
        let (output, jumped) = match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => todo!(),
        };
        if !jumped {
            self.ip += 2;
        }
        output
    }

    fn run(&mut self) -> Vec<u32> {
        let mut output = vec![];
        while self.ip < self.program.len() {
            if let Some(o) = self.do_cycle() {
                output.push(o)
            }
        }
        output
    }
}

fn parse_input() -> State {
    let lines = fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string();
    let lines: Vec<&str> = lines.lines().collect();

    let reg_a: u32 = lines[0]
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let reg_b: u32 = lines[1]
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let reg_c: u32 = lines[2]
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let program = lines[4].split_at(9).1;
    let program = program
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    State {
        registers: [reg_a, reg_b, reg_c],
        program,
        ip: 0,
    }
}

pub fn part1() {
    let mut state = parse_input();
    let r = state.run();
    let mut output = String::new();
    r.iter().for_each(|n| {
        output.push(char::from_digit(*n, 10).unwrap());
        output.push(',');
    });
    let output = output.trim_matches(',');
    println!("{output}");
}
