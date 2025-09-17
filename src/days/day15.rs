use core::panic;
use std::fs;

const INPUT: &str = "day15input.txt";

type Map = Vec<Vec<char>>;
struct State {
    map: Map,
    robot: (usize, usize),
}

impl State {
    fn _display(&self) {
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                print!("{}", self.map[y][x]);
            }
            println!();
        }
        println!();
    }

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
                if self.map[i][j] == 'O' || self.map[i][j] == '[' {
                    sum += 100 * i + j;
                }
            }
        }
        sum
    }

    fn try_move_left_part2(&mut self) {
        let new_robot = (self.robot.0, self.robot.1 - 1);

        match self.map[new_robot.0][new_robot.1] {
            '#' => (),
            '.' => {
                self.move_robot(new_robot);
            }
            ']' => {
                let first_after_boxes = {
                    let mut i = self.robot.1 - 2;
                    while self.map[self.robot.0][i] == '[' || self.map[self.robot.0][i] == ']' {
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
                            let box_symbol = if (i % 2) == 0 { '[' } else { ']' };
                            self.map[self.robot.0][first_after_boxes + i] = box_symbol;
                        }
                        self.move_robot(new_robot);
                    }
                    _ => panic!("unexpected map tile {first_after_boxes}"),
                }
            }
            _ => panic!("unexpected left map tile"),
        }
    }

    fn try_move_right_part2(&mut self) {
        let new_robot = (self.robot.0, self.robot.1 + 1);

        match self.map[new_robot.0][new_robot.1] {
            '#' => (),
            '.' => {
                self.move_robot(new_robot);
            }
            '[' => {
                let first_after_boxes = {
                    let mut i = self.robot.1 + 2;
                    while self.map[self.robot.0][i] == '[' || self.map[self.robot.0][i] == ']' {
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
                            let box_symbol = if (i % 2) == 1 { '[' } else { ']' };
                            self.map[self.robot.0][self.robot.1 + 1 + i] = box_symbol;
                        }
                        self.move_robot(new_robot);
                    }
                    _ => panic!("unexpected map tile {first_after_boxes}"),
                }
            }
            _ => panic!("unexpected left map tile"),
        }
    }

    fn first_row_after_boxes_up(
        &self,
        mut box_positions_acc: Vec<(usize, usize)>,
        current_start: (usize, usize),
        current_end: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let above_start = (current_start.0 - 1, current_start.1);
        let above_end = (current_end.0 - 1, current_end.1);

        let above_start = match self.map[above_start.0][above_start.1] {
            '[' | '#' => above_start,
            ']' => (above_start.0, above_start.1 - 1),
            '.' => {
                let mut x = above_start.1 + 1;
                while self.map[above_start.0][x] == '.' && x <= above_end.1 {
                    x += 1;
                }
                (above_start.0, x)
            }
            _ => panic!("unexpected map tile"),
        };

        let above_end = match self.map[above_end.0][above_end.1] {
            ']' | '#' => above_end,
            '[' => (above_end.0, above_end.1 + 1),
            '.' => {
                let mut x = above_end.1 - 1;
                while self.map[above_end.0][x] == '.' && x >= above_start.1 {
                    x -= 1;
                }
                (above_end.0, x)
            }
            _ => panic!("unexpected map tile"),
        };

        let mut hit_wall = false;
        let mut box_positions = vec![];
        for x in above_start.1..=above_end.1 {
            match self.map[above_start.0][x] {
                '#' => {
                    hit_wall = true;
                    break;
                }
                '[' | ']' => box_positions.push((above_start.0, x)),
                '.' => (),
                _ => panic!("unexpected map tile"),
            }
        }

        if hit_wall {
            None
        } else if box_positions.is_empty() {
            Some(box_positions_acc)
        } else {
            box_positions_acc.append(&mut box_positions);
            self.first_row_after_boxes_up(box_positions_acc, above_start, above_end)
        }
    }

    fn first_row_after_boxes_down(
        &self,
        mut box_positions_acc: Vec<(usize, usize)>,
        current_start: (usize, usize),
        current_end: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let below_start = (current_start.0 + 1, current_start.1);
        let below_end = (current_end.0 + 1, current_end.1);

        let below_start = match self.map[below_start.0][below_start.1] {
            '[' | '#' => below_start,
            ']' => (below_start.0, below_start.1 - 1),
            '.' => {
                let mut x = below_start.1 + 1;
                while self.map[below_start.0][x] == '.' && x <= below_end.1 {
                    x += 1;
                }
                (below_start.0, x)
            }
            _ => panic!("unknown tile"),
        };

        let below_end = match self.map[below_end.0][below_end.1] {
            '[' => (below_end.0, below_end.1 + 1),
            ']' | '#' => below_end,
            '.' => {
                let mut x = below_end.1 - 1;
                while self.map[below_end.0][x] == '.' && x >= below_start.1 {
                    x -= 1;
                }
                (below_end.0, x)
            }
            _ => panic!("unexpected map tile"),
        };

        let mut hit_wall = false;
        let mut box_positions = vec![];
        for x in below_start.1..=below_end.1 {
            match self.map[below_start.0][x] {
                '#' => {
                    hit_wall = true;
                    break;
                }
                '[' | ']' => box_positions.push((below_start.0, x)),
                '.' => (),
                _ => panic!("unexpected map tile"),
            }
        }

        if hit_wall {
            None
        } else if box_positions.is_empty() {
            Some(box_positions_acc)
        } else {
            box_positions_acc.append(&mut box_positions);
            self.first_row_after_boxes_down(box_positions_acc, below_start, below_end)
        }
    }

    fn try_move_up_part2(&mut self) {
        let new_robot = (self.robot.0 - 1, self.robot.1);

        match self.map[new_robot.0][new_robot.1] {
            '#' => (),
            '.' => {
                self.move_robot(new_robot);
            }
            '[' | ']' => {
                let acc = vec![];
                if let Some(mut to_move) =
                    self.first_row_after_boxes_up(acc, self.robot, self.robot)
                {
                    to_move.reverse();
                    to_move.iter().for_each(|pos| {
                        let symbol = self.map[pos.0][pos.1];
                        self.map[pos.0 - 1][pos.1] = symbol;
                        self.map[pos.0][pos.1] = '.';
                    });
                    self.move_robot(new_robot);
                }
            }
            _ => panic!("unexpected left map tile"),
        }
    }

    fn try_move_down_part2(&mut self) {
        let new_robot = (self.robot.0 + 1, self.robot.1);

        match self.map[new_robot.0][new_robot.1] {
            '#' => (),
            '.' => {
                self.move_robot(new_robot);
            }
            '[' | ']' => {
                let acc = vec![];
                if let Some(mut to_move) =
                    self.first_row_after_boxes_down(acc, self.robot, self.robot)
                {
                    to_move.reverse();
                    to_move.iter().for_each(|pos| {
                        let symbol = self.map[pos.0][pos.1];
                        self.map[pos.0 + 1][pos.1] = symbol;
                        self.map[pos.0][pos.1] = '.';
                    });
                    self.move_robot(new_robot);
                }
            }
            _ => panic!("unexpected left map tile"),
        }
    }

    fn make_move_part2(&mut self, direction: char) {
        match direction {
            '^' => {
                self.try_move_up_part2();
            }
            'v' => {
                self.try_move_down_part2();
            }
            '<' => {
                self.try_move_left_part2();
            }
            '>' => {
                self.try_move_right_part2();
            }
            _ => panic!("unknown direction"),
        }
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

fn widen_map(map: &Map) -> Map {
    let mut new_map: Map = Vec::new();
    for y in 0..map.len() {
        let mut line: Vec<char> = Vec::new();
        for x in 0..map[0].len() {
            match map[y][x] {
                '#' => {
                    line.push('#');
                    line.push('#');
                }
                'O' => {
                    line.push('[');
                    line.push(']');
                }
                '.' => {
                    line.push('.');
                    line.push('.');
                }
                '@' => {
                    line.push('@');
                    line.push('.');
                }
                _ => panic!("unknown tile"),
            }
        }
        new_map.push(line);
    }
    new_map
}

pub fn part2() {
    let (map, moves) = parse_input();
    let map = widen_map(&map);
    let robot = find_robot(&map);
    let mut state = State { map, robot };

    moves.chars().for_each(|c| {
        state.make_move_part2(c);
    });

    let s = state.gps_sum();
    println!("{s}");
}
