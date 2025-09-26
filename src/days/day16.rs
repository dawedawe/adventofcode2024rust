use std::fs;

const INPUT: &str = "day16input.txt";

type Map = Vec<Vec<char>>;
type Pos = (usize, usize);
type Path = Vec<Pos>;

#[derive(Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_clockwise(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
        }
    }
    fn turn_counter_clockwise(&self) -> Self {
        match self {
            Direction::North => Self::West,
            Direction::East => Self::North,
            Direction::South => Self::East,
            Direction::West => Self::South,
        }
    }
}

#[derive(Clone)]
struct Node {
    path: Path,
    facing: Direction,
    score: u64,
}

impl Node {
    fn try_move(&mut self, map: &Map, movement: &Move) -> bool {
        let current_pos = self.path.last().expect("unexpected empty path");
        match movement {
            Move::Forward => {
                let next_pos = match self.facing {
                    Direction::North => (current_pos.0 - 1, current_pos.1),
                    Direction::East => (current_pos.0, current_pos.1 + 1),
                    Direction::South => (current_pos.0 + 1, current_pos.1),
                    Direction::West => (current_pos.0, current_pos.1 - 1),
                };
                let is_circle = self.path.contains(&next_pos);
                let is_possible = matches!(map[next_pos.0][next_pos.1], '.' | 'E');
                if !is_circle && is_possible {
                    self.score += 1;
                    self.path.push(next_pos);
                    true
                } else {
                    false
                }
            }
            Move::TurnClockwise => {
                self.facing = self.facing.turn_clockwise();
                self.score += 1000;
                self.try_move(map, &Move::Forward)
            }
            Move::TurnCounterClockwise => {
                self.facing = self.facing.turn_counter_clockwise();
                self.score += 1000;
                self.try_move(map, &Move::Forward)
            }
        }
    }
}

enum Move {
    Forward,
    TurnClockwise,
    TurnCounterClockwise,
}

fn parse_input() -> Map {
    fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<_>>>()
}
fn is_other_better(q: &[Node], solutions: &[Node], node: &Node) -> bool {
    let solution_is_better = {
        if let Some(best_solution) = solutions.first() {
            best_solution.score < node.score
        } else {
            false
        }
    };
    solution_is_better
        || q.iter()
            .any(|o| o.path.contains(node.path.last().unwrap()) && o.score < node.score)
}

fn find_trails(map: &Map, head: Node) -> Vec<Node> {
    let possible_moves = [Move::TurnClockwise, Move::TurnCounterClockwise];
    let mut solutions = vec![];
    let mut q = vec![];
    q.push(head);
    while let Some(node) = q.pop() {
        let last_pos = node.path.last().expect("unexpected empty path");
        if map[last_pos.0][last_pos.1] == 'E' {
            solutions.push(node);
        } else {
            let mut forward_node = node.clone();
            let is_possible = forward_node.try_move(map, &Move::Forward);
            let other_is_better = is_other_better(&q, &solutions, &forward_node);
            if is_possible && !other_is_better {
                q.push(forward_node);
            }
            for m in possible_moves.iter() {
                let mut node = node.clone();
                let is_possible = node.try_move(map, m);
                let other_is_better = is_other_better(&q, &solutions, &node);
                if is_possible && !other_is_better {
                    q.insert(0, node);
                }
            }
        }
    }

    solutions
}

pub fn part1() {
    let map = parse_input();
    let starting_pos = (map.len() - 2, 1);
    let node = Node {
        path: vec![starting_pos],
        facing: Direction::East,
        score: 0,
    };
    let solutions = find_trails(&map, node);
    let min = solutions.iter().min_by_key(|n| n.score).unwrap();
    println!("{}", min.score);
}

pub fn part2() {
    let map = parse_input();
    let starting_pos = (map.len() - 2, 1);
    let node = Node {
        path: vec![starting_pos],
        facing: Direction::East,
        score: 0,
    };
    let solutions = find_trails(&map, node);
    let min = solutions.iter().min_by_key(|n| n.score).unwrap().score;
    let min_solutions: Vec<Path> = solutions
        .into_iter()
        .filter(|n| n.score == min)
        .map(|n| n.path)
        .collect();
    let mut tiles = min_solutions.concat();
    tiles.sort();
    tiles.dedup();
    println!("min {min}");
    println!("min_solutions {}", min_solutions.len());
    println!("tiles {}", tiles.len());
}
