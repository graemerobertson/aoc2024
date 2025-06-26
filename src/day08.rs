use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use num::integer::gcd;

fn in_grid(location: (i32, i32), width: i32, height: i32) -> bool {
    location.0 >= 0 && location.0 < width && location.1 >= 0 && location.1 < height
}

pub(crate) fn day08() {
    let f: File = File::open("data/day08.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (row_index, line) in lines.iter().enumerate() {
        for (col_index, c) in line.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_default()
                    .push((col_index as i32, row_index as i32));
            }
        }
    }

    let width = lines[0].len() as i32;
    let height = lines.len() as i32;
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for frequency in antennas.keys() {
        for (i, antenna_a) in antennas[frequency].iter().enumerate() {
            for antenna_b in antennas[frequency].iter().skip(i + 1) {
                let dx = antenna_b.0 - antenna_a.0;
                let dy = antenna_b.1 - antenna_a.1;
                let antinode = (antenna_b.0 + dx, antenna_b.1 + dy);
                if in_grid(antinode, width, height) {
                    antinodes.insert(antinode);
                }
                let antinode = (antenna_a.0 - dx, antenna_a.1 - dy);
                if in_grid(antinode, width, height) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    println!("Day 8 part 1: {}", antinodes.len());

    let mut antinodes_part2: HashSet<(i32, i32)> = HashSet::new();

    for frequency in antennas.keys() {
        for (i, antenna_a) in antennas[frequency].iter().enumerate() {
            for antenna_b in antennas[frequency].iter().skip(i + 1) {
                let dx = antenna_b.0 - antenna_a.0;
                let dy = antenna_b.1 - antenna_a.1;
                let gcd_value = gcd(dx, dy);
                let dx = dx / gcd_value;
                let dy = dy / gcd_value;

                for direction in [1, -1] {
                    let mut count = 0;
                    loop {
                        let antinode = (
                            antenna_b.0 + count * direction * dx,
                            antenna_b.1 + count * direction * dy,
                        );
                        if in_grid(antinode, width, height) {
                            antinodes_part2.insert(antinode);
                            count += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("Day 8 part 2: {}", antinodes_part2.len());
}
