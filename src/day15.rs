use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn calculate_points_to_move(
    start_location: (isize, isize),
    movement: (isize, isize),
    grid: &Vec<Vec<char>>,
) -> Option<Vec<(isize, isize)>> {
    let mut points_to_move: Vec<(isize, isize)> = vec![start_location];

    match grid[(start_location.0 + movement.0) as usize][(start_location.1 + movement.1) as usize] {
        'O' => {
            if let Some(points) = calculate_points_to_move(
                (start_location.0 + movement.0, start_location.1 + movement.1),
                movement,
                grid,
            ) {
                points_to_move.extend_from_slice(&points);
            } else {
                points_to_move.clear();
            }
        }
        '#' => {
            points_to_move.clear();
        }
        '[' => {
            if let Some(left_branch_points) = calculate_points_to_move(
                (start_location.0 + movement.0, start_location.1 + movement.1),
                movement,
                grid,
            ) {
                // We've found the left half of a big block. If we're moving up or down, then we also need to consider
                // the right half.
                if movement.0 != 0 {
                    if let Some(right_branch_points) = calculate_points_to_move(
                        (
                            start_location.0 + movement.0,
                            start_location.1 + movement.1 + 1,
                        ),
                        movement,
                        grid,
                    ) {
                        points_to_move.extend_from_slice(&left_branch_points);
                        points_to_move.extend_from_slice(&right_branch_points);
                    } else {
                        points_to_move.clear();
                    }
                } else {
                    points_to_move.extend_from_slice(&left_branch_points);
                }
            } else {
                points_to_move.clear();
            }
        }
        ']' => {
            if let Some(right_branch_points) = calculate_points_to_move(
                (start_location.0 + movement.0, start_location.1 + movement.1),
                movement,
                grid,
            ) {
                // We've found the left half of a big block. If we're moving up or down, then we also need to consider
                // the right half.
                if movement.0 != 0 {
                    if let Some(left_branch_points) = calculate_points_to_move(
                        (
                            start_location.0 + movement.0,
                            start_location.1 + movement.1 - 1,
                        ),
                        movement,
                        grid,
                    ) {
                        points_to_move.extend_from_slice(&right_branch_points);
                        points_to_move.extend_from_slice(&left_branch_points);
                    } else {
                        points_to_move.clear();
                    }
                } else {
                    points_to_move.extend_from_slice(&right_branch_points);
                }
            } else {
                points_to_move.clear();
            }
        }
        _ => {}
    }

    if points_to_move.is_empty() {
        None
    } else {
        Some(points_to_move)
    }
}

fn move_points(
    points_to_move: &mut Vec<(isize, isize)>,
    movement: (isize, isize),
    grid: &Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    // Assume that the first point is the robot's current position. We want to sort the rest of the points to move by
    // distance from the robot - and move the furthest away points first.
    let start = points_to_move[0];
    if movement.0 == 0 {
        points_to_move
            .sort_by(|a: &(isize, isize), b| ((b.1 - start.1).abs()).cmp(&(a.1 - start.1).abs()));
    } else {
        points_to_move
            .sort_by(|a: &(isize, isize), b| ((b.0 - start.0).abs()).cmp(&(a.0 - start.0).abs()));
    }

    let mut new_grid: Vec<Vec<char>> = grid.clone();
    for point in points_to_move.into_iter() {
        // Move point.
        new_grid[(point.0 + movement.0) as usize][(point.1 + movement.1) as usize] =
            grid[point.0 as usize][point.1 as usize];
        // Set the vacated spot to empty. This is why our earlier sorting operation was important; we don't know if
        // anything else is moving into this spot, but after sorting, we can be confident that if it is then it'll
        // happen later.
        new_grid[(point.0) as usize][(point.1) as usize] = '.';
    }
    new_grid
}

fn update_grid(
    robot_location: &mut (isize, isize),
    instruction: (isize, isize),
    grid: &mut Vec<Vec<char>>,
) {
    let points_to_move = calculate_points_to_move(robot_location.clone(), instruction, &grid);
    if let Some(mut points) = points_to_move {
        *robot_location = (
            robot_location.0 + instruction.0,
            robot_location.1 + instruction.1,
        );
        *grid = move_points(&mut points, instruction, &grid)
    }
}

pub(crate) fn day15() {
    let f: File = File::open("data/day15.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let start_of_instructions = lines.iter().position(|l| *l == "").unwrap();
    let grid_lines = &lines[..start_of_instructions];
    let instruction_lines = &lines[start_of_instructions + 1..];

    let mut part1_grid: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; start_of_instructions];
    let mut part2_grid: Vec<Vec<char>> = vec![vec!['.'; lines[0].len() * 2]; start_of_instructions];
    let mut part1_robot_location: (isize, isize) = (0, 0);
    let mut part2_robot_location: (isize, isize) = (0, 0);
    for (row_index, line) in grid_lines.iter().take(start_of_instructions).enumerate() {
        for (col_index, c) in line.chars().enumerate() {
            part1_grid[row_index][col_index] = c;
            match c {
                'O' => {
                    part2_grid[row_index][2 * col_index] = '[';
                    part2_grid[row_index][2 * col_index + 1] = ']';
                }
                '@' => {
                    part1_robot_location = (row_index as isize, col_index as isize);
                    part2_robot_location = (row_index as isize, 2 * col_index as isize);
                    part2_grid[row_index][2 * col_index] = '@';
                    part2_grid[row_index][2 * col_index + 1] = '.';
                }
                _ => {
                    part2_grid[row_index][2 * col_index] = c;
                    part2_grid[row_index][2 * col_index + 1] = c;
                }
            }
        }
    }
    let mut instructions: Vec<(isize, isize)> = vec![];
    for line in instruction_lines {
        for c in line.chars() {
            match c {
                '>' => instructions.push((0, 1)),
                '<' => instructions.push((0, -1)),
                '^' => instructions.push((-1, 0)),
                'v' => instructions.push((1, 0)),
                _ => panic!(),
            }
        }
    }

    for instruction in instructions {
        update_grid(&mut part1_robot_location, instruction, &mut part1_grid);
        update_grid(&mut part2_robot_location, instruction, &mut part2_grid);
    }

    let mut part1_sum = 0;
    for (row_index, row) in part1_grid.iter().enumerate() {
        for (col_index, c) in row.iter().enumerate() {
            if *c == 'O' {
                part1_sum += 100 * row_index + col_index;
            }
        }
    }

    let mut part2_sum = 0;
    for (row_index, row) in part2_grid.iter().enumerate() {
        for (col_index, c) in row.iter().enumerate() {
            if *c == '[' {
                part2_sum += 100 * row_index + col_index;
            }
        }
    }

    println!("Day 15 part 1: {}", part1_sum);
    println!("Day 15 part 2: {}", part2_sum);
}
