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

// Our input.txt is 141x141
const MAX_DIM: usize = 150;
type Grid = [[char; MAX_DIM]; MAX_DIM];

// Use an invalid char as a border to simplify bounds checking of indices
const INVALID: char = ' ';

type Coord = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Facing {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug)]
struct Reindeer {
    pos: Coord,
    face: Facing,
    steps: usize,
    rotates: usize,
}

// `Eq` and `PartialEq` need to be implemented.
impl Eq for Reindeer {}
impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.face == other.face
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos
            .cmp(&other.pos)
            .then_with(|| self.face.cmp(&other.face))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Reindeer {
    fn new(pos: Coord, face: Facing) -> Self {
        Self {
            pos,
            face,
            steps: 0,
            rotates: 0,
        }
    }

    fn cost(&self) -> usize {
        self.rotates * 1000 + self.steps
    }

    fn rotate_clockwise(&mut self) {
        self.face = match self.face {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        };
        self.rotates += 1;
    }

    fn rotate_counterclockwise(&mut self) {
        self.face = match self.face {
            Facing::North => Facing::West,
            Facing::East => Facing::North,
            Facing::South => Facing::East,
            Facing::West => Facing::South,
        };
        self.rotates += 1;
    }

    fn move_forward(&mut self) {
        match self.face {
            Facing::North => {
                self.pos.1 -= 1;
            }
            Facing::East => {
                self.pos.0 += 1;
            }
            Facing::South => {
                self.pos.1 += 1;
            }
            Facing::West => {
                self.pos.0 -= 1;
            }
        }
        self.steps += 1;
    }

    fn move_left(&mut self) {
        self.rotate_counterclockwise();
        self.move_forward();
    }

    fn move_right(&mut self) {
        self.rotate_clockwise();
        self.move_forward();
    }
}

fn in_bounds(grid: &Grid, pos: Coord) -> bool {
    match grid[pos.1][pos.0] {
        '#' => false,
        '.' => true,
        'S' | 'E' => true,
        c => panic!("Invalid char: {c}"),
    }
}

fn possible_moves(grid: &Grid, r: Reindeer) -> Vec<Reindeer> {
    let mut possible_moves = Vec::new();

    let mut r_forward = r;
    r_forward.move_forward();
    if in_bounds(grid, r_forward.pos) {
        possible_moves.push(r_forward);
    }

    let mut r_left = r;
    r_left.move_left();
    if in_bounds(grid, r_left.pos) {
        possible_moves.push(r_left);
    }

    let mut r_right = r;
    r_right.move_right();
    if in_bounds(grid, r_right.pos) {
        possible_moves.push(r_right);
    }

    possible_moves
}

#[derive(Eq, PartialEq)]
struct State {
    r: Reindeer,
    path: Vec<Coord>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .r
            .cost()
            .cmp(&self.r.cost())
            .then_with(|| self.r.cmp(&other.r))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type DistType = BTreeMap<Reindeer, usize>;
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

    let r = Reindeer::new(start, Facing::East);
    dist.insert(r, 0);
    heap.push(State {
        r,
        path: vec![start],
    });

    let mut score = usize::MAX;
    let mut paths = Vec::new();

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { r, path }) = heap.pop() {
        // Important as we may have already found a better way
        let &prev_cost = dist.get(&r).unwrap_or(&usize::MAX);
        if r.cost() > prev_cost {
            continue;
        }

        // Alternatively we could have continued to find all shortest paths
        if r.pos == end && r.cost() <= score {
            score = r.cost();
            paths.push(path.clone());
        }

        for r_move in possible_moves(grid, r) {
            let &past_move_cost = dist.get(&r_move).unwrap_or(&usize::MAX);
            if r_move.cost() <= past_move_cost {
                let mut new_path = path.clone();
                new_path.push(r_move.pos);
                heap.push(State {
                    r: r_move,
                    path: new_path,
                });
                if r_move.cost() == past_move_cost {
                    //println!("Updating predecessors");
                    let prior_set = prev.get_mut(&r_move.pos).unwrap();
                    prior_set.insert(r.pos);
                } else {
                    // Relaxation, we have now found a better way
                    let mut new_set = HashSet::new();
                    // We create a new set with just the single predecessor here
                    new_set.insert(r.pos);
                    prev.insert(r_move.pos, new_set);
                }
                dist.insert(r_move, r_move.cost());
            }
        }
    }

    (score, dist, prev, paths)
}

#[derive(Debug)]
struct Maze {
    grid: Grid,
    width: usize,
    height: usize,
    start: Coord,
    end: Coord,
}

impl Maze {
    fn new(lines: &[String]) -> Self {
        let mut grid = [[INVALID; MAX_DIM]; MAX_DIM];
        let height = lines.len();
        let width = lines[0].len();
        let unknown = (0, 0);

        for y in 0..height {
            let line = lines[y].chars().collect::<Vec<_>>();
            grid[y][0..width].copy_from_slice(&line);
        }

        let mut maze = Self {
            grid,
            width,
            height,
            start: unknown,
            end: unknown,
        };
        maze.start = maze.find('S');
        maze.end = maze.find('E');

        maze
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        println!("width: {} height: {}", self.width, self.height);
        println!("start: {:?} end: {:?}", self.start, self.end);
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.grid[y][x]);
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print_grid2(&self, tiles: &HashSet<Coord>) {
        println!("width: {} height: {}", self.width, self.height);
        println!("start: {:?} end: {:?}", self.start, self.end);
        for y in 0..self.height {
            for x in 0..self.width {
                if tiles.contains(&(x, y)) {
                    print!("O");
                } else {
                    print!("{}", self.grid[y][x]);
                }
            }
            println!();
        }
    }

    fn find(&self, c: char) -> Coord {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == c {
                    return (x, y);
                }
            }
        }
        (0, 0)
    }

    fn best_score(&self) -> usize {
        let (score, _, _, _) = dijkstra(&self.grid, self.start, self.end);

        println!("Score: {score}");
        score
    }

    fn best_path_tiles(&self) -> usize {
        /*let (_, prev, _) = dijkstra(&self.grid, self.start, self.end);

        let mut tiles = HashSet::new();
        let mut work_vec = Vec::new();
        work_vec.push(self.end);
        while let Some(pos) = work_vec.pop() {
            tiles.insert(pos);
            if let Some(preds) = prev.get(&pos) {
                preds.iter().for_each(|&x| work_vec.push(x));
            }
        }*/

        let (_, _, _, paths) = dijkstra(&self.grid, self.start, self.end);

        let mut tiles = HashSet::new();
        for path in paths {
            for tile in path {
                tiles.insert(tile);
            }
        }

        //self.print_grid2(&tiles);
        println!("Best path tiles: {}", tiles.len());
        tiles.len()
    }
}

#[test]
fn test_prelim() {
    let score = Maze::new(&get_input("prelim.txt")).best_score();
    assert_eq!(score, 7036);
}

#[test]
fn test_part1() {
    let score = Maze::new(&get_input("input.txt")).best_score();
    assert_eq!(score, 95444);
}

#[test]
fn test_prelim2() {
    let tiles = Maze::new(&get_input("prelim.txt")).best_path_tiles();
    assert_eq!(tiles, 45);
}

#[test]
fn test_part2() {
    let tiles = Maze::new(&get_input("input.txt")).best_path_tiles();
    assert_eq!(tiles, 513);
}

fn main() {
    let maze = Maze::new(&get_input("prelim.txt"));
    maze.best_score();
    Maze::new(&get_input("input.txt")).best_score();
    maze.best_path_tiles();
    Maze::new(&get_input("input.txt")).best_path_tiles();
}
