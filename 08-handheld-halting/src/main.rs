use std::collections::HashSet;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Type {
    Nop,
    Acc,
    Jmp,
}

impl Type {
    fn is_flippable(&self) -> bool {
        use Type::*;
        match self {
            Nop => true,
            Jmp => true,
            _ => false,
        }
    }

    fn flip(&self) -> Self {
        use Type::*;
        match self {
            Nop => Jmp,
            Jmp => Nop,
            o @ _ => *o,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    typ: Type,
    data: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr = &s[0..3];
        let typ = match instr {
            "nop" => Type::Nop,
            "acc" => Type::Acc,
            "jmp" => Type::Jmp,
            _ => panic!("unrecognized instruction {}", instr),
        };

        let data = &s[4..];
        let data = data.parse::<i32>().unwrap();

        Ok(Instruction { typ, data })
    }
}

#[derive(Debug, Clone)]
struct Program {
    code: Vec<Instruction>,
    visited: HashSet<usize>,
    accumulator: i32,
    program_counter: usize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Terminate {
    Done,
    Repeat,
    Error,
}

impl Program {
    fn run(&mut self) -> Terminate {
        loop {
            let not_visited = self.visited.insert(self.program_counter);
            if !not_visited {
                return Terminate::Repeat;
            }

            use Type::*;
            match self.code.get(self.program_counter) {
                Some(Instruction { typ: Nop, data: _ }) => {
                    self.program_counter += 1;
                }
                Some(Instruction { typ: Acc, data }) => {
                    self.accumulator += data;
                    self.program_counter += 1;
                }
                Some(Instruction { typ: Jmp, data }) => {
                    self.program_counter =
                        (self.program_counter as isize + *data as isize) as usize;
                }
                None if self.program_counter == self.code.len() => {
                    return Terminate::Done;
                }
                None => {
                    return Terminate::Error;
                }
            }
        }
    }

    fn reset(&mut self) {
        self.visited.clear();
        self.accumulator = 0;
        self.program_counter = 0;
    }

    fn find_flip(&mut self) -> Option<usize> {
        for index in (0..self.code.len()) {
            if !self.code[index].typ.is_flippable() {
                continue;
            }

            self.code[index].typ = self.code[index].typ.flip();

            self.reset();
            let result = self.run();
            if result == Terminate::Done {
                return Some(index);
            }

            self.code[index].typ = self.code[index].typ.flip();
        }

        None
    }
}

fn main() {
    let mut program = stdin_to_program();
    program.run();
    println!("accumulator is {}", program.accumulator);

    let flip = program.find_flip();
    println!("flip is {:?}", flip);
    println!("accumulator is {}", program.accumulator);
}

fn stdin_to_program() -> Program {
    let mut code = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let instruction = line.parse::<Instruction>().unwrap();
        code.push(instruction);
    }

    Program {
        code,
        visited: HashSet::new(),
        accumulator: 0,
        program_counter: 0,
    }
}
