use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day11() {
    let f: File = File::open("data/day11.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let stones = lines[0]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // Create a map which maps (x, y) to z, and tells us that for a stone engraved with x, in y blinks it will have
    // proliferated into z stones. This will saves us doing the same calculations over and over again.
    let mut stone_states: HashMap<(u64, u32), u64> = HashMap::new();

    let mut part1_count: u64 = 0;
    for stone in &stones {
        part1_count += count_stones(*stone, 25, &mut stone_states);
    }
    println!("Day 11 part 1: {}", part1_count);

    let mut part2_count: u64 = 0;
    for stone in &stones {
        part2_count += count_stones(*stone, 75, &mut stone_states);
    }
    println!("Day 11 part 2: {}", part2_count);
}

// Given a single stone, count how many stones it will have proliferated into after the specified number of blinks.
fn count_stones(
    stone: u64,
    blinks_remaining: u32,
    stone_states: &mut HashMap<(u64, u32), u64>,
) -> u64 {
    // First check our state map, maybe we'll get lucky
    if stone_states.contains_key(&(stone, blinks_remaining)) {
        return *stone_states.get(&(stone, blinks_remaining)).unwrap();
    }

    // Now perform a blink
    let mut stones_post_blink: Vec<u64> = vec![];
    if stone == 0 {
        stones_post_blink.push(1);
    } else {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            stones_post_blink.push(stone_str[..stone_str.len() / 2].parse().unwrap());
            stones_post_blink.push(stone_str[stone_str.len() / 2..].parse().unwrap());
        } else {
            stones_post_blink.push(stone * 2024);
        }
    }

    let mut count: u64 = 0;
    if blinks_remaining > 1 {
        // We have more blinks to do
        for next_stone in stones_post_blink {
            count += count_stones(next_stone, blinks_remaining - 1, stone_states);
        }
    } else {
        // That's it
        count = stones_post_blink.len() as u64;
    }

    // Record what we've learnt and return the count
    stone_states.insert((stone, blinks_remaining), count);
    count
}
