use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
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

// Our input.txt is 71x71
const MAX_DIM: usize = 80;
type Grid = [[char; MAX_DIM]; MAX_DIM];

// Use an invalid char as a border to simplify bounds checking of indices
const INVALID: char = '#';

type Coord = (usize, usize);

fn in_bounds(grid: &Grid, pos: Coord) -> bool {
    match grid[pos.1][pos.0] {
        '#' => false,
        '.' => true,
        'S' | 'E' => true,
        c => panic!("Invalid char: {c}"),
    }
}

fn possible_moves(grid: &Grid, pos: Coord) -> Vec<Coord> {
    let mut possible_moves = Vec::new();

    let up = (pos.0, pos.1 - 1);
    if in_bounds(grid, up) {
        possible_moves.push(up);
    }
    let down = (pos.0, pos.1 + 1);
    if in_bounds(grid, down) {
        possible_moves.push(down);
    }
    let left = (pos.0 - 1, pos.1);
    if in_bounds(grid, left) {
        possible_moves.push(left);
    }
    let right = (pos.0 + 1, pos.1);
    if in_bounds(grid, right) {
        possible_moves.push(right);
    }
    possible_moves
}

#[derive(Eq, PartialEq)]
struct State {
    pos: Coord,
    cost: usize,
    path: Vec<Coord>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type DistType = BTreeMap<Coord, usize>;
type PrevType = BTreeMap<Coord, HashSet<Coord>>;

type HeapType = BinaryHeap<State>;
type PathsType = Vec<Vec<Coord>>;

// Returns the dist, prev entries
fn dijkstra(grid: &Grid, start: Coord, end: Coord) -> (usize, DistType, PrevType, PathsType) {
    /*
        Based on https://doc.rust-lang.org/nightly/std/collections/binary_heap/index.html
        and https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm

        1   function Dijkstra(Graph, source):
        2       create vertex priority queue Q
        3
        4       dist[source] ← 0                          // Initialization
        5       Q.add_with_priority(source, 0)            // associated priority equals dist[·]
        6
        7       for each vertex v in Graph.Vertices:
        8           if v ≠ source
        9               prev[v] ← UNDEFINED               // Predecessor of v
        10              dist[v] ← INFINITY                // Unknown distance from source to v
        11              Q.add_with_priority(v, INFINITY)
        12
        13
        14      while Q is not empty:                     // The main loop
        15          u ← Q.extract_min()                   // Remove and return best vertex
        16          for each neighbor v of u:             // Go through all v neighbors of u
        17              alt ← dist[u] + Graph.Edges(u, v)
        18              if alt < dist[v]:
        19                  prev[v] ← u
        20                  dist[v] ← alt
        21                  Q.decrease_priority(v, alt)
        22
        23      return dist, prev
    */
    let mut heap: HeapType = BinaryHeap::new();
    let mut dist: DistType = BTreeMap::new();

    // We'll keep track of all valid predecessors
    let mut prev: PrevType = BTreeMap::new();

    dist.insert(start, 0);
    heap.push(State {
        pos: start,
        cost: 0,
        path: vec![start],
    });

    let mut best = usize::MAX;
    let mut paths = Vec::new();

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { pos, cost, path }) = heap.pop() {
        // Important as we may have already found a better way
        let &prev_cost = dist.get(&pos).unwrap_or(&usize::MAX);
        if cost > prev_cost {
            continue;
        }

        // Alternatively we could have continued to find all shortest paths
        if pos == end && cost < best {
            best = cost;
            paths.push(path.clone());
        }

        for next_move in possible_moves(grid, pos) {
            let &past_move_cost = dist.get(&next_move).unwrap_or(&usize::MAX);
            let next_cost = cost + 1;
            if next_cost < past_move_cost {
                let mut new_path = path.clone();
                new_path.push(next_move);
                heap.push(State {
                    pos: next_move,
                    cost: next_cost,
                    path: new_path,
                });
                /*if next_cost == past_move_cost {
                    //println!("Updating predecessors");
                    let prior_set = prev.get_mut(&next_move).unwrap();
                    prior_set.insert(pos);
                } else {*/
                // Relaxation, we have now found a better way
                let mut new_set = HashSet::new();
                // We create a new set with just the single predecessor here
                new_set.insert(pos);
                prev.insert(next_move, new_set);
                //}
                dist.insert(next_move, next_cost);
            }
        }
    }

    (best, dist, prev, paths)
}

#[derive(Debug)]
struct Memory {
    grid: Grid,
    width: usize,
    height: usize,
    start: Coord,
    end: Coord,
    corrupted: Vec<Coord>,
    num_corrupted: usize,
}

