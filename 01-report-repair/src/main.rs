use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let set = stdin_to_set();

    let target = 2020;

    let pairs = find_sum_pair(target, &set);
    for pair in pairs {
        println!("{} * {} = {}", pair.0, pair.1, pair.0 * pair.1);
    }

    let triples = find_sum_triples(target, &set);
    for triple in triples {
        println!("{} * {} * {} = {}", triple.0, triple.1, triple.2, triple.0 * triple.1 * triple.2);
    }
}

fn stdin_to_set() -> HashSet<u32> {
    let stdin = io::stdin();
    let mut set = HashSet::new();
    for line in stdin.lock().lines() {
        let number: u32 = line.unwrap().parse().unwrap();
        set.insert(number);
    }

    set
}

fn find_sum_pair(target: u32, set: &HashSet<u32>) -> Vec<(u32, u32)> {
    let mut results = Vec::new();
    for number in set.iter() {
        if target < *number {
            continue;
        }

        let other = target - *number;
        if number < &other && set.contains(&other) {
            results.push((*number, other));
        }
    }

    results
}

fn find_sum_triples(target: u32, set: &HashSet<u32>) -> Vec<(u32, u32, u32)> {
    let mut results = Vec::new();
    for number in set.iter() {
        let rest = target - number;
        let pairs = find_sum_pair(rest, set);
        for pair in pairs {
            if *number < pair.0 {
                results.push((*number, pair.0, pair.1));
            }
        }
    }

    results
}
