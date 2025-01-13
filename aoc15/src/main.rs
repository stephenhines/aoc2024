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

// Our input.txt is 50x50, but it doubles in width in part 2
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

    // This almost seems like it doesn't belong in this struct, due to having
    // to clone it to iterate (while mutating the rest of the struct with
    // other helper functions).
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

        // We only need to look when we didn't expand, since it just copies memory
        if !warehouse.expand {
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

    fn get_next_coord(dir: char, pos: Coord) -> Coord {
        match dir {
            '<' => (pos.0 - 1, pos.1),
            '>' => (pos.0 + 1, pos.1),
            '^' => (pos.0, pos.1 - 1),
            'v' => (pos.0, pos.1 + 1),
            _ => panic!("Unknown move direction {}", dir),
        }
    }

    fn update_grid(&mut self, pos: Coord, item: char) {
        self.grid[pos.1][pos.0] = item;
        if item == '@' {
            self.robot = pos;
        }
    }

    fn move_item(&mut self, dir: char, pos: Coord, just_check: bool) -> bool {
        let item = self.grid[pos.1][pos.0];
        match item {
            '.' => true,
            '#' => false,
            'O' | '@' => {
                let new_pos = Self::get_next_coord(dir, pos);
                if self.move_item(dir, new_pos, just_check) {
                    self.update_grid(pos, '.');
                    self.update_grid(new_pos, item);
                    true
                } else {
                    false
                }
            }
            '[' => {
                let new_l_pos = Self::get_next_coord(dir, pos);
                if dir == '<' {
                    let old_r_pos = (pos.0 + 1, pos.1);
                    let new_r_pos = pos;
                    if self.move_item(dir, new_l_pos, just_check) {
                        self.update_grid(old_r_pos, '.');
                        self.update_grid(new_l_pos, '[');
                        self.update_grid(new_r_pos, ']');
                        return true;
                    } else {
                        return false;
                    }
                } else if dir == '>' {
                    // Skip ahead over the right side of the box in this case
                    let empty_pos = (pos.0 - 1, pos.1);
                    let new_r_pos = Self::get_next_coord(dir, new_l_pos);
                    if self.move_item(dir, new_r_pos, just_check) {
                        self.update_grid(empty_pos, '.');
                        self.update_grid(new_l_pos, '[');
                        self.update_grid(new_r_pos, ']');
                        return true;
                    } else {
                        return false;
                    }
                } else if self.move_item(dir, new_l_pos, true) {
                    // We need to check both halves
                    let old_l_pos = pos;
                    let old_r_pos = (pos.0 + 1, pos.1);
                    let new_r_pos = Self::get_next_coord(dir, old_r_pos);
                    if self.move_item(dir, new_r_pos, true) {
                        if just_check {
                            return true;
                        }
                        self.move_item(dir, new_l_pos, false);
                        self.move_item(dir, new_r_pos, false);
                        self.update_grid(old_l_pos, '.');
                        self.update_grid(old_r_pos, '.');
                        self.update_grid(new_l_pos, '[');
                        self.update_grid(new_r_pos, ']');
                        return true;
                    }
                }
                false
            }
            ']' => {
                // Forward operation to the '[' case instead.
                self.move_item(dir, (pos.0 - 1, pos.1), just_check)
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

        // There's no great way (that I know) to handle iterating over the
        // moves, while mutating the rest of the data structure itself.
        let moves = self.moves.clone();
        for dir in moves {
            //println!("Moving {dir}");
            self.move_item(dir, self.robot, false);
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

        let expand = if self.expand { "(expand)" } else { "" };
        println!("GPS{expand}: {gps}");
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
