use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, PartialEq)]
enum Point {
    Empty,
    Obstacle,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_right(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }
}

fn check_if_new_obstacle_creates_loop(
    new_obstacle: &(usize, usize),
    guard_start_location: &(usize, usize),
    guard_start_direction: &Direction,
    grid: &[Vec<Point>],
) -> bool {
    let mut guard_location = *guard_start_location;
    let mut guard_direction = *guard_start_direction;

    // We keep track of states when the guard bumps into an obstacle, and we record the location of the obstacle
    // and the direction the guard was facing when they bumped into it. If we hit the same state again, we know we're in
    // a loop.
    let mut previous_states: HashSet<((usize, usize), Direction)> = HashSet::new();

    loop {
        let next_point: (usize, usize) = match guard_direction {
            Direction::North => (guard_location.0, guard_location.1 - 1),
            Direction::South => (guard_location.0, guard_location.1 + 1),
            Direction::East => (guard_location.0 + 1, guard_location.1),
            Direction::West => (guard_location.0 - 1, guard_location.1),
        };

        if next_point.0 >= grid[0].len() || next_point.1 >= grid.len() {
            // The guard has exited the grid without a loop.
            // Note that if the guard exists west or north, I'm lazily relying on the the relevant coordinate wrapping
            // around to USIZE_MAX - and the grid being smaller than USIZE_MAX.
            return false;
        }

        if next_point == *new_obstacle || grid[next_point.1][next_point.0] == Point::Obstacle {
            // The guard has hit an obstacle.
            //
            // If we've hit this state before, it's a loop! Otherwise, record this state.
            if previous_states.contains(&(next_point, guard_direction)) {
                return true;
            } else {
                previous_states.insert((next_point, guard_direction));
            }

            // We're still going, so the guard turns to the right.
            guard_direction.rotate_right();
        } else {
            guard_location = next_point;
        }
    }
}

pub(crate) fn day06() {
    let f: File = File::open("data/day06.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut grid: Vec<Vec<Point>> = vec![vec![Point::Empty; lines[0].len()]; lines.len()];
    let mut guard_location: (usize, usize) = (0, 0);
    let mut guard_direction: Direction = Direction::North;

    for (row_index, line) in lines.iter().enumerate() {
        for (col_index, c) in line.chars().enumerate() {
            if c == '#' {
                grid[row_index][col_index] = Point::Obstacle;
            } else {
                grid[row_index][col_index] = Point::Empty;
                match c {
                    '^' => {
                        guard_location = (col_index, row_index);
                        guard_direction = Direction::North;
                    }
                    '>' => {
                        guard_location = (col_index, row_index);
                        guard_direction = Direction::East;
                    }
                    '<' => {
                        guard_location = (col_index, row_index);
                        guard_direction = Direction::West;
                    }
                    'v' => {
                        guard_location = (col_index, row_index);
                        guard_direction = Direction::South;
                    }
                    _ => {}
                }
            }
        }
    }

    let guard_start_location = guard_location;
    let guard_start_direction = guard_direction;

    let mut visited_locations: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let next_point: (usize, usize) = match guard_direction {
            Direction::North => (guard_location.0, guard_location.1 - 1),
            Direction::South => (guard_location.0, guard_location.1 + 1),
            Direction::East => (guard_location.0 + 1, guard_location.1),
            Direction::West => (guard_location.0 - 1, guard_location.1),
        };

        if next_point.0 >= grid[0].len() || next_point.1 >= grid.len() {
            // The guard has exited the grid.
            // Note that if the guard exists west or north, I'm lazily relying on the the relevant coordinate wrapping
            // around to USIZE_MAX - and the grid being smaller than USIZE_MAX.
            break;
        }

        if grid[next_point.1][next_point.0] == Point::Empty {
            visited_locations.insert(next_point);
            guard_location = next_point;
        } else if grid[next_point.1][next_point.0] == Point::Obstacle {
            guard_direction.rotate_right();
        } else {
            panic!();
        }
    }
    println!("Day 6 part 1: {}", visited_locations.len());

    // Let's repurpose the visited locations as a set of potential new obstacles. First remove the guard's starting
    // location since we can't just place an obstacle on top of them.
    visited_locations.remove(&guard_start_location);

    let part2_count = visited_locations
        .iter()
        .filter(|&&potential_new_obstacle| {
            check_if_new_obstacle_creates_loop(
                &potential_new_obstacle,
                &guard_start_location,
                &guard_start_direction,
                &grid,
            )
        })
        .count();

    println!("Day 6 part 2: {}", part2_count);
}
