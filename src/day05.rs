use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day05() {
    let f: File = File::open("data/day05.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let final_rule_index = lines.iter().position(|x| x.is_empty()).unwrap() - 1;
    let start_of_updates_index = final_rule_index + 2;

    // Parse the page ordering rules
    //
    // Create a map where the key is the page number and the value is a set of page numbers that must come before it
    let mut page_ordering_rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for i in 0..final_rule_index + 1 {
        let parts: Vec<u32> = lines[i]
            .split("|")
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect();
        page_ordering_rules
            .entry(parts[1])
            .or_default()
            .insert(parts[0]);
    }

    // Process the updates
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    for i in start_of_updates_index..lines.len() {
        // Our strategy is to process the page numbers one at a time
        //  - If the page number has no dependencies that feature later in the list, it's already in the right place
        //  - If the page number does have dependencies that feature later in the list, and therefore is not in the
        //    right place, we will add it to the end the list (specifically to the end of the page_numbers Vec), and
        //    process it again when we get there
        //
        // This may not be the most efficient algorithm, but we will eventually arrive at the correct order
        //
        // Note that we assume that the page numbers are unique and that there are no cycles in the dependencies
        let mut page_numbers: Vec<u32> = lines[i]
            .split(',')
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect();

        // Create a set of the page numbers which we'll use for quickly determining which pages are still to be processed
        let mut future_page_numbers: HashSet<u32> = page_numbers.iter().cloned().collect();

        // We're going to use this Vec to build the correct order of page numbers
        let mut correct_page_numbers: Vec<u32> = Vec::new();

        // Use this index to track our position in the page_numbers Vec
        let mut page_numbers_index: usize = 0;
        loop {
            let page_number = if page_numbers_index >= page_numbers.len() {
                break;
            } else {
                &page_numbers[page_numbers_index]
            };
            if let Some(dependencies) = page_ordering_rules.get(page_number) {
                if future_page_numbers.is_disjoint(dependencies) {
                    // This page number is in the correct place and we're done with it
                    correct_page_numbers.push(*page_number);
                    future_page_numbers.remove(&page_number);
                } else {
                    // This page number is not in the correct place, so push it to the end of the list
                    page_numbers.push(*page_number);
                }
            }
            page_numbers_index += 1;
        }
        // For part 1, we want the sum of the middle page numbers of updates that were already correct
        // For part 2, we want the sum of the middle page numbers of updates that we've had to correct
        //
        // If page_numbers and correct_page_numbers have the same length, it means we didn't have to do any wrangling -
        // hence why we're using that check here
        if page_numbers.len() == correct_page_numbers.len() {
            part1_sum += correct_page_numbers[correct_page_numbers.len() / 2];
        } else {
            part2_sum += correct_page_numbers[correct_page_numbers.len() / 2];
        }
    }

    println!("Day 5 part 1: {}", part1_sum);
    println!("Day 5 part 2: {}", part2_sum);
}
