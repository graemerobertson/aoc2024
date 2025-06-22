use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, PartialEq)]
enum Block {
    File(u64),
    Space,
}

#[derive(Debug, Clone, PartialEq)]
enum SetOfBlocks {
    File(u64, u64),
    Space(u64),
}

pub(crate) fn day09() {
    let f: File = File::open("data/day09.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut disk: Vec<Block> = Vec::new();
    let mut id: u64 = 0;
    let mut next_char_represents_file = true;
    for c in lines[0].chars() {
        let value = c.to_digit(10).unwrap();
        match next_char_represents_file {
            true => {
                for _ in 0..value {
                    disk.push(Block::File(id));
                }
                id += 1;
            }
            false => {
                for _ in 0..value {
                    disk.push(Block::Space);
                }
            }
        }
        next_char_represents_file = !next_char_represents_file;
    }

    let mut disk_clone: Vec<Block> = disk.clone();
    for i in 0..disk.len() {
        if disk[i] == Block::Space {
            let original_disk_clone_len = disk_clone.len();
            // Pop blocks off disk_clone until we find a file block
            let mut new_value = Block::Space;
            while new_value == Block::Space {
                new_value = disk_clone.pop().unwrap();
            }
            if disk_clone.len() > i {
                disk[i] = new_value;
            } else {
                for j in original_disk_clone_len..disk.len() {
                    disk[j] = Block::Space;
                }
                break;
            }
        }
    }

    let mut checksum: u64 = 0;
    for (i, block) in disk.iter().enumerate() {
        if let Block::File(id) = block {
            checksum += i as u64 * id;
        }
    }
    println!("Day 9 part 1: {}", checksum);

    let mut part2_disk: Vec<SetOfBlocks> = Vec::new();
    let mut next_char_represents_file = true;
    let mut file_id = 0;
    for c in lines[0].chars() {
        let size = c.to_digit(10).unwrap() as u64;
        match next_char_represents_file {
            true => {
                part2_disk.push(SetOfBlocks::File(size, file_id));
                file_id += 1;
            }
            false => part2_disk.push(SetOfBlocks::Space(size)),
        }
        next_char_represents_file = !next_char_represents_file;
    }

    let mut part2_disk_clone: Vec<SetOfBlocks> = part2_disk.clone();
    while part2_disk_clone.len() > 2 {
        let file = part2_disk_clone.pop().unwrap();
        if let SetOfBlocks::File(size, file_to_move_id) = file {
            let index = part2_disk.iter().position(|x| {
                if let SetOfBlocks::Space(s) = x {
                    *s >= size
                } else if let SetOfBlocks::File(_, compare_id) = x {
                    // If we've reached the file we're trying to move, end the search - we don't want to move the file
                    // further right than here.
                    file_to_move_id == *compare_id
                } else {
                    false
                }
            });

            // Due to the gross code above, this index variable may contain one of two things:
            // 1. The index of a space block that is large enough to hold the file
            // 2. The index of the file block itself, which indicates that it's already in the correct position.
            // Unwrap and filter out this second option now.
            let index = index.unwrap();
            if part2_disk[index] == SetOfBlocks::File(size, file_to_move_id) {
                continue;
            }

            // Now we've determined we're going to move the file, let's first overwrite the file with a space block.
            // First, find it.
            let index_of_file_in_part2_disk = part2_disk
                .iter()
                .position(|y| {
                    if let SetOfBlocks::File(_, id) = y {
                        file_to_move_id == *id
                    } else {
                        false
                    }
                })
                .unwrap();
            part2_disk[index_of_file_in_part2_disk] = SetOfBlocks::Space(size);

            // Now insert the file at the correct position.
            part2_disk.insert(index, SetOfBlocks::File(size, file_to_move_id));

            // Finally, update or remove the space block that was previously at the index where we inserted the file.
            let space_block = &part2_disk[index + 1];
            if let SetOfBlocks::Space(space_block_size) = space_block {
                if *space_block_size > size {
                    part2_disk[index + 1] = SetOfBlocks::Space(*space_block_size - size);
                } else {
                    part2_disk.remove(index + 1);
                }
            } else {
                panic!("Expected a space block here, but found: {:?}", space_block);
            }
        }
    }

    let mut part2_checksum: u64 = 0;
    let mut i = 0;
    for block in part2_disk {
        match block {
            SetOfBlocks::File(size, id) => {
                for _ in 0..size {
                    part2_checksum += i * id;
                    i += 1;
                }
            }
            SetOfBlocks::Space(size) => {
                i += size;
            }
        }
    }
    println!("Day 9 part 2: {}", part2_checksum);
}
