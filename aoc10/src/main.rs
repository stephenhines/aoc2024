use std::collections::HashSet;
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

// Just cap this at 50x50
// Use a fancy trick to add a 1 element border on all sides, so this really
// only supports 48x48
const MAX_DIM: usize = 50;
const INVALID: i8 = 99;
type Grid = [[i8; MAX_DIM]; MAX_DIM];

type Coord = (usize, usize);

struct TopoMap {
    grid: Grid,
    width: usize,
    height: usize,
}

impl TopoMap {
    fn new(lines: &[String]) -> Self {
        let mut grid = [[INVALID; 50]; 50];
        let height = lines.len();
        let width = lines[0].len();
        if width > MAX_DIM - 2 || height > MAX_DIM - 2 {
            panic!("Invalid dimensions - height: {height} width: {width}");
        }

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.char_indices() {
                grid[y + 1][x + 1] = match c {
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
                    _ => panic!("Invalid input digit {c}"),
                };
            }
        }

        /*
        println!("Width: {width} Height: {height}");
        for y in 1..height + 1 {
            for x in 1..width + 1 {
                print!("{}", grid[y][x]);
            }
            println!();
        }
        */
        Self {
            grid,
            width,
            height,
        }
    }

    fn get_height(&self, loc: Coord) -> i8 {
        self.grid[loc.1][loc.0]
    }

    fn find_unique_paths(&self, loc: Coord, prev_height: i8) -> usize {
        let (x, y) = loc;

        let height = self.get_height(loc);

        // Check that we're going up incrementally!
        if height != prev_height + 1 {
            return 0;
        }

        if height == 9 {
            // We've reached a plateau
            //println!("Found plateau {:?}", loc);
            return 1;
        }

        let mut score = 0;
        score += self.find_unique_paths((x - 1, y), height); // Left
        score += self.find_unique_paths((x + 1, y), height); // Right
        score += self.find_unique_paths((x, y - 1), height); // Up
        score += self.find_unique_paths((x, y + 1), height); // Down

        //println!("intermediate {:?} - score {score}", loc);
        score
    }

    fn get_trailhead_rating(&self, loc: Coord) -> usize {
        if self.get_height(loc) == 0 {
            self.find_unique_paths(loc, -1)
        } else {
            0
        }
    }

    fn total_rating(&self) -> usize {
        let mut score = 0;
        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                score += self.get_trailhead_rating((x, y));
            }
        }

        println!("Total rating: {score}");
        score
    }

    fn find_unique_plateaus(&self, loc: Coord, prev_height: i8) -> HashSet<Coord> {
        let (x, y) = loc;

        let height = self.get_height(loc);

        // Check that we're going up incrementally!
        if height != prev_height + 1 {
            return HashSet::new();
        }

        if height == 9 {
            // We've reached a plateau
            let mut set = HashSet::new();
            set.insert(loc);
            return set;
        }

        let mut set = self.find_unique_plateaus((x - 1, y), height); // Left
        set.extend(self.find_unique_plateaus((x + 1, y), height)); // Right
        set.extend(self.find_unique_plateaus((x, y - 1), height)); // Up
        set.extend(self.find_unique_plateaus((x, y + 1), height)); // Down

        set
    }

    fn get_trailhead_score(&self, loc: Coord) -> usize {
        if self.get_height(loc) == 0 {
            let set = self.find_unique_plateaus(loc, -1);
            set.len()
        } else {
            0
        }
    }

    fn total_score(&self) -> usize {
        let mut score = 0;
        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                score += self.get_trailhead_score((x, y));
            }
        }

        println!("Total score: {score}");
        score
    }
}

#[test]
fn test_prelim() {
    let total_score = TopoMap::new(&get_input("prelim.txt")).total_score();
    assert_eq!(total_score, 36);
}

#[test]
fn test_part1() {
    let total_score = TopoMap::new(&get_input("input.txt")).total_score();
    assert_eq!(total_score, 501);
}

#[test]
fn test_prelim2() {
    let total_rating = TopoMap::new(&get_input("prelim.txt")).total_rating();
    assert_eq!(total_rating, 81);
}

#[test]
fn test_part2() {
    let total_rating = TopoMap::new(&get_input("input.txt")).total_rating();
    assert_eq!(total_rating, 1017);
}

fn main() {
    let topomap = TopoMap::new(&get_input("prelim.txt"));
    topomap.total_score();
    topomap.total_rating();
    let topomap = TopoMap::new(&get_input("input.txt"));
    topomap.total_score();
    topomap.total_rating();
}
