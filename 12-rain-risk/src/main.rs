use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    action: Action,
    value: usize
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Action::*;
        let action = match s.chars().nth(0).unwrap() {
            'N' => North,
            'S' => South,
            'E' => East,
            'W' => West,
            'R' => Right,
            'L' => Left,
            'F' => Forward,
            e @ _ => panic!("unrecognized action {}", e),
        };

        let value = s[1..s.len()].parse::<usize>().unwrap();

        Ok(Instruction {
            action,
            value,
        })
    }
}

#[derive(Copy, Clone, Debug)]
struct State {
    heading: usize,
    north: isize,
    east: isize,
}

impl State {
    fn new() -> Self {
        State {
            heading: 90,
            north: 0,
            east: 0,
        }
    }

    fn forward(&mut self, distance: usize) {
        use Action::*;
        let dir = match self.heading {
            0 => North,
            180 => South,
            90 => East,
            270 => West,
            _ => unreachable!(),
        };
        let instruction = Instruction {
            action: dir,
            value: distance,
        };
        self.run(&instruction);
    }

    fn run(&mut self, i: &Instruction) {
        use Action::*;
        match i.action {
            North => self.north += i.value as isize,
            South => self.north -= i.value as isize,
            East => self.east += i.value as isize,
            West => self.east -= i.value as isize,
            Right => {
                self.heading += i.value as usize;
                self.heading %= 360;
            },
            Left => {
                self.heading += 360;
                self.heading -= i.value as usize;
                self.heading %= 360;
            }
            Forward => {
                self.forward(i.value);
            }
        }
    }

    fn manhattan(&self) -> usize {
        self.north.abs() as usize + self.east.abs() as usize
    }
}

fn main() {
    let instructions = stdin_to_instructions();
    let mut state = State::new();
    for instruction in instructions {
        state.run(&instruction);
    }

    println!("distance {}", state.manhattan());
}

fn stdin_to_instructions() -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let instruction = line.parse::<Instruction>().unwrap();
        instructions.push(instruction);
    }

    instructions
}
