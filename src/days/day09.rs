use std::fs;

const INPUT: &str = "day09input.txt";

#[derive(Debug)]
struct Entry {
    id: u32,
    file_blocks: Vec<(u32, usize, u32)>, // (id, pos, count)
    free_blocks: u32,
}

fn parse(s: String) -> Vec<Entry> {
    let mut layout: Vec<Entry> = vec![];
    let digits = s.chars().collect::<Vec<char>>();
    let mut idx = 0;
    loop {
        let id = (idx / 2) as u32;
        let file_blocks_count = digits[idx].to_digit(10).unwrap();
        let pos: usize = if layout.is_empty() {
            0
        } else {
            let prev = layout.last().unwrap();
            let last_file_block = prev.file_blocks.last().unwrap();
            last_file_block.1 + last_file_block.2 as usize + prev.free_blocks as usize
        };
        let file_blocks = vec![(id, pos, file_blocks_count)];

        let free_blocks = if idx + 1 < digits.len() {
            digits[idx + 1].to_digit(10).unwrap()
        } else {
            0
        };

        let entry = Entry {
            id,
            file_blocks,
            free_blocks,
        };
        layout.push(entry);

        if idx >= digits.len() - 2 {
            break;
        } else {
            idx += 2;
        }
    }

    layout
}

fn compact_part2(layout: &mut [Entry]) {
    for source_idx in (1..layout.len()).rev() {
        for target_idx in 0..source_idx {
            if layout[target_idx].free_blocks >= layout[source_idx].file_blocks[0].2 {
                let new_pos = layout[target_idx].file_blocks.last().unwrap().1
                    + layout[target_idx].file_blocks.last().unwrap().2 as usize;
                layout[target_idx].free_blocks -= layout[source_idx].file_blocks[0].2;
                layout[source_idx].free_blocks += layout[source_idx].file_blocks[0].2;
                let moved = (
                    layout[source_idx].file_blocks[0].0,
                    new_pos,
                    layout[source_idx].file_blocks[0].2,
                );
                layout[target_idx].file_blocks.push(moved);
                layout[source_idx].file_blocks.remove(0);
                break;
            }
        }
    }
}

fn compact(layout: &mut [Entry]) {
    for i in 0..layout.len() - 1 {
        if layout[i].free_blocks > 0 {
            for j in (i + 1..=layout.len() - 1).rev() {
                if layout[j].file_blocks.is_empty() {
                    continue;
                }
                let movable_blocks = layout[j].file_blocks[0].2.min(layout[i].free_blocks);
                if movable_blocks > 0 {
                    let new_starting_pos = layout[i].file_blocks.last().unwrap().1
                        + layout[i].file_blocks.last().unwrap().2 as usize;
                    layout[i].free_blocks -= movable_blocks;
                    let block_to_move = (layout[j].id, new_starting_pos, movable_blocks);
                    layout[i].file_blocks.push(block_to_move);
                    let left_blocks = layout[j].file_blocks[0].2 - movable_blocks;
                    let left_starting_pos = layout[j].file_blocks[0].1;
                    layout[j].file_blocks.clear();
                    if left_blocks > 0 {
                        let left_block = (layout[j].id, left_starting_pos, left_blocks);
                        layout[j].file_blocks.push(left_block);
                    }
                }

                if layout[i].free_blocks == 0 {
                    break;
                }
            }
        }
    }
}

fn checksum(layout: &[Entry]) -> u64 {
    let mut sum: u64 = 0;
    for entry in layout {
        for (id, position, count) in &entry.file_blocks {
            for pos in *position..*position + *count as usize {
                let c = pos * *id as usize;
                sum += c as u64;
            }
        }
    }
    sum
}

pub fn part1() {
    let input = fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string();
    let mut layout = parse(input);
    compact(&mut layout);
    let c = checksum(&layout);
    println!("{c}");
}

pub fn part2() {
    let input = fs::read_to_string(INPUT)
        .expect("read_to_string failed")
        .trim()
        .to_string();
    let mut layout = parse(input);
    compact_part2(&mut layout);
    let c = checksum(&layout);
    println!("{c}");
}
