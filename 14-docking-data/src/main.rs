#[macro_use]
extern crate lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::io::{self, BufRead};

enum Command {
    Mask(String),
    Mem(u64, u64),
}

struct State {
    memory: HashMap<u64, u64>,
    mask_clear: u64,
    mask_set: u64,
}

impl State {
    fn new() -> Self {
        State {
            memory: HashMap::new(),
            mask_clear: 0,
            mask_set: 0,
        }
    }

    fn handle_mem(&mut self, captures: Captures) {
        let location = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let value = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
        let value = value & !self.mask_clear;
        let value = value | self.mask_set;
        self.memory.insert(location, value);
    }

    fn handle_mask(&mut self, captures: Captures) {
        let mask = captures.get(1).unwrap().as_str();
        self.mask_clear = mask
            .chars()
            .map(|c| c == '0')
            .fold(0, |a, i| 2 * a + (i as u64));
        self.mask_set = mask
            .chars()
            .map(|c| c == '1')
            .fold(0, |a, i| 2 * a + (i as u64));
    }

    fn handle(&mut self, line: &str) {
        lazy_static! {
            static ref MASK_REGEX: Regex = Regex::new(r"^mask = ([X01]{36})$").unwrap();
            static ref MEM_REGEX: Regex = Regex::new(r"^mem\[([0-9]+)\] = ([0-9]+)$").unwrap();
        }

        let functions: [(&Regex, fn(&mut State, Captures)); 2] = [
            (&MASK_REGEX, State::handle_mask),
            (&MEM_REGEX, State::handle_mem),
        ];

        for item in &functions {
            if let Some(captures) = item.0.captures(line) {
                return item.1(self, captures);
            }
        }

        panic!("couldn't parse {}", line);
    }

    fn sum(&self) -> u64 {
        self.memory.values().sum::<u64>()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut state = State::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        state.handle(&line);
    }

    println!("sum is {}", state.sum());
}
