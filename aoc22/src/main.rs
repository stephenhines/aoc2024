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
fn test_part1() {
    let sum = compute_secret_n_sum(&read_secret_numbers(&get_input("input.txt")), 2000);
    assert_eq!(sum, 19458130434);
}

fn main() {
    compute_secret_n_sum(&read_secret_numbers(&get_input("prelim.txt")), 2000);
    compute_secret_n_sum(&read_secret_numbers(&get_input("input.txt")), 2000);
}
