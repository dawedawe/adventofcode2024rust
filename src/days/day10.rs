use std::{collections::LinkedList, fs};

const INPUT: &str = "day10input.txt";

type Map = Vec<Vec<char>>;
type Pos = (i32, i32); // x, y
type Path = Vec<Pos>;

fn parse_input(s: String) -> Vec<Vec<char>> {
    s.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_trailheads(map: &Map) -> Vec<Pos> {
    let mut trailheads = vec![];
    (0..map.len()).for_each(|y| {
        (0..map[y].len()).for_each(|x| {
            let c = map[y][x];
            if c == '0' {
                trailheads.push((x as i32, y as i32));
            }
        })
    });
    trailheads
}

fn next_steps(map: &Map, (x, y): Pos) -> Vec<Pos> {
    let mut next = vec![];

    let new_y = y - 1;
    if (new_y) >= 0 {
        next.push((x, new_y));
    }

    let new_y = y + 1;
    if new_y < map.len() as i32 {
        next.push((x, new_y));
    }

    let new_x = x - 1;
    if (new_x) >= 0 {
        next.push((x - 1, y));
    }

    let new_x = x + 1;
    if new_x < map[0].len() as i32 {
        next.push((new_x, y));
    }
    next
}

fn find_trails(map: &Map, head: Pos) -> Vec<Path> {
    let mut trails = vec![];
    let mut q: LinkedList<Path> = std::collections::LinkedList::new();
    let initial_path = vec![head];
    q.push_back(initial_path);
    while let Some(v) = q.pop_front() {
        let (last_x, last_y) = v[v.len() - 1];
        if map[last_y as usize][last_x as usize] == '9' {
            trails.push(v);
        } else {
            let last_height = map[last_y as usize][last_x as usize].to_digit(10).unwrap();
            let candidates = next_steps(map, (last_x, last_y));
            for c in candidates {
                let candidate_height = map[c.1 as usize][c.0 as usize].to_digit(10).unwrap();
                if candidate_height == last_height + 1 {
                    let mut new_path = v.clone();
                    new_path.push(c);
                    q.push_back(new_path);
                }
            }
        }
    }

    trails
}

pub fn part1() {
    let input = fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string();
    let map = parse_input(input);
    let heads = find_trailheads(&map);
    let trails = heads
        .into_iter()
        .map(|head| find_trails(&map, head))
        .collect::<Vec<Vec<Path>>>();
    let sum: usize = trails
        .iter()
        .map(|trails| {
            let mut trails: Vec<Pos> = trails.iter().map(|trail| trail[trail.len() - 1]).collect();
            trails.sort();
            trails.dedup();
            trails.len()
        })
        .sum();

    println!("{sum}");
}

pub fn part2() {
    let input = fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string();
    let map = parse_input(input);
    let heads = find_trailheads(&map);
    let sum: usize = heads
        .into_iter()
        .map(|head| find_trails(&map, head).len())
        .sum();
    println!("{sum}");
}
