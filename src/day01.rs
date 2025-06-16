use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day01() {
    let f: File = File::open("data/day01.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let (mut left_list, mut right_list): (Vec<u32>, Vec<u32>) = lines
        .iter()
        .map(|s| {
            let mut parts = s.split_whitespace().map(|d| d.parse::<u32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip();

    left_list.sort();
    right_list.sort();

    let part1_sum: u32 = left_list
        .iter()
        .zip(&right_list)
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    println!("Day 1 part 1: {}", part1_sum);

    let part2_sum: u32 = left_list
        .iter()
        .map(|a| a * right_list.iter().filter(|&&x| &x == a).count() as u32)
        .sum();
    println!("Day 1 part 2: {}", part2_sum);
}
