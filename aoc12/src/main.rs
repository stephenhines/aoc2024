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

// Our input.txt is 140x140
const MAX_DIM: usize = 150;
type Grid = [[char; MAX_DIM]; MAX_DIM];

// Use an invalid char as a border to simplify bounds checking of indices
const INVALID: char = ' ';

type Coord = (usize, usize);

#[derive(Debug)]
struct Garden {
    grid: Grid,
    width: usize,
    height: usize,
}

impl Garden {
    fn new(lines: &[String]) -> Self {
        let mut grid = [[INVALID; MAX_DIM]; MAX_DIM];
        let height = lines.len();
        let width = lines[0].len();

        for y in 0..height {
            let line = lines[y].chars().collect::<Vec<_>>();
            grid[y + 1][1..width + 1].copy_from_slice(&line);
        }
        Self {
            grid,
            width,
            height,
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

    fn get_region(&self, pos: Coord) -> HashSet<Coord> {
        let mut region = HashSet::new();
        let search = self.grid[pos.1][pos.0];

        // Let's use an iterative method this time, since we almost always use recursion.
        let mut candidates = Vec::new();
        candidates.push(pos);
        while let Some(pos) = candidates.pop() {
            let (x, y) = pos;
            if region.contains(&pos) {
                // We've already seen this plot
                continue;
            }
            if self.grid[y][x] != search {
                // Skip plants that we aren't searching for
                continue;
            }

            region.insert(pos);
            candidates.push((x, y - 1)); // Up
            candidates.push((x, y + 1)); // Down
            candidates.push((x - 1, y)); // Left
            candidates.push((x + 1, y)); // Right
        }

        region
    }

    fn get_area(&self, region: &HashSet<Coord>) -> usize {
        region.len()
    }

    fn get_perimeter(&self, region: &HashSet<Coord>) -> usize {
        let mut perimeter = 0;
        let (x, y) = region.iter().collect::<Vec<_>>().first().unwrap();
        let search = self.grid[*y][*x];

        for &pos in region {
            let (x, y) = pos;
            perimeter += 4;
            if self.grid[y - 1][x] == search {
                perimeter -= 1;
            }
            if self.grid[y + 1][x] == search {
                perimeter -= 1;
            }
            if self.grid[y][x - 1] == search {
                perimeter -= 1;
            }
            if self.grid[y][x + 1] == search {
                perimeter -= 1;
            }
        }

        perimeter
    }

    fn get_sides(&self, region: &HashSet<Coord>) -> usize {
        // This comes down to looking for corners, which are equal to the number of sides
        let mut sides = 0;
        let (x, y) = region.iter().collect::<Vec<_>>().first().unwrap();
        let search = self.grid[*y][*x];

        for &pos in region {
            let (x, y) = pos;

            // Corners can be detected with the following kinds of patterns. For a plot with
            // plant type O, we look for differing X plants in cardinal directions.
            //
            // .X.  .X.  ...  ...
            // XO.  .OX  XO.  .OX
            // ...  ...  .X.  .X.
            let diff_up_left = self.grid[y - 1][x - 1] != search;
            let diff_up = self.grid[y - 1][x] != search;
            let diff_up_right = self.grid[y - 1][x + 1] != search;
            let diff_left = self.grid[y][x - 1] != search;
            let diff_right = self.grid[y][x + 1] != search;
            let diff_down_left = self.grid[y + 1][x - 1] != search;
            let diff_down = self.grid[y + 1][x] != search;
            let diff_down_right = self.grid[y + 1][x + 1] != search;

            if diff_up && diff_left {
                sides += 1; // Upper left corner
            }
            if diff_up && diff_right {
                sides += 1; // Upper right corner
            }
            if diff_down && diff_left {
                sides += 1; // Lower left corner
            }
            if diff_down && diff_right {
                sides += 1; // Lower right corner
            }

            // Check if we have the other kind of inner corner where only one diagonal neighbor is
            // different. Examples below show how this works when starting with the center O.
            //
            // XO.  .OX  ...  ...
            // OO.  .OO  OO.  .OO
            // ...  ...  XO.  .OX
            if !diff_up && !diff_left && diff_up_left {
                sides += 1; // Upper left inner corner
            }
            if !diff_up && !diff_right && diff_up_right {
                sides += 1; // Upper right inner corner
            }
            if !diff_down && !diff_left && diff_down_left {
                sides += 1; // Lower left inner corner
            }
            if !diff_down && !diff_right && diff_down_right {
                sides += 1; // Lower right inner corner
            }
        }

        //println!("{search}: {sides} sides");
        sides
    }

    fn get_regions(&self) -> Vec<HashSet<Coord>> {
        let mut regions = Vec::new();

        // Keep track of the plots we have visited
        let mut counted: HashSet<Coord> = HashSet::new();
        for y in 1..=self.height {
            for x in 1..=self.width {
                let pos = (x, y);
                if counted.contains(&pos) {
                    continue;
                }
                let region = self.get_region(pos);
                counted.extend(&region);
                regions.push(region);
            }
        }
        regions
    }

    fn fence_price(&self) -> usize {
        let mut price = 0;

        let regions = self.get_regions();
        for region in regions {
            price += self.get_area(&region) * self.get_perimeter(&region);
        }

        println!("Price: {price}");
        price
    }

    fn fence_price_sides(&self) -> usize {
        let mut price = 0;

        let regions = self.get_regions();
        for region in regions {
            price += self.get_area(&region) * self.get_sides(&region);
        }
        println!("Price (sides): {price}");
        price
    }
}

#[test]
fn test_prelim() {
    let price = Garden::new(&get_input("prelim.txt")).fence_price();
    assert_eq!(price, 140);

    let price = Garden::new(&get_input("prelim_holes.txt")).fence_price();
    assert_eq!(price, 772);

    let price = Garden::new(&get_input("prelim_large.txt")).fence_price();
    assert_eq!(price, 1930);
}

#[test]
fn test_part1() {
    let price = Garden::new(&get_input("input.txt")).fence_price();
    assert_eq!(price, 1363484);
}

#[test]
fn test_prelim2() {
    let price = Garden::new(&get_input("prelim.txt")).fence_price_sides();
    assert_eq!(price, 80);

    let price = Garden::new(&get_input("prelim_holes.txt")).fence_price_sides();
    assert_eq!(price, 436);

    let price = Garden::new(&get_input("prelim_e.txt")).fence_price_sides();
    assert_eq!(price, 236);
}

#[test]
fn test_part2() {
    let price = Garden::new(&get_input("input.txt")).fence_price_sides();
    assert_eq!(price, 838988);
}

fn main() {
    Garden::new(&get_input("prelim.txt")).fence_price();
    Garden::new(&get_input("input.txt")).fence_price();
    Garden::new(&get_input("prelim.txt")).fence_price_sides();
    Garden::new(&get_input("prelim_holes.txt")).fence_price_sides();
    Garden::new(&get_input("input.txt")).fence_price_sides();
}
