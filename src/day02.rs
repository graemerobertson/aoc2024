use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn safe_report(report: &[i32]) -> bool {
    // Gradually decreasing candidate
    if report[0] > report[1] {
        if report.windows(2).all(|l| l[0] > l[1] && l[0] <= l[1] + 3) {
            return true;
        }
    // Gradually increasing candidate
    } else if report[0] < report[1] {
        if report.windows(2).all(|l| l[0] < l[1] && l[0] >= l[1] - 3) {
            return true;
        }
    }
    false
}

pub(crate) fn day02() {
    let f: File = File::open("data/day02.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut part1_count = 0;
    let mut unsafe_reports: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let report = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if safe_report(&report) {
            part1_count += 1;
        } else {
            unsafe_reports.push(report);
        }
    }
    println!("Day 2 part 1: {}", part1_count);

    // For each unsafe report, try removing one element
    let mut part2_count = part1_count;
    for report in unsafe_reports {
        for i in 0..report.len() {
            let mut report_delta = report.clone();
            report_delta.remove(i);
            if safe_report(&report_delta) {
                part2_count += 1;
                break;
            }
        }
    }

    println!("Day 2 part 2: {}", part2_count);
}
