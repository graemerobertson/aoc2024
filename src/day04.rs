use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day04() {
    let f: File = File::open("data/day04.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let rows = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut x_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut m_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut a_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut s_locations: HashSet<(i32, i32)> = HashSet::new();
    for (row_index, row) in rows.iter().enumerate() {
        for col_index in 0..row.len() {
            let c = row.chars().nth(col_index).unwrap();
            if c == 'X' {
                x_locations.insert((row_index as i32, col_index as i32));
            } else if c == 'M' {
                m_locations.insert((row_index as i32, col_index as i32));
            } else if c == 'A' {
                a_locations.insert((row_index as i32, col_index as i32));
            } else if c == 'S' {
                s_locations.insert((row_index as i32, col_index as i32));
            }
        }
    }

    // Search for instances of XMAS from the Xs
    let mut part1_count = 0;
    for x in x_locations.iter() {
        for direction in [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            if m_locations.contains(&(x.0 + direction.0, x.1 + direction.1))
                && a_locations.contains(&(x.0 + direction.0 * 2, x.1 + direction.1 * 2))
                && s_locations.contains(&(x.0 + direction.0 * 3, x.1 + direction.1 * 3))
            {
                part1_count += 1;
            }
        }
    }
    println!("Day 4 part 1: {}", part1_count);

    // Search for instances of XMAS from the As
    let mut part2_count = 0;
    for a in a_locations.iter() {
        if (m_locations.contains(&(a.0 + 1, a.1 + 1)) && s_locations.contains(&(a.0 - 1, a.1 - 1))
            || s_locations.contains(&(a.0 + 1, a.1 + 1))
                && m_locations.contains(&(a.0 - 1, a.1 - 1)))
            && (m_locations.contains(&(a.0 + 1, a.1 - 1))
                && s_locations.contains(&(a.0 - 1, a.1 + 1))
                || s_locations.contains(&(a.0 + 1, a.1 - 1))
                    && m_locations.contains(&(a.0 - 1, a.1 + 1)))
        {
            part2_count += 1;
        }
    }
    println!("Day 4 part 2: {}", part2_count);
}
