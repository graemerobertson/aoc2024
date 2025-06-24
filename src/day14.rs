use std::fs::File;
use std::io::{self, BufRead, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

struct Robot {
    x_start: i32,
    y_start: i32,
    x_velocity: i32,
    y_velocity: i32,
}

pub(crate) fn day14() {
    let f: File = File::open("data/day14.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let grid = vec![vec!['.'; HEIGHT as usize]; WIDTH as usize];
    let mut robots: Vec<Robot> = vec![];
    for line in &lines {
        lazy_static! {
            static ref INPUT_RE: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        }
        let captures = INPUT_RE.captures(&line).unwrap();
        let x_start = captures[1].parse::<i32>().unwrap();
        let y_start = captures[2].parse::<i32>().unwrap();
        let x_velocity = captures[3].parse::<i32>().unwrap();
        let y_velocity = captures[4].parse::<i32>().unwrap();
        robots.push(Robot {
            x_start,
            y_start,
            x_velocity,
            y_velocity,
        });
    }

    let mut number_of_seconds = 0;
    // Search for 6500 seconds (because I know my answer is 6493...)
    while number_of_seconds < 6500 {
        number_of_seconds += 1;
        let mut grid_clone = grid.clone();
        let mut quadrant_counts: Vec<usize> = vec![0; 4];

        for robot in &robots {
            let x_finish =
                ((robot.x_start + robot.x_velocity * number_of_seconds) % WIDTH + WIDTH) % WIDTH;
            let y_finish =
                ((robot.y_start + robot.y_velocity * number_of_seconds) % HEIGHT + HEIGHT) % HEIGHT;

            if x_finish < (WIDTH - 1) / 2 && y_finish < (HEIGHT - 1) / 2 {
                quadrant_counts[0] += 1;
            } else if x_finish > (WIDTH - 1) / 2 && y_finish < (HEIGHT - 1) / 2 {
                quadrant_counts[1] += 1;
            } else if x_finish > (WIDTH - 1) / 2 && y_finish > (HEIGHT - 1) / 2 {
                quadrant_counts[2] += 1;
            } else if x_finish < (WIDTH - 1) / 2 && y_finish > (HEIGHT - 1) / 2 {
                quadrant_counts[3] += 1;
            }
            grid_clone[x_finish as usize][y_finish as usize] = '#';
        }

        if number_of_seconds == 100 {
            println!(
                "Day 14 part 1: {}",
                quadrant_counts.iter().product::<usize>()
            );
        }

        // I originally found the tree by searching for 20 robots along the central lines. I've refined this to 24
        // robots - because that's how many there are in my image, and it reduces the amount of output.
        //
        // (There are 500 robots in total)
        if quadrant_counts.iter().sum::<usize>() < 476 {
            println!("Grid after {} seconds:", number_of_seconds);
            for row in grid_clone {
                let mut row_str = "".to_string();
                for c in row {
                    row_str.push(c);
                }
                println!("{}", row_str);
            }
        }
    }
}
