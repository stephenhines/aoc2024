use std::collections::HashMap;
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

type Coord = (isize, isize);

struct Graph {
    antennas: HashMap<char, Vec<Coord>>,
    width: usize,
    height: usize,
}

impl Graph {
    fn inbounds(&self, coord: Coord) -> bool {
        coord.0 >= 0
            && coord.0 < self.width as isize
            && coord.1 >= 0
            && coord.1 < self.height as isize
    }

    fn compute_antinodes(&self, left: Coord, right: Coord, harmonics: bool) -> Vec<Coord> {
        let mut nodes = Vec::new();
        let mut coord = left;
        let diff = (left.0 - right.0, left.1 - right.1);
        if harmonics {
            while self.inbounds(coord) {
                nodes.push(coord);
                coord = (coord.0 + diff.0, coord.1 + diff.1);
            }
        } else {
            coord = (coord.0 + diff.0, coord.1 + diff.1);
            if self.inbounds(coord) {
                nodes.push(coord);
            }
        }

        nodes
    }

    fn find_antinodes(&mut self, harmonics: bool) -> usize {
        let mut antinodes = HashSet::new();
        for locs in self.antennas.values() {
            for (idx, &left) in locs.iter().enumerate() {
                for &right in locs.iter().skip(idx + 1) {
                    let nodes = self.compute_antinodes(left, right, harmonics);
                    for node in nodes {
                        antinodes.insert(node);
                    }
                    let nodes = self.compute_antinodes(right, left, harmonics);
                    for node in nodes {
                        antinodes.insert(node);
                    }
                }
            }
        }

        println!("Antinodes: {}", antinodes.len());
        //self.display_graph(&antinodes);
        antinodes.len()
    }

    #[allow(dead_code)]
    fn display_graph(&self, antinodes: &HashSet<Coord>) {
        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                let coord = (x, y);
                if antinodes.contains(&coord) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn read_graph(lines: &[String]) -> Graph {
    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();
    let height = lines.len();
    let width = lines[0].len();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.char_indices() {
            if c != '.' {
                let coord = (x as isize, y as isize);
                if let std::collections::hash_map::Entry::Vacant(e) = antennas.entry(c) {
                    e.insert(vec![coord]);
                } else {
                    antennas.get_mut(&c).unwrap().push(coord);
                }
            }
        }
    }

    //dbg!(&antennas);
    Graph {
        antennas,
        width,
        height,
    }
}

#[test]
fn test_prelim() {
    let antinodes = read_graph(&get_input("prelim.txt")).find_antinodes(false);
    assert_eq!(antinodes, 14);
}

#[test]
fn test_part1() {
    let antinodes = read_graph(&get_input("input.txt")).find_antinodes(false);
    assert_eq!(antinodes, 344);
}

#[test]
fn test_prelim2() {
    let antinodes = read_graph(&get_input("prelim.txt")).find_antinodes(true);
    assert_eq!(antinodes, 34);
}

#[test]
fn test_part2() {
    let antinodes = read_graph(&get_input("input.txt")).find_antinodes(true);
    assert_eq!(antinodes, 1182);
}

fn main() {
    read_graph(&get_input("prelim.txt")).find_antinodes(false);
    read_graph(&get_input("input.txt")).find_antinodes(false);
    read_graph(&get_input("prelim.txt")).find_antinodes(true);
    read_graph(&get_input("input.txt")).find_antinodes(true);
}
