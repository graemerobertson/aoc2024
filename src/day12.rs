use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day12() {
    let f: File = File::open("data/day12.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut arrangement: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }

        arrangement.push(row);
    }

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut processed_plots: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..arrangement[0].len() {
        for j in 0..arrangement.len() {
            let mut region: HashSet<(usize, usize)> = HashSet::new();
            region.insert((i, j));
            let perimeter = process_region((i, j), &mut processed_plots, &arrangement, &mut region);
            part1_sum += region.len() * perimeter;

            // If this is a new region (and not just a plot we'd already processed), it'll have a perimeter.
            if perimeter > 0 {
                // Now calculate the number of faces this region has by calculating the number of corners it has.
                //
                // We do that by considering each vertex, and counting the plots within the region at the vertex.
                let mut max_x = 0;
                let mut max_y = 0;
                let mut min_x = std::usize::MAX;
                let mut min_y = std::usize::MIN;

                for plot in &region {
                    if plot.0 > max_x {
                        max_x = plot.0;
                    }
                    if plot.0 < min_x {
                        min_x = plot.0;
                    }
                    if plot.1 > max_y {
                        max_y = plot.1;
                    }
                    if plot.1 < min_y {
                        min_y = plot.1;
                    }
                }

                let mut number_of_faces = 0;
                for x in min_x..max_x + 2 {
                    for y in min_y..max_y + 2 {
                        let mut count = 0;
                        let mut top_left = false;
                        let mut bottom_left = false;
                        let mut top_right = false;
                        let mut bottom_right = false;
                        if region.contains(&(x - 1, y - 1)) {
                            top_left = true;
                            count += 1;
                        }
                        if region.contains(&(x - 1, y)) {
                            top_right = true;
                            count += 1;
                        }
                        if region.contains(&(x, y - 1)) {
                            bottom_left = true;
                            count += 1;
                        }
                        if region.contains(&(x, y)) {
                            bottom_right = true;
                            count += 1;
                        }

                        if count == 1 {
                            // This vertex has one plot in the region - it's a corner.
                            number_of_faces += 1;
                        } else if count == 2 {
                            // This vertex has two plots in the region - either it's part of a straight line, or
                            // if those plots are opposite each others, it represents two corners.
                            if (bottom_right && top_left) || (bottom_left && top_right) {
                                number_of_faces += 2;
                            }
                        } else if count == 3 {
                            // This vertex has three plots in the region = it's a corner.
                            number_of_faces += 1;
                        }
                    }
                }

                part2_sum += region.len() * number_of_faces;
            }
        }
    }

    println!("Day 1 part 1: {}", part1_sum);
    println!("Day 1 part 2: {}", part2_sum);
}

// Terribly designed function that, given a specific plot:
//  * Builds the region containing that plot in the input variable `region`
//  * Calculates and returns the perimeter of that region
//  * Tracks which plots we've processed on the way in the input variable `processed_plots`
fn process_region(
    location: (usize, usize),
    processed_plots: &mut HashSet<(usize, usize)>,
    arrangement: &Vec<Vec<char>>,
    region: &mut HashSet<(usize, usize)>,
) -> usize {
    if processed_plots.contains(&(location.0, location.1)) {
        return 0;
    } else {
        processed_plots.insert((location.0, location.1));
    }

    let plant_type = arrangement[location.0][location.1];
    let mut neighbouring_plots_in_same_region: HashSet<(usize, usize)> = HashSet::new();
    if location.0 > 0 && arrangement[location.0 - 1][location.1] == plant_type {
        neighbouring_plots_in_same_region.insert((location.0 - 1, location.1));
        region.insert((location.0 - 1, location.1));
    }
    if location.1 > 0 && arrangement[location.0][location.1 - 1] == plant_type {
        neighbouring_plots_in_same_region.insert((location.0, location.1 - 1));
        region.insert((location.0, location.1 - 1));
    }
    if location.0 < arrangement[0].len() - 1
        && arrangement[location.0 + 1][location.1] == plant_type
    {
        neighbouring_plots_in_same_region.insert((location.0 + 1, location.1));
        region.insert((location.0 + 1, location.1));
    }
    if location.1 < arrangement.len() - 1 && arrangement[location.0][location.1 + 1] == plant_type {
        neighbouring_plots_in_same_region.insert((location.0, location.1 + 1));
        region.insert((location.0, location.1 + 1));
    }

    let mut perimeter = 4 - neighbouring_plots_in_same_region.len();
    for neighbour in neighbouring_plots_in_same_region {
        let neighbour_perimeter = process_region(neighbour, processed_plots, arrangement, region);
        perimeter += neighbour_perimeter;
    }

    perimeter
}
