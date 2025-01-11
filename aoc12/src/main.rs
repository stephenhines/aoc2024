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

type Coord = (usize, usize);

#[derive(Debug)]
struct Garden {
    grid: Grid,
    width: usize,
    height: usize,
}

impl Garden {
    fn new(lines: &[String]) -> Self {
        let mut grid = [[' '; MAX_DIM]; MAX_DIM];
        let height = lines.len();
        let width = lines[0].len();

        for y in 0..height {
            let line = lines[y].chars().collect::<Vec<_>>();
            for x in 0..width {
                grid[y + 1][x + 1] = line[x];
            }
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

    fn fence_price(&self) -> usize {
        let mut price = 0;

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

                //println!("Region: {} {}", self.grid[y][x], region.len());
                price += self.get_area(&region) * self.get_perimeter(&region);
            }
        }

        //self.print_grid();
        println!("Price: {price}");
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

fn main() {
    Garden::new(&get_input("prelim.txt")).fence_price();
    Garden::new(&get_input("input.txt")).fence_price();
}
