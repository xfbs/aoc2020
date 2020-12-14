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

    fn apply_mask(&self, value: u64) -> u64 {
        let value = value & !self.mask_clear;
        let value = value | self.mask_set;
        value
    }

    fn handle_mem(&mut self, captures: Captures) {
        let location = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let value = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
        let value = self.apply_mask(value);
        self.memory.insert(location, value);
    }

    fn handle_mem2(&mut self, captures: Captures) {
        let location = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let value = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();

        // apply 1s from mask
        let location = location | self.mask_set;

        // compute Xes
        let mut toggles = ((1 << 36) - 1) & !self.mask_clear & !self.mask_set;

        // clear current Xes
        let location = location & !toggles;

        // identify the locations of all ones
        let mut ones = Vec::new();
        let mut index = 0;
        while toggles != 0 {
            if toggles & 1 == 1 {
                ones.push(index);
            }

            index += 1;
            toggles >>= 1;
        }

        // compute all possibilities for the toggles and set memory at those
        // locations
        for x in (0..1 << ones.len()) {
            let mut location = location;
            let mut x = x;
            for one in &ones {
                let bit = (1 << one) * (x & 1);
                location |= bit;
                x >>= 1;
            }

            self.memory.insert(location, value);
        }
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

    fn handle2(&mut self, line: &str) {
        lazy_static! {
            static ref MASK_REGEX: Regex = Regex::new(r"^mask = ([X01]{36})$").unwrap();
            static ref MEM_REGEX: Regex = Regex::new(r"^mem\[([0-9]+)\] = ([0-9]+)$").unwrap();
        }

        let functions: [(&Regex, fn(&mut State, Captures)); 2] = [
            (&MASK_REGEX, State::handle_mask),
            (&MEM_REGEX, State::handle_mem2),
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
    let mut state2 = State::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        state.handle(&line);
        state2.handle2(&line);
    }

    println!("sum is {}", state.sum());
    println!("sum2 is {}", state2.sum());
}
