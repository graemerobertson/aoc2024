use std::fs::File;
use std::io::{self, BufRead, BufReader};

enum Operator {
    Plus,
    Multiply,
    Concatenate,
}

fn solve_equation(
    current_value: u64,
    remaining_equation: &[u64],
    next_operator: Operator,
    target: u64,
    concatentation_enabled: bool,
) -> bool {
    let new_value = match next_operator {
        Operator::Plus => current_value + remaining_equation[0],
        Operator::Multiply => current_value * remaining_equation[0],
        Operator::Concatenate => {
            if !concatentation_enabled {
                return false;
            } else {
                format!("{}{}", current_value, remaining_equation[0])
                    .parse::<u64>()
                    .unwrap()
            }
        }
    };

    if remaining_equation.len() == 1 {
        return new_value == target;
    } else {
        for next_operator in [Operator::Plus, Operator::Multiply, Operator::Concatenate] {
            if solve_equation(
                new_value,
                &remaining_equation[1..],
                next_operator,
                target,
                concatentation_enabled,
            ) {
                return true;
            }
        }
    }
    false
}

pub(crate) fn day07() {
    let f: File = File::open("data/day07.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut part1_sum: u64 = 0;
    let mut part2_sum: u64 = 0;
    for line in &lines {
        let equation = line
            .split(" ")
            .map(|x| x.trim().trim_end_matches(":").parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let target = equation[0];

        for operator in [Operator::Plus, Operator::Multiply] {
            if solve_equation(equation[1], &equation[2..], operator, target, false) {
                part1_sum += target;
                break;
            }
        }

        for operator in [Operator::Plus, Operator::Multiply, Operator::Concatenate] {
            if solve_equation(equation[1], &equation[2..], operator, target, true) {
                part2_sum += target;
                break;
            }
        }
    }

    println!("Day 7 part 1: {}", part1_sum);
    println!("Day 7 part 2: {}", part2_sum);
}
