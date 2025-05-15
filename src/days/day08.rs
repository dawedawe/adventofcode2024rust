use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};

const INPUT: &str = "day08input.txt";

// (y, x)
type Coords = (i32, i32);

struct Spec {
    antennas: HashMap<char, Vec<Coords>>,
    max_y: usize,
    max_x: usize,
}

fn is_valid_coord(coord: Coords, max_y: i32, max_x: i32) -> bool {
    coord.0 >= 0 && coord.0 <= max_y && coord.1 >= 0 && coord.1 <= max_x
}

fn get_pairs(coords: &[Coords]) -> Vec<(Coords, Coords)> {
    let mut pairs = vec![];
    for i in 0..coords.len() - 1 {
        for j in i + 1..coords.len() {
            let pair = (coords[i], coords[j]);
            pairs.push(pair);
        }
    }
    pairs
}

fn get_antinodes(a1: Coords, a2: Coords) -> (Coords, Coords) {
    let antinode1 = (a1.0 - (a2.0 - a1.0), a1.1 - (a2.1 - a1.1));
    let antinode2 = (a2.0 + (a2.0 - a1.0), a2.1 + (a2.1 - a1.1));
    (antinode1, antinode2)
}

fn get_antinodes_part2(a1: Coords, a2: Coords, spec: &Spec) -> Vec<Coords> {
    let mut antinodes = vec![a1];
    let (y_diff, x_diff) = ((a2.0 - a1.0), (a2.1 - a1.1));

    loop {
        let ref_coord = antinodes[antinodes.len() - 1];
        let candidate = (ref_coord.0 - y_diff, ref_coord.1 - x_diff);
        if is_valid_coord(candidate, spec.max_y as i32, spec.max_x as i32) {
            antinodes.push(candidate);
        } else {
            break;
        }
    }

    antinodes.push(a2);

    loop {
        let ref_coord = antinodes[antinodes.len() - 1];

        let candidate = (ref_coord.0 + y_diff, ref_coord.1 + x_diff);
        if is_valid_coord(candidate, spec.max_y as i32, spec.max_x as i32) {
            antinodes.push(candidate);
        } else {
            break;
        }
    }

    antinodes
}

fn calc_antinodes(spec: &Spec) -> usize {
    let mut antinodes_set: HashSet<Coords> = HashSet::new();

    for a in spec.antennas.iter() {
        let pairs = get_pairs(a.1);
        pairs.iter().for_each(|p| {
            let antinodes = get_antinodes(p.0, p.1);
            if is_valid_coord(antinodes.0, spec.max_y as i32, spec.max_x as i32) {
                antinodes_set.insert(antinodes.0);
            }
            if is_valid_coord(antinodes.1, spec.max_y as i32, spec.max_x as i32) {
                antinodes_set.insert(antinodes.1);
            }
        });
    }
    antinodes_set.len()
}

fn calc_antinodes_part2(spec: &Spec) -> usize {
    let mut antinodes_set: HashSet<Coords> = HashSet::new();

    for a in spec.antennas.iter() {
        let pairs = get_pairs(a.1);
        pairs.iter().for_each(|p| {
            let antinodes = get_antinodes_part2(p.0, p.1, spec);
            antinodes.into_iter().for_each(|n| {
                antinodes_set.insert(n);
            });
        });
    }
    antinodes_set.len() // + spec.antennas.len()
}

fn parse_input(input: String) -> Spec {
    let lines = input.lines().collect::<Vec<&str>>();
    let max_y = lines.len() - 1;
    let max_x = lines[0].len() - 1;
    let mut antennas: HashMap<char, Vec<Coords>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                if let std::collections::hash_map::Entry::Vacant(e) = antennas.entry(c) {
                    e.insert(vec![(y as i32, x as i32)]);
                } else {
                    antennas.get_mut(&c).unwrap().push((y as i32, x as i32));
                }
            }
        }
    }

    let antennas_filtered = antennas.into_iter().filter(|e| e.1.len() >= 2);
    let mut antennas: HashMap<char, Vec<Coords>> = HashMap::new();
    antennas_filtered.into_iter().for_each(|x| {
        antennas.insert(x.0, x.1);
    });

    Spec {
        antennas,
        max_y,
        max_x,
    }
}

pub fn part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let spec = parse_input(input);
    let r = calc_antinodes(&spec);
    println!("{}", r);
}

pub fn part2() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let spec = parse_input(input);
    let r = calc_antinodes_part2(&spec);
    println!("{}", r);
}
