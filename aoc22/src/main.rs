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

const MAX_STEPS: usize = 2000 + 1;

type Secrets = [u64; MAX_STEPS];
type Price = [i64; MAX_STEPS];
type Diff = [i64; MAX_STEPS];

type Quad = (i8, i8, i8, i8);
type QuadMap = HashMap<Quad, i64>;

#[derive(Debug)]
struct BananaMarket {
    secrets: Vec<Secrets>,
    prices: Vec<Price>,
    diffs: Vec<Diff>,
    quad_map: QuadMap,
}

impl BananaMarket {
    fn new() -> Self {
        BananaMarket {
            secrets: Vec::new(),
            prices: Vec::new(),
            diffs: Vec::new(),
            quad_map: HashMap::new(),
        }
    }

    fn from_vec(start_secrets: &Vec<u64>) -> Self {
        let mut market = BananaMarket::new();
        for secret in start_secrets {
            market.add_bidder(*secret);
        }

        market
    }

    fn add_bidder(&mut self, start_secret: u64) {
        let mut secrets: Secrets = [0; MAX_STEPS];
        let mut prices: Price = [0; MAX_STEPS];
        let mut diffs: Diff = [0; MAX_STEPS];
        let mut last_price = (start_secret % 10) as i64;
        let mut secret = start_secret;

        let mut seen_set = HashSet::new();

        for i in 0..MAX_STEPS {
            secrets[i] = secret;
            prices[i] = (secret % 10) as i64;
            diffs[i] = prices[i] - last_price;
            last_price = prices[i];

            // We use i > 3, because technically the first price doesn't
            // really have a difference to it.
            if i > 3 {
                let quad = (
                    diffs[i - 3] as i8,
                    diffs[i - 2] as i8,
                    diffs[i - 1] as i8,
                    diffs[i] as i8,
                );
                if !seen_set.contains(&quad) {
                    if let std::collections::hash_map::Entry::Vacant(e) = self.quad_map.entry(quad)
                    {
                        e.insert(prices[i]);
                    } else {
                        *self.quad_map.get_mut(&quad).unwrap() += prices[i];
                    }
                    seen_set.insert(quad);
                }
            }
            secret = compute_secret(secret);
        }
        self.secrets.push(secrets);
        self.prices.push(prices);
        self.diffs.push(diffs);
    }

    // Enumerate all possible quads of value changes
    fn find_diff_quads(&self) -> HashSet<(i64, i64, i64, i64)> {
        let mut quads = HashSet::new();

        for diff in &self.diffs {
            for i in 0..MAX_STEPS - 3 {
                let a = diff[i];
                let b = diff[i + 1];
                let c = diff[i + 2];
                let d = diff[i + 3];
                let quad = (a, b, c, d);
                quads.insert(quad);
            }
        }

        quads
    }

    // Find the first instance of the diff quad for a particular bidder and return their final price
    fn evaluate_single_diff_quad(&self, num: usize, diff_quad: &(i64, i64, i64, i64)) -> i64 {
        let diff = &self.diffs[num];
        let prices = &self.prices[num];

        for i in 0..MAX_STEPS - 3 {
            if diff[i] == diff_quad.0
                && diff[i + 1] == diff_quad.1
                && diff[i + 2] == diff_quad.2
                && diff[i + 3] == diff_quad.3
            {
                return prices[i + 3];
            }
        }

        0
    }

    fn evaluate_diff_quad(&self, diff_quad: &(i64, i64, i64, i64)) -> i64 {
        let mut total = 0;
        let num_bidders = self.prices.len();

        for i in 0..num_bidders {
            total += self.evaluate_single_diff_quad(i, diff_quad);
        }

        total
    }

    fn find_best_total_lookup(&self) -> i64 {
        let mut best_quad = (0, 0, 0, 0);
        let mut best = 0;
        for (&k, &v) in self.quad_map.iter() {
            if v > best {
                best = v;
                best_quad = k;
            }
        }

        println!("Best price: {best}");
        println!("Best quad: {:?}", best_quad);

        best
    }

    fn find_best_total_brute_force(&self) -> i64 {
        let mut best = 0;

        let quads = self.find_diff_quads();
        let mut best_quad = (0, 0, 0, 0);
        for quad in quads {
            let val = self.evaluate_diff_quad(&quad);
            if val > best {
                best = val;
                best_quad = quad;
            }
        }

        println!("Best price: {best}");
        println!("Best quad: {:?}", best_quad);

        best
    }
}

fn read_secret_numbers(lines: &Vec<String>) -> Vec<u64> {
    let mut result = Vec::new();
    for line in lines {
        let v = line.parse::<u64>().unwrap();
        result.push(v);
    }

    result
}

fn mix(secret: u64, mix_val: u64) -> u64 {
    secret ^ mix_val
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn compute_secret(val: u64) -> u64 {
    let mut result = val;
    let mul_res = result * 64; // Multiply by 64
    result = mix(result, mul_res); // Mix
    result = prune(result); // Prune

    let div_res = result / 32; // Divide by 32
    result = mix(result, div_res); // Mix
    result = prune(result); // Prune

    let mul_res = result * 2048; // Multiply by 2048
    result = mix(result, mul_res); // Mix
    result = prune(result); // Prune

    result
}

fn compute_secret_n(val: u64, n: u64) -> u64 {
    let mut secret = val;
    for _ in 0..n {
        secret = compute_secret(secret);
    }
    secret
}

fn compute_secret_n_sum(secrets: &Vec<u64>, n: u64) -> u64 {
    let mut sum = 0;

    for secret in secrets {
        let new_secret = compute_secret_n(*secret, n);
        sum += new_secret;
    }

    println!("Sum: {sum}");
    sum
}

#[test]
fn test_compute_secret_10() {
    let mut secrets = Vec::new();
    secrets.push(123);
    for i in 0..10 {
        let secret = secrets[i];
        let new_secret = compute_secret(secret);
        secrets.push(new_secret);
    }

    let expect_vec = vec![
        123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
        5908254,
    ];

    assert_eq!(secrets, expect_vec);
}

#[test]
fn test_prelim() {
    let sum = compute_secret_n_sum(&read_secret_numbers(&get_input("prelim.txt")), 2000);
    assert_eq!(sum, 37327623);
}

#[test]
fn test_prelim2() {
    let total = BananaMarket::from_vec(&read_secret_numbers(&get_input("prelim2.txt")))
        .find_best_total_brute_force();
    assert_eq!(total, 23);
    let total = BananaMarket::from_vec(&read_secret_numbers(&get_input("prelim2.txt")))
        .find_best_total_lookup();
    assert_eq!(total, 23);
}

#[test]
fn test_part1() {
    let sum = compute_secret_n_sum(&read_secret_numbers(&get_input("input.txt")), 2000);
    assert_eq!(sum, 19458130434);
}

#[test]
fn test_part2() {
    let total = BananaMarket::from_vec(&read_secret_numbers(&get_input("input.txt")))
        .find_best_total_lookup();
    assert_eq!(total, 2130);
}

fn main() {
    compute_secret_n_sum(&read_secret_numbers(&get_input("prelim.txt")), 2000);
    compute_secret_n_sum(&read_secret_numbers(&get_input("input.txt")), 2000);
    let market = BananaMarket::from_vec(&read_secret_numbers(&get_input("prelim2.txt")));
    market.find_best_total_brute_force();
    market.find_best_total_lookup();
    let market = BananaMarket::from_vec(&read_secret_numbers(&get_input("input.txt")));
    market.find_best_total_lookup();
    //market.find_best_total_brute_force();
}
