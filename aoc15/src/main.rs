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

// Our input.txt is 50x50
const MAX_DIM: usize = 60;
type Grid = [[char; MAX_DIM]; MAX_DIM];

// Use an invalid char as a border to simplify bounds checking of indices
const INVALID: char = ' ';

type Coord = (usize, usize);

#[derive(Debug)]
struct Warehouse {
    grid: Grid,
    width: usize,
    height: usize,

    moves: Vec<char>,
}

impl Warehouse {
    fn new(lines: &[String]) -> Self {
        let width = lines[0].len();
        let mut height = 0;

        // Get the grid first
        let mut grid = [[INVALID; MAX_DIM]; MAX_DIM];
        let mut line_iter = lines.iter();
        for (y, line) in line_iter.by_ref().enumerate() {
            if line.is_empty() {
                height = y;
                break;
            }

            let line = line.chars().collect::<Vec<_>>();
            grid[y + 1][1..width + 1].copy_from_slice(&line);
        }

        // Get the moves next
        let mut moves = Vec::new();
        for line in line_iter {
            let line = line.chars().collect::<Vec<_>>();
            moves.extend_from_slice(&line);
        }

        Self {
            grid,
            width,
            height,
            moves,
        }
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        println!("width: {} height: {}", self.width, self.height);
        for y in 1..=self.height {
            for x in 1..=self.width {
                print!("{}", self.grid[y][x]);
            }
            println!();
        }
    }

    // Returns true if pos is empty, or if we can free up space to make it empty
    fn move_item(&mut self, dir: char, pos: Coord) -> bool {
        let item = self.grid[pos.1][pos.0];
        match item {
            '.' => true,
            '#' => false,
            'O' | '@' => {
                let new_pos = match dir {
                    '<' => (pos.0 - 1, pos.1),
                    '>' => (pos.0 + 1, pos.1),
                    '^' => (pos.0, pos.1 - 1),
                    'v' => (pos.0, pos.1 + 1),
                    _ => {
                        panic!("Unknown move direction {}", dir);
                    }
                };
                if self.move_item(dir, new_pos) {
                    self.grid[new_pos.1][new_pos.0] = item;
                    self.grid[pos.1][pos.0] = '.';
                    true
                } else {
                    false
                }
            }
            _ => {
                panic!("Unknown contents at {:?}: {}", pos, self.grid[pos.1][pos.0]);
            }
        }
    }

    fn find_robot(&self) -> Coord {
        for y in 1..=self.height {
            for x in 1..=self.width {
                if self.grid[y][x] == '@' {
                    return (x, y);
                }
            }
        }
        unreachable!("No robot found");
    }

    fn move_robot(&mut self) -> &Self {
        let mut robot = self.find_robot();
        //self.print_grid();
        //println!("Robot: {:?}\n", robot);
        let moves = self.moves.clone();
        for dir in moves {
            //println!("Moving {dir}");
            if self.move_item(dir, robot) {
                robot = self.find_robot();
            }
            //self.print_grid();
            //println!();
        }
        self
    }

    fn compute_gps(&self) -> usize {
        let mut gps = 0;
        for y in 1..=self.height {
            for x in 1..=self.width {
                if self.grid[y][x] == 'O' {
                    // We're offset because of the border
                    gps += (y - 1) * 100 + (x - 1);
                }
            }
        }

        println!("GPS: {gps}");
        gps
    }
}

#[test]
fn test_prelim() {
    let gps = Warehouse::new(&get_input("prelim.txt"))
        .move_robot()
        .compute_gps();
    assert_eq!(gps, 2028);

    let gps = Warehouse::new(&get_input("prelim2.txt"))
        .move_robot()
        .compute_gps();
    assert_eq!(gps, 10092);
}

#[test]
fn test_part1() {
    let gps = Warehouse::new(&get_input("input.txt"))
        .move_robot()
        .compute_gps();
    assert_eq!(gps, 1490942);
}

fn main() {
    Warehouse::new(&get_input("prelim.txt"))
        .move_robot()
        .compute_gps();
    Warehouse::new(&get_input("prelim2.txt"))
        .move_robot()
        .compute_gps();
    Warehouse::new(&get_input("input.txt"))
        .move_robot()
        .compute_gps();
}
