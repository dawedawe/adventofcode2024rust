use core::panic;
use std::fs;

const INPUT: &str = "day15input.txt";

type Map = Vec<Vec<char>>;
struct State {
    map: Map,
    robot: (usize, usize),
}

impl State {
    fn move_robot(&mut self, new_robot: (usize, usize)) {
        self.map[self.robot.0][self.robot.1] = '.';
        self.map[new_robot.0][new_robot.1] = '@';
        self.robot = new_robot;
    }

    fn try_move_left(&mut self) {
        let new_robot = (self.robot.0, self.robot.1 - 1);

        match self.map[new_robot.0][new_robot.1] {
            '#' => (),
            '.' => {
                self.move_robot(new_robot);
            }
            'O' => {
                let first_after_boxes = {
                    let mut i = self.robot.1 - 2;
                    while self.map[self.robot.0][i] == 'O' {
                        i -= 1;
                    }
                    i
                };
                match self.map[self.robot.0][first_after_boxes] {
                    '#' => (),
                    '.' => {
                        let left_most_box = first_after_boxes + 1;
                        let boxes_count = self.robot.1 - left_most_box;
                        for i in 0..boxes_count {
                            self.map[self.robot.0][first_after_boxes + i] = 'O';
                        }
                        self.move_robot(new_robot);
                    }
                    _ => panic!("unexpected map tile {first_after_boxes}"),
                }
            }
            _ => panic!("unexpected left map tile"),
        }
    }

    fn try_move_right(&mut self) {
        let new_robot = (self.robot.0, self.robot.1 + 1);

        match self.map[new_robot.0][new_robot.1] {
            '#' => (),
            '.' => {
                self.move_robot(new_robot);
            }
            'O' => {
                let first_after_boxes = {
                    let mut i = self.robot.1 + 2;
                    while self.map[self.robot.0][i] == 'O' {
                        i += 1;
                    }
                    i
                };
                match self.map[self.robot.0][first_after_boxes] {
                    '#' => (),
                    '.' => {
                        let right_most_box = first_after_boxes - 1;
                        let boxes_count = right_most_box - self.robot.1;
                        for i in 1..=boxes_count {
                            self.map[self.robot.0][self.robot.1 + 1 + i] = 'O';
                        }
                        self.move_robot(new_robot);
                    }
                    _ => panic!("unexpected map tile {first_after_boxes}"),
                }
            }
            _ => panic!("unexpected left map tile"),
        }
    }
    fn try_move_up(&mut self) {
        let new_robot = (self.robot.0 - 1, self.robot.1);

        match self.map[new_robot.0][new_robot.1] {
            '#' => (),
            '.' => {
                self.move_robot(new_robot);
            }
            'O' => {
                let first_after_boxes = {
                    let mut i = self.robot.0 - 2;
                    while self.map[i][self.robot.1] == 'O' {
                        i -= 1;
                    }
                    i
                };
                match self.map[first_after_boxes][self.robot.1] {
                    '#' => (),
                    '.' => {
                        let up_most_box = first_after_boxes + 1;
                        let boxes_count = self.robot.0 - up_most_box;
                        for i in 1..=boxes_count {
                            self.map[new_robot.0 - i][self.robot.1] = 'O';
                        }
                        self.move_robot(new_robot);
                    }
                    _ => panic!("unexpected map tile {first_after_boxes}"),
                }
            }
            _ => panic!("unexpected left map tile"),
        }
    }

    fn try_move_down(&mut self) {
        let new_robot = (self.robot.0 + 1, self.robot.1);

        match self.map[new_robot.0][new_robot.1] {
            '#' => (),
            '.' => {
                self.move_robot(new_robot);
            }
            'O' => {
                let first_after_boxes = {
                    let mut i = self.robot.0 + 2;
                    while self.map[i][self.robot.1] == 'O' {
                        i += 1;
                    }
                    i
                };
                match self.map[first_after_boxes][self.robot.1] {
                    '#' => (),
                    '.' => {
                        let lowest_box = first_after_boxes - 1;
                        let boxes_count = lowest_box - self.robot.0;
                        for i in 1..=boxes_count {
                            self.map[new_robot.0 + i][self.robot.1] = 'O';
                        }
                        self.move_robot(new_robot);
                    }
                    _ => panic!("unexpected map tile {first_after_boxes}"),
                }
            }
            _ => panic!("unexpected left map tile"),
        }
    }

    fn make_move(&mut self, direction: char) {
        match direction {
            '^' => {
                self.try_move_up();
            }
            'v' => {
                self.try_move_down();
            }
            '<' => {
                self.try_move_left();
            }
            '>' => {
                self.try_move_right();
            }
            _ => panic!("unknown direction"),
        }
    }

    fn gps_sum(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if let 'O' = self.map[i][j] {
                    sum += 100 * i + j;
                }
            }
        }
        sum
    }
}

fn parse_input() -> (Map, String) {
    let input = fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string();
    let (map, moves) = input.split_once("\n\n").unwrap();
    let map = map
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<_>>>();
    (map, moves.replace("\n", "").to_owned())
}

fn find_robot(map: &Map) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                return (y, x);
            }
        }
    }

    panic!("no robot found");
}

pub fn part1() {
    let (map, moves) = parse_input();
    let robot = find_robot(&map);
    let mut state = State { map, robot };

    moves.chars().for_each(|c| state.make_move(c));

    let s = state.gps_sum();
    println!("{s}");
}
