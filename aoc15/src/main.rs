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
const MAX_DIM: usize = 120;
type Grid = [[char; MAX_DIM]; MAX_DIM];

// Use an invalid char as a border to simplify bounds checking of indices
const INVALID: char = ' ';

type Coord = (usize, usize);

#[derive(Debug)]
struct Warehouse {
    grid: Grid,
    width: usize,
    height: usize,
    expand: bool,
    robot: Coord,

    moves: Vec<char>,
}

impl Warehouse {
    fn new(lines: &[String], expand: bool) -> Self {
        let width = if expand {
            lines[0].len() * 2
        } else {
            lines[0].len()
        };
        let mut height = 0;
        let mut robot = (0, 0);

        // Get the grid first
        let mut grid = [[INVALID; MAX_DIM]; MAX_DIM];
        let mut line_iter = lines.iter();
        for (y, line) in line_iter.by_ref().enumerate() {
            if line.is_empty() {
                height = y;
                break;
            }

            let line = line.chars().collect::<Vec<_>>();
            if !expand {
                grid[y + 1][1..width + 1].copy_from_slice(&line);
            } else {
                let mut x = 1;
                for c in line {
                    match c {
                        '#' | '.' => {
                            grid[y + 1][x] = c;
                            grid[y + 1][x + 1] = c;
                        }
                        'O' => {
                            grid[y + 1][x] = '[';
                            grid[y + 1][x + 1] = ']';
                        }
                        '@' => {
                            grid[y + 1][x] = c;
                            grid[y + 1][x + 1] = '.';
                            robot = (x, y + 1);
                        }
                        _ => {
                            panic!("Invalid grid node: {c}");
                        }
                    }
                    x += 2;
                }
            }
        }

        // Get the moves next
        let mut moves = Vec::new();
        for line in line_iter {
            let line = line.chars().collect::<Vec<_>>();
            moves.extend_from_slice(&line);
        }

        let mut warehouse = Self {
            grid,
            width,
            height,
            expand,
            robot,
            moves,
        };

        if !expand {
            warehouse.robot = warehouse.find_robot();
        }

        warehouse
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
            'O' | '@' | '[' | ']' => {
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
                    if item == '@' {
                        self.robot = new_pos;
                    }
                    true
                } else {
                    false
                }
            }
            _ => panic!("Unknown contents at {:?}: {}", pos, self.grid[pos.1][pos.0]),
        }
    }

    fn get_next_coord(dir: char, pos: Coord) -> Coord {
        match dir {
            '<' => (pos.0 - 1, pos.1),
            '>' => (pos.0 + 1, pos.1),
            '^' => (pos.0, pos.1 - 1),
            'v' => (pos.0, pos.1 + 1),
            _ => panic!("Unknown move direction {}", dir),
        }
    }

    fn move_item2(&mut self, dir: char, pos: Coord, just_check: bool) -> bool {
        if dir == '<' || dir == '>' {
            return self.move_item(dir, pos);
        }
        let item = self.grid[pos.1][pos.0];
        let new_pos = Self::get_next_coord(dir, pos);
        match item {
            '.' => true,
            '#' => false,
            'O' | '@' => {
                if self.move_item2(dir, new_pos, just_check) {
                    self.grid[new_pos.1][new_pos.0] = item;
                    self.grid[pos.1][pos.0] = '.';
                    if item == '@' {
                        self.robot = new_pos;
                    }
                    true
                } else {
                    false
                }
            }
            '[' => {
                // We need to check both halves
                if self.move_item2(dir, new_pos, true) {
                    let new_neighbor = Self::get_next_coord(dir, (pos.0 + 1, pos.1));
                    if self.move_item2(dir, new_neighbor, true) {
                        if just_check {
                            return true;
                        }
                        self.move_item2(dir, new_pos, false);
                        self.move_item2(dir, new_neighbor, false);
                        self.grid[pos.1][pos.0] = '.';
                        self.grid[pos.1][pos.0 + 1] = '.';
                        self.grid[new_pos.1][new_pos.0] = '[';
                        self.grid[new_pos.1][new_pos.0 + 1] = ']';
                        return true;
                    }
                }
                false
            }
            ']' => {
                // Forward operation back to the '[' case instead
                self.move_item2(dir, (pos.0 - 1, pos.1), just_check)
            }
            _ => panic!("Unknown contents at {:?}: {}", pos, self.grid[pos.1][pos.0]),
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
        //self.print_grid();
        //println!("Robot: {:?}\n", self.robot);
        let moves = self.moves.clone();
        for dir in moves {
            //println!("Moving {dir}");
            if self.expand {
                self.move_item2(dir, self.robot, false);
            } else {
                self.move_item(dir, self.robot);
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
                match self.grid[y][x] {
                    'O' | '[' => {
                        // We're offset because of the border
                        gps += (y - 1) * 100 + (x - 1);
                    }
                    _ => {}
                }
            }
        }

        println!("GPS: {gps}");
        gps
    }
}

#[test]
fn test_prelim() {
    let gps = Warehouse::new(&get_input("prelim.txt"), false)
        .move_robot()
        .compute_gps();
    assert_eq!(gps, 2028);

    let gps = Warehouse::new(&get_input("prelim2.txt"), false)
        .move_robot()
        .compute_gps();
    assert_eq!(gps, 10092);
}

#[test]
fn test_part1() {
    let gps = Warehouse::new(&get_input("input.txt"), false)
        .move_robot()
        .compute_gps();
    assert_eq!(gps, 1490942);
}

#[test]
fn test_prelim2() {
    let gps = Warehouse::new(&get_input("prelim3.txt"), true)
        .move_robot()
        .compute_gps();
    assert_eq!(gps, 618);

    let gps = Warehouse::new(&get_input("prelim2.txt"), true)
        .move_robot()
        .compute_gps();
    assert_eq!(gps, 9021);
}

#[test]
fn test_part2() {
    let gps = Warehouse::new(&get_input("input.txt"), true)
        .move_robot()
        .compute_gps();
    assert_eq!(gps, 1519202);
}

fn main() {
    Warehouse::new(&get_input("prelim.txt"), false)
        .move_robot()
        .compute_gps();
    Warehouse::new(&get_input("prelim2.txt"), false)
        .move_robot()
        .compute_gps();
    Warehouse::new(&get_input("input.txt"), false)
        .move_robot()
        .compute_gps();
    Warehouse::new(&get_input("prelim3.txt"), true)
        .move_robot()
        .compute_gps();
    Warehouse::new(&get_input("prelim2.txt"), true)
        .move_robot()
        .compute_gps();
    Warehouse::new(&get_input("input.txt"), true)
        .move_robot()
        .compute_gps();
}
