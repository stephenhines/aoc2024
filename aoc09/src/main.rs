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
fn print_filesystem(filesystem: &Vec<isize>) {
    for &id in filesystem {
        if id == -1 {
            print!(".");
        } else {
            print!("{id}");
        }
    }
    println!();
}

struct Disk {
    diskmap: Vec<usize>,
}

impl Disk {
    fn compute_checksum(&mut self) -> usize {
        let mut file_id: isize = 0;
        let mut total_blocks = 0;
        let mut filesystem = Vec::new();
        for (idx, &v) in self.diskmap.iter().enumerate() {
            if idx % 2 == 0 {
                for _ in 0..v {
                    filesystem.push(file_id);
                }
                total_blocks += v;
                file_id += 1;
            } else {
                for _ in 0..v {
                    filesystem.push(-1);
                }
            }
        }

        //print_filesystem(&filesystem);

        let mut checksum = 0;
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

            //print_filesystem(&filesystem);
        }

        //print_filesystem(&filesystem);

        for (idx, &v) in filesystem.iter().take(total_blocks).enumerate() {
            assert_ne!(v, -1);
            checksum += idx * v as usize;
        }

        println!("Checksum: {checksum}");
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

fn main() {
    read_diskmap(&get_input("prelim.txt")).compute_checksum();
    read_diskmap(&get_input("input.txt")).compute_checksum();
}
