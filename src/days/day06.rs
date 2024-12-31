use std::{collections::HashSet, fs};

const INPUT: &str = "day06input.txt";

type Pos = (i32, i32);

type Map = Vec<Vec<u8>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
struct Guard {
    pos: Pos,
    dir: Direction,
}

fn find_start(lines: &Map) -> Guard {
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == b'^' {
                let g = Guard {
                    pos: (j as i32, i as i32),
                    dir: Direction::Up,
                };
                return g;
            }
        }
    }

    panic!("failed to find start")
}

fn patrol(guard: &Guard, map: &Map, acc: &mut HashSet<Pos>) {
    let next = get_next(guard, map);
    if let Some(next_guard) = next {
        acc.insert(next_guard.pos);
        patrol(&next_guard, map, acc);
    }
}

fn get_next(guard: &Guard, map: &Map) -> Option<Guard> {
    let next_pos = match guard.dir {
        Direction::Up => (guard.pos.0, guard.pos.1 - 1),
        Direction::Down => (guard.pos.0, guard.pos.1 + 1),
        Direction::Left => (guard.pos.0 - 1, guard.pos.1),
        Direction::Right => (guard.pos.0 + 1, guard.pos.1),
    };

    let on_map = next_pos.0 >= 0
        && next_pos.0 < map[0].len() as i32
        && next_pos.1 >= 0
        && next_pos.1 < map.len() as i32;
    if !on_map {
        None
    } else if map[next_pos.1 as usize][next_pos.0 as usize] != b'#' {
        Some(Guard {
            pos: next_pos,
            dir: guard.dir,
        })
    } else {
        let turned_guard = Guard {
            pos: guard.pos,
            dir: guard.dir.turn_right(),
        };
        get_next(&turned_guard, map)
    }
}

fn get_map(path: &str) -> Map {
    fs::read_to_string(path)
        .expect("read_to_string failed")
        .lines()
        .map(|line| line.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
}

pub fn part1() {
    let map = get_map(INPUT);
    let guard = find_start(&map);
    let mut acc = HashSet::new();
    acc.insert(guard.pos);
    patrol(&guard, &map, &mut acc);
    println!("{}", acc.len());
}

pub fn part2() {
    let map = get_map(INPUT);
    let guard = find_start(&map);
    let mut acc = HashSet::new();
    acc.insert(guard.pos);
    patrol(&guard, &map, &mut acc);
    acc.remove(&guard.pos);
    let options = acc
        .iter()
        .map(|candidate| {
            let mut map_altered = map.clone();
            map_altered[candidate.1 as usize][candidate.0 as usize] = b'#';
            i32::from(is_loop(&guard, &map_altered))
        })
        .sum::<i32>();

    println!("{}", options);
}

fn is_loop(guard: &Guard, map: &Map) -> bool {
    fn run(pos: &Guard, map: &Map, acc: &mut HashSet<Guard>) -> bool {
        let next = get_next(pos, map);
        if let Some(next_guard) = next {
            if acc.insert(next_guard.clone()) {
                return run(&next_guard, map, acc);
            } else {
                return true;
            }
        }

        false
    }

    let mut acc = HashSet::new();
    acc.insert((*guard).clone());

    run(guard, map, &mut acc)
}