impl Memory {
    fn new(lines: &[String], dim: usize) -> Self {
        // Create an empty fill to make the space usable
        let mut grid = [['.'; MAX_DIM]; MAX_DIM];
        let height = dim;
        let width = dim;
        let mut corrupted = Vec::new();
        let num_corrupted = 0;

        // Create borders around everything
        for x in 0..=width {
            grid[0][x] = INVALID;
            grid[dim][x] = INVALID;
        }
        #[allow(clippy::needless_range_loop)]
        for y in 1..=height {
            grid[y][0] = INVALID;
            grid[y][dim] = INVALID;
        }
        let start = (1, 1);
        let end = (dim - 1, dim - 1);

        for line in lines {
            let (x_str, y_str) = line.split_once(',').unwrap();
            let x = x_str.parse::<usize>().unwrap();
            let y = y_str.parse::<usize>().unwrap();
            corrupted.push((x, y));
        }

        Self {
            grid,
            width,
            height,
            start,
            end,
            corrupted,
            num_corrupted,
        }
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        println!("width: {} height: {}", self.width, self.height);
        println!("start: {:?} end: {:?}", self.start, self.end);
        for y in 0..=self.height {
            for x in 0..=self.width {
                print!("{}", self.grid[y][x]);
            }
            println!();
        }
    }

    fn reset_grid(&mut self) {
        for y in 1..self.height {
            for x in 1..self.width {
                self.grid[y][x] = '.';
            }
        }
        self.num_corrupted = 0;
    }

    fn corrupt(&mut self, num: usize) -> &mut Self {
        if self.num_corrupted + num > self.corrupted.len() {
            panic!("Attempt to corrupt too far");
        }
        for &(x, y) in self.corrupted[self.num_corrupted..].iter().take(num) {
            self.grid[y + 1][x + 1] = INVALID;
        }
        self.num_corrupted += num;
        self
    }

    fn shortest_path(&self) -> usize {
        let (steps, _, _, _) = dijkstra(&self.grid, self.start, self.end);
        steps
    }

    fn shortest_path_verbose(&self) -> usize {
        let steps = self.shortest_path();
        println!("Steps: {steps}");
        steps
    }

    fn get_corrupt_coord(&mut self, base_corrupt: usize) -> Coord {
        let mut lower = base_corrupt;
        let mut upper = self.corrupted.len();
        self.reset_grid();
        if self.corrupt(lower).shortest_path() == usize::MAX {
            panic!("Lower bound doesn't work");
        }
        self.reset_grid();
        if self.corrupt(upper).shortest_path() != usize::MAX {
            panic!("Upper bound doesn't work");
        }

        loop {
            if upper == lower + 1 {
                self.reset_grid();
                if self.corrupt(lower).shortest_path() == usize::MAX {
                    panic!("Lower bound doesn't work");
                }
                self.reset_grid();
                if self.corrupt(upper).shortest_path() != usize::MAX {
                    panic!("Upper bound doesn't work");
                }

                break;
            }
            let c = (upper + lower) / 2;
            self.reset_grid();
            self.corrupt(c);
            let steps = self.shortest_path();
            if steps == usize::MAX {
                upper = c;
            } else {
                lower = c;
            }
        }

        let point = self.corrupted[lower];
        println!("corrupt: {:?}", point);
        point
    }
}

#[test]
fn test_prelim() {
    let mut memory = Memory::new(&get_input("prelim.txt"), 8);
    memory.corrupt(12);
    let steps = memory.shortest_path_verbose();
    assert_eq!(steps, 22);
}

#[test]
fn test_part1() {
    let steps = Memory::new(&get_input("input.txt"), 72)
        .corrupt(1024)
        .shortest_path_verbose();
    assert_eq!(steps, 252);
}

#[test]
fn test_prelim2() {
    let mut memory = Memory::new(&get_input("prelim.txt"), 8);
    let pos = memory.get_corrupt_coord(12);
    assert_eq!(pos, (6, 1));
}

#[test]
fn test_part2() {
    let mut memory = Memory::new(&get_input("input.txt"), 72);
    let pos = memory.get_corrupt_coord(1024);
    assert_eq!(pos, (5, 60));
}

fn main() {
    let mut mem = Memory::new(&get_input("prelim.txt"), 8);
        mem.corrupt(12)
        .shortest_path_verbose();
    mem.print_grid();
    Memory::new(&get_input("input.txt"), 72)
        .corrupt(1024)
        .shortest_path_verbose();

    let mut memory = Memory::new(&get_input("prelim.txt"), 8);
    memory.get_corrupt_coord(12);

    let mut memory = Memory::new(&get_input("input.txt"), 72);
    memory.get_corrupt_coord(1024);
}
