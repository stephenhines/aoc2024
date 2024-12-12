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

type Coord = (usize, usize);

#[derive(Debug)]
enum Direction {
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Up,
    UpRight,
}

const DIRECTIONS: [Direction; 8] = [
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
];

struct Puzzle {
    chars: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Puzzle {
    fn new(lines: &Vec<String>) -> Self {
        let mut chars = Vec::new();
        for line in lines {
            let mut col = Vec::new();
            for c in line.chars() {
                col.push(c);
            }
            chars.push(col);
        }
        let height = chars.len();
        let width = chars[0].len();
        Self {
            chars,
            height,
            width,
        }
    }

    fn get_char(&self, pos: Coord) -> char {
        self.chars[pos.1][pos.0]
    }

    fn check_char(&self, c: char, pos: Coord) -> bool {
        self.get_char(pos) == c
    }

    fn search_start(&self, needle: &str, pos: Coord) -> u32 {
        let (x, y) = pos;
        let mut total = 0;

        let height = self.height;
        let width = self.width;
        let needle_len = needle.len();

        for dir in DIRECTIONS {
            let mut valid = true;
            // Check horizontal spread
            match dir {
                Direction::UpRight | Direction::Right | Direction::DownRight => {
                    if x + needle_len > width {
                        valid = false;
                    }
                }
                Direction::UpLeft | Direction::Left | Direction::DownLeft => {
                    if x + 1 < needle_len {
                        valid = false;
                    }
                }
                _ => {}
            }
            // Check vertical spread
            match dir {
                Direction::UpLeft | Direction::Up | Direction::UpRight => {
                    if y + 1 < needle_len {
                        valid = false;
                    }
                }
                Direction::DownLeft | Direction::Down | Direction::DownRight => {
                    if y + needle_len > height {
                        valid = false;
                    }
                }
                _ => {}
            }
            if valid {
                let mut new_pos = pos;
                for c in needle.chars() {
                    valid &= self.check_char(c, new_pos);
                    new_pos = next_pos(new_pos, &dir);
                }
            }
            if valid {
                total += 1;
            }
        }

        total
    }

    fn search_mas(&self, pos: Coord) -> bool {
        let mut valid = true;
        let cur = self.get_char(pos);
        if cur != 'A' {
            return false;
        }

        let up_left = self.get_char(next_pos(pos, &Direction::UpLeft));
        let up_right = self.get_char(next_pos(pos, &Direction::UpRight));
        let down_left = self.get_char(next_pos(pos, &Direction::DownLeft));
        let down_right = self.get_char(next_pos(pos, &Direction::DownRight));

        valid &= (up_left == 'M' && down_right == 'S') || (up_left == 'S' && down_right == 'M');
        valid &= (up_right == 'M' && down_left == 'S') || (up_right == 'S' && down_left == 'M');

        valid
    }

    fn search_xmas(&self) -> u32 {
        let mut result = 0;
        let needle = "XMAS";

        for y in 0..self.height {
            for x in 0..self.width {
                if self.check_char(needle.chars().next().unwrap(), (x, y)) {
                    result += self.search_start(needle, (x, y));
                }
            }
        }

        println!("Found {}: {}", needle, result);
        result
    }

    fn search_x_mas(&self) -> u32 {
        let mut result = 0;

        // Only have to walk the inner perimeter, since we are just trying to find the 'A' values,
        // and then check for the X pattern.
        for y in 1..self.height-1 {
            for x in 1..self.width-1 {
                if self.check_char('A', (x, y)) {
                    if self.search_mas((x, y)) {
                        result += 1;
                    }
                }
            }
        }

        println!("Found X-MAS: {}", result);
        result
    }
}

fn next_pos(pos: Coord, dir: &Direction) -> Coord {
    let mut new_x = pos.0;
    let mut new_y = pos.1;
    match dir {
        Direction::UpRight | Direction::Right | Direction::DownRight => {
            new_x = new_x.saturating_add(1);
        }
        Direction::UpLeft | Direction::Left | Direction::DownLeft => {
            new_x = new_x.saturating_sub(1);
        }
        _ => {}
    }
    match dir {
        Direction::UpLeft | Direction::Up | Direction::UpRight => {
            new_y = new_y.saturating_sub(1);
        }
        Direction::DownLeft | Direction::Down | Direction::DownRight => {
            new_y = new_y.saturating_add(1);
        }
        _ => {}
    }
    (new_x, new_y)
}


#[test]
fn test_prelim() {
    let result = Puzzle::new(&get_input("prelim.txt")).search_xmas();
    assert_eq!(result, 18);
}

#[test]
fn test_prelim2() {
    let result = Puzzle::new(&get_input("prelim.txt")).search_x_mas();
    assert_eq!(result, 9);
}

#[test]
fn test_part1() {
    let result = Puzzle::new(&get_input("input.txt")).search_xmas();
    assert_eq!(result, 2644);
}

#[test]
fn test_part2() {
    let result = Puzzle::new(&get_input("input.txt")).search_x_mas();
    assert_eq!(result, 1952);
}

fn main() {
    let prelim = Puzzle::new(&get_input("prelim.txt"));
    let puzzle = Puzzle::new(&get_input("input.txt"));
    prelim.search_xmas();
    puzzle.search_xmas();
    prelim.search_x_mas();
    puzzle.search_x_mas();
}
