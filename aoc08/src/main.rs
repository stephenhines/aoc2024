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
    fn find_antinodes(&mut self) -> usize {
        let mut antinodes = HashSet::new();
        for locs in self.antennas.values() {
            for (idx, &left) in locs.iter().enumerate() {
                for &right in locs.iter().skip(idx + 1) {
                    let nodes = compute_antinodes(left, right);
                    for node in nodes {
                        if node.0 >= 0
                            && node.0 < self.width as isize
                            && node.1 >= 0
                            && node.1 < self.height as isize
                        {
                            antinodes.insert(node);
                        }
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

// None of these are bounds-checked. We do that in the caller.
fn compute_antinodes(left: Coord, right: Coord) -> Vec<Coord> {
    let mut nodes = Vec::new();
    let x_diff = (left.0 - right.0).abs();
    let y_diff = (left.1 - right.1).abs();
    if left.0 < right.0 {
        if left.1 < right.1 {
            // L.
            // .R
            let x = left.0 - x_diff;
            let y = left.1 - y_diff;
            nodes.push((x, y));

            let x = right.0 + x_diff;
            let y = right.1 + y_diff;
            nodes.push((x, y));
        } else {
            // left.1 >= right.1
            // .R
            // L.
            let x = left.0 - x_diff;
            let y = left.1 + y_diff;
            nodes.push((x, y));

            let x = right.0 + x_diff;
            let y = right.1 - y_diff;
            nodes.push((x, y));
        }
    } else {
        // left.0 >= right.0
        if left.1 < right.1 {
            // .L
            // R.
            let x = right.0 - x_diff;
            let y = right.1 + y_diff;
            nodes.push((x, y));

            let x = left.0 + x_diff;
            let y = left.1 - y_diff;
            nodes.push((x, y));
        } else {
            // left.1 >= right.1
            // R.
            // .L
            let x = right.0 - x_diff;
            let y = right.1 - y_diff;
            nodes.push((x, y));

            let x = left.0 + x_diff;
            let y = left.1 + y_diff;
            nodes.push((x, y));
        }
    }

    nodes
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
    let antinodes = read_graph(&get_input("prelim.txt")).find_antinodes();
    assert_eq!(antinodes, 14);
}

#[test]
fn test_part1() {
    let antinodes = read_graph(&get_input("input.txt")).find_antinodes();
    assert_eq!(antinodes, 344);
}

fn main() {
    read_graph(&get_input("prelim.txt")).find_antinodes();
    read_graph(&get_input("input.txt")).find_antinodes();
}
