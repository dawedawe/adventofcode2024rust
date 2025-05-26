use std::{fs, vec};

const INPUT: &str = "day12input.txt";

pub fn parse_input() -> Map {
    let input = fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string();
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

type Pos = (i32, i32); // (x, y)
type Map = Vec<Vec<char>>;

#[derive(PartialEq)]
enum Orientation {
    Top,
    Right,
    Bottom,
    Left,
}

struct Side {
    orientation: Orientation,
    pos: Pos,
}

struct Region {
    plant: char,
    positions: Vec<Pos>,
}

impl Region {
    fn pos_are_adjacent(pos1: &Pos, pos2: &Pos) -> bool {
        (pos1.0.saturating_sub(1), pos1.1) == *pos2
            || (pos1.0.saturating_add(1), pos1.1) == *pos2
            || (pos1.0, pos1.1.saturating_sub(1)) == *pos2
            || (pos1.0, pos1.1.saturating_add(1)) == *pos2
    }

    fn is_adjacent_to_pos(&self, pos: &Pos) -> bool {
        self.positions
            .iter()
            .any(|p| Region::pos_are_adjacent(p, pos))
    }

    fn is_adjacent_to_other(&self, other: &Region) -> bool {
        other
            .positions
            .iter()
            .any(|other_pos| self.is_adjacent_to_pos(other_pos))
    }

    fn should_merge(&self, other: &Region) -> bool {
        self.plant == other.plant && self.is_adjacent_to_other(other)
    }

    fn add_positins(&mut self, to_add: Vec<Pos>) {
        to_add.iter().for_each(|p| self.positions.push(*p));
    }

    fn is_different_region_or_outside_map(&self, pos: &Pos, map: &Map) -> bool {
        let is_outside =
            pos.0 < 0 || pos.0 >= map[0].len() as i32 || pos.1 < 0 || pos.1 >= map.len() as i32;
        let is_different = !is_outside && map[pos.1 as usize][pos.0 as usize] != self.plant;

        is_outside || is_different
    }

    fn perimeter(&self, map: &Map) -> u32 {
        let mut perimeter = 0;
        for pos in &self.positions {
            let upper = (pos.0, pos.1 - 1);
            let lower = (pos.0, pos.1 + 1);
            let left = (pos.0 - 1, pos.1);
            let right = (pos.0 + 1, pos.1);
            let neighbours = [upper, lower, left, right];
            neighbours.iter().for_each(|n| {
                if self.is_different_region_or_outside_map(n, map) {
                    perimeter += 1;
                }
            });
        }
        perimeter
    }

    fn price(&self, map: &Map) -> u32 {
        self.positions.len() as u32 * self.perimeter(map)
    }

    fn price_part2(&self, map: &Map) -> u32 {
        self.positions.len() as u32 * self.sides(map)
    }

    fn pos_sides(&self, pos: &Pos, map: &Map) -> Vec<Side> {
        let mut sides = vec![];
        [
            ((pos.0, pos.1 - 1), Orientation::Top),
            ((pos.0, pos.1 + 1), Orientation::Bottom),
            ((pos.0 - 1, pos.1), Orientation::Left),
            ((pos.0 + 1, pos.1), Orientation::Right),
        ]
        .into_iter()
        .for_each(|(p, o)| {
            if self.is_different_region_or_outside_map(&p, map) {
                sides.push(Side {
                    orientation: o,
                    pos: *pos,
                })
            }
        });

        sides
    }

    fn sides(&self, map: &Map) -> u32 {
        let mut sides_count = 0;
        let mut all_sides = vec![];
        for pos in &self.positions {
            let sides_of_pos = self.pos_sides(pos, map);
            sides_of_pos.into_iter().for_each(|s| {
                if !Region::is_continuation_side(&all_sides, &s) {
                    sides_count += 1;
                }
                all_sides.push(s)
            });
        }

        sides_count
    }

    fn is_continuation_side(all_sides: &[Side], new_side: &Side) -> bool {
        let upper = (new_side.pos.0, new_side.pos.1 - 1);
        let lower = (new_side.pos.0, new_side.pos.1 + 1);
        let left = (new_side.pos.0 - 1, new_side.pos.1);
        let right = (new_side.pos.0 + 1, new_side.pos.1);

        match new_side.orientation {
            Orientation::Top => all_sides.iter().any(|side| {
                side.orientation == Orientation::Top && (side.pos == left || side.pos == right)
            }),
            Orientation::Right => all_sides.iter().any(|side| {
                side.orientation == Orientation::Right && (side.pos == upper || side.pos == lower)
            }),
            Orientation::Bottom => all_sides.iter().any(|side| {
                side.orientation == Orientation::Bottom && (side.pos == left || side.pos == right)
            }),
            Orientation::Left => all_sides.iter().any(|side| {
                side.orientation == Orientation::Left && (side.pos == upper || side.pos == lower)
            }),
        }
    }
}

fn find_regions(map: &Map) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];
    let mut y = 0;
    let mut x;
    while y < map.len() {
        x = 0;
        while x < map[y].len() {
            let current_plant = map[y][x];
            let mut current_reg_positions = vec![(x as i32, y as i32)];
            loop {
                x += 1;
                if x == map[y].len() {
                    break;
                }
                let next_plant = map[y][x];
                if current_plant == next_plant {
                    current_reg_positions.push((x as i32, y as i32));
                } else {
                    break;
                }
            }
            let mut region_candidate = Region {
                plant: current_plant,
                positions: current_reg_positions,
            };

            let to_merge: Vec<Region> = regions
                .extract_if(.., |r| r.should_merge(&region_candidate))
                .collect();
            for t in to_merge {
                region_candidate.add_positins(t.positions);
            }
            regions.push(region_candidate);
        }
        y += 1;
    }

    regions
}

pub fn part1() {
    let map = parse_input();
    let regions = find_regions(&map);
    let sum: u32 = regions.iter().map(|r| r.price(&map)).sum();
    println!("{}", sum);
}

pub fn part2() {
    let map = parse_input();
    let regions = find_regions(&map);
    let sum: u32 = regions.iter().map(|r| r.price_part2(&map)).sum();
    println!("{}", sum);
}
