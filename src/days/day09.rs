use std::fs;

const INPUT: &str = "day09input.txt";

#[derive(Debug)]
struct Entry {
    id: u32,
    file_blocks: Vec<(u32, u32)>, // (id, count)
    free_blocks: u32,
}

fn parse(s: String) -> Vec<Entry> {
    let mut layout = vec![];
    let digits = s.chars().collect::<Vec<char>>();
    for (id, w) in digits.chunks(2).enumerate() {
        let id = id as u32;
        let file_blocks_count = w[0].to_digit(10).unwrap();

        let file_blocks = vec![(id, file_blocks_count)];

        let free_blocks = if w.len() == 2 {
            w[1].to_digit(10).unwrap()
        } else {
            0
        };
        let entry = Entry {
            id,
            file_blocks,
            free_blocks,
        };
        layout.push(entry);
    }

    layout
}

fn compact(layout: &mut [Entry]) {
    for i in 0..layout.len() - 1 {
        if layout[i].free_blocks > 0 {
            for j in (i + 1..=layout.len() - 1).rev() {
                if layout[j].file_blocks.is_empty() {
                    continue;
                }
                let movable_blocks = layout[j].file_blocks[0].1.min(layout[i].free_blocks);
                if movable_blocks > 0 {
                    layout[i].free_blocks -= movable_blocks;
                    let block_to_move = (layout[j].id, movable_blocks);
                    layout[i].file_blocks.push(block_to_move);
                    let left_blocks = layout[j].file_blocks[0].1 - movable_blocks;
                    layout[j].file_blocks.clear();
                    if left_blocks > 0 {
                        let left_block = (layout[j].id, left_blocks);
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
    let mut position: u32 = 0;
    let mut sum: u64 = 0;
    for entry in layout {
        for (id, count) in &entry.file_blocks {
            for pos in position..position + *count {
                let c = pos * *id;
                sum += c as u64;
            }
            position += *count;
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
