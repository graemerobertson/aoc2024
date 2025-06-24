use std::fs::File;
use std::io::{self, BufRead, BufReader};

use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;

// Basic simultaneous equations
//
// Return required number of tokens, or 0 if unsolvable
fn calculate_tokens(x_a: i64, y_a: i64, x_b: i64, y_b: i64, x_prize: i64, y_prize: i64) -> i64 {
    let lcm = lcm(x_a, y_a);
    let x_multiplier = lcm / x_a;
    let y_multiplier = lcm / y_a;
    if (x_prize * x_multiplier - y_prize * y_multiplier) % (x_b * x_multiplier - y_b * y_multiplier)
        == 0
    {
        let b_presses = (x_prize * x_multiplier - y_prize * y_multiplier)
            / (x_b * x_multiplier - y_b * y_multiplier);
        let a_presses = (x_prize - (x_b * b_presses)) / x_a;
        return 3 * a_presses + b_presses;
    } else {
        return 0;
    }
}

pub(crate) fn day13() {
    let f: File = File::open("data/day13.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let mut lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    lines.retain(|s| *s != "");

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    for machine in lines.chunks(3) {
        lazy_static! {
            static ref BUTTONS_RE: Regex = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        }
        lazy_static! {
            static ref PRIZE_RE: Regex = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
        }

        let button_a = BUTTONS_RE.captures(&machine[0]).unwrap();
        let x_a = button_a[1].parse::<i64>().unwrap();
        let y_a = button_a[2].parse::<i64>().unwrap();
        let button_b = BUTTONS_RE.captures(&machine[1]).unwrap();
        let x_b = button_b[1].parse::<i64>().unwrap();
        let y_b = button_b[2].parse::<i64>().unwrap();
        let prize = PRIZE_RE.captures(&machine[2]).unwrap();
        let x_prize = prize[1].parse::<i64>().unwrap();
        let y_prize = prize[2].parse::<i64>().unwrap();

        part1_sum += calculate_tokens(x_a, y_a, x_b, y_b, x_prize, y_prize);
        part2_sum += calculate_tokens(
            x_a,
            y_a,
            x_b,
            y_b,
            x_prize + 10000000000000,
            y_prize + 10000000000000,
        );
    }

    println!("Day 13 part 1: {}", part1_sum);
    println!("Day 13 part 2: {}", part2_sum);
}
