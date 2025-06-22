use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day10() {
    let f: File = File::open("data/day10.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut trailheads: HashSet<(i32, i32)> = HashSet::new();

    for (row_index, line) in lines.iter().enumerate() {
        let mut row: Vec<i32> = Vec::new();
        for (col_index, c) in line.chars().enumerate() {
            let value = c.to_digit(10).unwrap() as i32;
            row.push(value);
            if c == '0' {
                // This is a trailhead
                trailheads.insert((col_index as i32, row_index as i32));
            }
        }

        map.push(row);
    }

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    for trailhead in trailheads {
        // count_hiking_trails walks through the map on all possible routes.
        // For part 1, we just count the number of peaks (9s) we can reach - and to do that, count_hiking_trails
        // will insert the peaks into a HashSet, which will ensure we only count each peak once.
        // For part 2, we need the actual count of hiking trails, so we return that value.
        let mut peaks: HashSet<(i32, i32)> = HashSet::new();
        part2_sum += count_hiking_trails(&map, trailhead, 0, &mut peaks);
        part1_sum += peaks.len();
    }

    println!("Day 10 part 1: {}", part1_sum);
    println!("Day 10 part 2: {}", part2_sum);
}

fn count_hiking_trails(
    map: &Vec<Vec<i32>>,
    current_location: (i32, i32),
    current_elevation: i32,
    peaks: &mut HashSet<(i32, i32)>,
) -> usize {
    let mut count: usize = 0;
    // For each adjacent point...
    for delta in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let next_x = current_location.0 + delta.0;
        let next_y = current_location.1 + delta.1;

        // If this point is within bounds and has an elevation of current_elevation + 1...
        if next_x >= 0
            && next_x < map[0].len() as i32
            && next_y >= 0
            && next_y < map.len() as i32
            && map[next_y as usize][next_x as usize] == current_elevation + 1
        {
            if current_elevation + 1 == 9 {
                // We've reached the end of a trail
                peaks.insert((next_x, next_y));
                count += 1;
            } else {
                // Continue hiking
                count += count_hiking_trails(map, (next_x, next_y), current_elevation + 1, peaks);
            }
        }
    }
    count
}
