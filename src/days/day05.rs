use std::fs;

const INPUT: &str = "day05input.txt";

pub fn part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines = input.lines().collect::<Vec<&str>>();
    let rules = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|s| {
            let (a, b) = s.split_once("|").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();
    let updates = lines
        .iter()
        .skip(rules.len() + 1)
        .map(|s| s.split(",").map(|n| n.parse::<usize>().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();

    let sum = updates
        .iter()
        .filter(|update| is_valid_update(update, &rules))
        .map(|v| v[v.len() / 2])
        .sum::<usize>();

    println!("{}", sum);
}

fn is_valid_update(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    for page_idx in 0..(update.len() - 1) {
        let current_page = update[page_idx];
        for next_page_idx in page_idx + 1..update.len() {
            let next_page = update[next_page_idx];
            let invalidating_rule = (next_page, current_page);
            if rules.contains(&invalidating_rule) {
                return false;
            }
        }
    }

    true
}
