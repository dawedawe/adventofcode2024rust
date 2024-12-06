use std::fs;

const INPUT: &str = "day04input.txt";

pub fn part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines = input.lines().collect::<Vec<&str>>();

    let mut matrix: Vec<Vec<char>> = Vec::new();
    lines
        .into_iter()
        .for_each(|line| matrix.push(line.chars().collect()));

    let mut count = 0;
    for line_idx in 0..matrix.len() {
        for col_idx in 0..matrix[0].len() {
            count += count_words_starting_at((line_idx as i32, col_idx as i32), &matrix);
        }
    }

    println!("{count}");
}

fn count_words_starting_at(pos: (i32, i32), matrix: &[Vec<char>]) -> i32 {
    let hor_right = hor_right_positions(pos);
    let a = is_xmas(hor_right, matrix);
    let hor_left = hor_left_positions(pos);
    let b = is_xmas(hor_left, matrix);
    let ver_up = vert_up_positions(pos);
    let c = is_xmas(ver_up, matrix);
    let ver_down = vert_down_positions(pos);
    let d = is_xmas(ver_down, matrix);
    let diag_right_up = diag_right_up_positions(pos);
    let e = is_xmas(diag_right_up, matrix);
    let diag_right_down = diag_right_down_positions(pos);
    let f = is_xmas(diag_right_down, matrix);
    let diag_left_up = diag_left_up_positions(pos);
    let g = is_xmas(diag_left_up, matrix);
    let diag_left_down = diag_left_down_positions(pos);
    let h = is_xmas(diag_left_down, matrix);

    [a, b, c, d, e, f, g, h].iter().map(|x| i32::from(*x)).sum()
}

fn is_xmas(positions: Vec<(i32, i32)>, matrix: &[Vec<char>]) -> bool {
    let is_legal = |(line_idx, col_idx): &(i32, i32)| {
        *line_idx >= 0
            && *line_idx < matrix.len() as i32
            && *col_idx >= 0
            && *col_idx < matrix[0].len() as i32
    };
    let chars = positions
        .iter()
        .filter(|pos| is_legal(pos))
        .map(|(line, col)| matrix[*line as usize][*col as usize]);
    let s = String::from_iter(chars);
    s == "XMAS"
}

fn hor_right_positions((line_idx, col_idx): (i32, i32)) -> Vec<(i32, i32)> {
    let range = col_idx..col_idx + 4;
    range.map(|i| (line_idx, i)).collect()
}

fn hor_left_positions((line_idx, col_idx): (i32, i32)) -> Vec<(i32, i32)> {
    let range = (col_idx - 3..=col_idx).rev();
    range.map(|i| (line_idx, i)).collect()
}

fn vert_up_positions((line_idx, col_idx): (i32, i32)) -> Vec<(i32, i32)> {
    let range = (line_idx - 3..=line_idx).rev();
    range.map(|i| (i, col_idx)).collect()
}

fn vert_down_positions((line_idx, col_idx): (i32, i32)) -> Vec<(i32, i32)> {
    let range = line_idx..line_idx + 4;
    range.map(|i| (i, col_idx)).collect()
}

fn diag_right_up_positions((line_idx, col_idx): (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (line_idx, col_idx),
        (line_idx - 1, col_idx + 1),
        (line_idx - 2, col_idx + 2),
        (line_idx - 3, col_idx + 3),
    ]
}

fn diag_right_down_positions((line_idx, col_idx): (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (line_idx, col_idx),
        (line_idx + 1, col_idx + 1),
        (line_idx + 2, col_idx + 2),
        (line_idx + 3, col_idx + 3),
    ]
}

fn diag_left_up_positions((line_idx, col_idx): (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (line_idx, col_idx),
        (line_idx - 1, col_idx - 1),
        (line_idx - 2, col_idx - 2),
        (line_idx - 3, col_idx - 3),
    ]
}

fn diag_left_down_positions((line_idx, col_idx): (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (line_idx, col_idx),
        (line_idx + 1, col_idx - 1),
        (line_idx + 2, col_idx - 2),
        (line_idx + 3, col_idx - 3),
    ]
}
