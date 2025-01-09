use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

#[allow(dead_code)]
fn print_filesystem(filesystem: &[isize]) {
    for &id in filesystem {
        if id == -1 {
            print!(".");
        } else {
            print!("{id}");
        }
    }
    println!();
}

#[allow(dead_code)]
fn print_blocks(blocks: &[Block]) {
    for &block in blocks {
        match block.block_type {
            BlockType::Space => {
                (0..block.size).for_each(|_| print!("."));
            }
            BlockType::File => {
                (0..block.size).for_each(|_| print!("{}", block.file_id));
            }
        }
    }
    println!();
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BlockType {
    File,
    Space,
}

#[derive(Clone, Copy, Debug)]
struct Block {
    block_type: BlockType,
    file_id: isize,
    size: usize,
}

struct Disk {
    diskmap: Vec<usize>,
}

impl Disk {
    fn create_filesystem(&self) -> Vec<isize> {
        let mut file_id: isize = 0;
        let mut filesystem = Vec::new();
        for (idx, &v) in self.diskmap.iter().enumerate() {
            if idx % 2 == 0 {
                for _ in 0..v {
                    filesystem.push(file_id);
                }
                file_id += 1;
            } else {
                for _ in 0..v {
                    filesystem.push(-1);
                }
            }
        }

        filesystem
    }

    fn create_blocks(&self) -> Vec<Block> {
        let mut file_id: isize = 0;
        let mut blocks = Vec::new();
        for (idx, &size) in self.diskmap.iter().enumerate() {
            if idx % 2 == 0 {
                let block_type = BlockType::File;
                let block = Block {
                    block_type,
                    file_id,
                    size,
                };
                blocks.push(block);
                file_id += 1;
            } else if size > 0 {
                let block_type = BlockType::Space;
                let block = Block {
                    block_type,
                    file_id: -1,
                    size,
                };
                blocks.push(block);
            }
        }

        blocks
    }

    fn compact_filesystem(&self, filesystem: &mut [isize]) {
        let total_blocks = filesystem.iter().filter(|x| !x.is_negative()).count();

        let mut i = 0;
        let mut last = filesystem.len() - 1;
        while i < total_blocks {
            // Find the first empty block
            while filesystem[i] != -1 {
                i += 1;
            }

            // Find the previous file block
            while filesystem[last] == -1 {
                last -= 1;
            }

            if i >= total_blocks {
                break;
            }

            filesystem[i] = filesystem[last];
            filesystem[last] = -1;
            i += 1;
            last -= 1;
        }
    }

    // These always move blocks from back to front, so we don't need to worry about coalescing free space at the back.
    fn move_block(blocks: &mut Vec<Block>, file_id: isize) {
        let file_block_id = blocks
            .iter()
            .enumerate()
            .find(|(_, x)| x.file_id == file_id)
            .unwrap()
            .0;
        let size = blocks[file_block_id].size;
        if let Some((space_block_id, space_block)) = blocks.iter().enumerate().find(|(bidx, x)| {
            bidx < &file_block_id && x.block_type == BlockType::Space && x.size >= size
        }) {
            let leftover_space = space_block.size - size;
            if leftover_space == 0 {
                // We can just swap in the data here in this case! :)
                blocks[space_block_id].block_type = BlockType::File;
                blocks[space_block_id].file_id = file_id;
                blocks[file_block_id].block_type = BlockType::Space;
                blocks[file_block_id].file_id = -1;
            } else {
                let new_block = blocks[file_block_id];
                // Subtract off the space for the newly moved file
                blocks[space_block_id].size -= size;
                blocks[file_block_id].block_type = BlockType::Space;
                blocks[file_block_id].file_id = -1;
                // Insert the block last, since we otherwise run into trouble with indices
                blocks.insert(space_block_id, new_block);
            }
        }
    }

    fn compute_checksum_whole(&self) -> usize {
        let mut blocks = self.create_blocks();

        // Find the highest numbered File block
        let last_block_id = blocks
            .iter()
            .rev()
            .find(|x| x.block_type == BlockType::File)
            .unwrap()
            .file_id;

        // Work backwards through the file_ids
        for file_id in (1..=last_block_id).rev() {
            Self::move_block(&mut blocks, file_id);
        }

        // Compute the actual checksum
        let mut checksum = 0;
        let mut idx = 0;
        for block in blocks {
            match block.block_type {
                BlockType::Space => {
                    idx += block.size;
                }
                BlockType::File => {
                    for _ in 0..block.size {
                        checksum += idx * block.file_id as usize;
                        idx += 1;
                    }
                }
            }
        }

        //print_blocks(&blocks);
        println!("Checksum (whole): {checksum}");
        checksum
    }

    fn compute_checksum(&self) -> usize {
        let mut filesystem = self.create_filesystem();
        self.compact_filesystem(&mut filesystem);

        let checksum = filesystem
            .iter()
            .enumerate()
            .filter(|(_, x)| !x.is_negative())
            .fold(0, |sum, (idx, &v)| sum + idx * v as usize);

        println!("Checksum (fragmented): {checksum}");
        checksum
    }
}

fn read_diskmap(lines: &[String]) -> Disk {
    assert_eq!(lines.len(), 1);
    let mut diskmap: Vec<usize> = Vec::new();
    for c in lines[0].chars() {
        let val = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => {
                panic!("Invalid char input {c}");
            }
        };
        diskmap.push(val);
    }

    Disk { diskmap }
}

#[test]
fn test_prelim() {
    let checksum = read_diskmap(&get_input("prelim.txt")).compute_checksum();
    assert_eq!(checksum, 1928);
}

#[test]
fn test_part1() {
    let checksum = read_diskmap(&get_input("input.txt")).compute_checksum();
    assert_eq!(checksum, 6154342787400);
}

#[test]
fn test_prelim2() {
    let checksum = read_diskmap(&get_input("prelim.txt")).compute_checksum_whole();
    assert_eq!(checksum, 2858);
}

#[test]
fn test_part2() {
    let checksum = read_diskmap(&get_input("input.txt")).compute_checksum_whole();
    assert_eq!(checksum, 6183632723350);
}

fn main() {
    read_diskmap(&get_input("prelim.txt")).compute_checksum();
    read_diskmap(&get_input("input.txt")).compute_checksum();
    read_diskmap(&get_input("prelim.txt")).compute_checksum_whole();
    read_diskmap(&get_input("input.txt")).compute_checksum_whole();
}
