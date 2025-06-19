use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day03() {
    let f: File = File::open("data/day03.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input = reader.lines().collect::<io::Result<String>>().unwrap();

    // Find valid multiplication instructions
    lazy_static! {
        static ref PART_1_RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    }
    let mut part1_sum = 0;
    for instruction in PART_1_RE.captures_iter(&input) {
        part1_sum +=
            instruction[1].parse::<u32>().unwrap() * instruction[2].parse::<u32>().unwrap();
    }
    println!("Day 3 part 1: {}", part1_sum);

    // Find valid multiplication, do, and don't instructions
    //
    // Name the do and don't matches to make the logic that follows easier
    lazy_static! {
        static ref PART_2_RE: Regex =
            Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(?P<do>do\(\))|(?P<dont>don't\(\))").unwrap();
    }
    let mut part2_sum = 0;
    let mut enabled = true;
    for instruction in PART_2_RE.captures_iter(&input) {
        if instruction.name("dont").is_some() {
            enabled = false;
        } else if instruction.name("do").is_some() {
            enabled = true;
        // Not a do or a don't, so must be a multiplication; check if we're currently enabled and act if so
        } else if enabled {
            part2_sum +=
                instruction[1].parse::<u32>().unwrap() * instruction[2].parse::<u32>().unwrap();
        }
    }
    println!("Day 3 part 2: {}", part2_sum);
}
