use std::io::{self, BufRead};
use std::ops::Range;
use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Policy {
    range: Range<usize>,
    letter: char,
}

#[derive(Clone, Debug)]
struct Item {
    policy: Policy,
    password: String,
}

impl Item {
    fn validate(&self) -> bool {
        let occurences = self.password.chars().filter(|c| *c == self.policy.letter).count();
        self.policy.range.contains(&occurences)
    }

    fn validate_two(&self) -> bool {
        let first = self.password.chars().nth(self.policy.range.start - 1) == Some(self.policy.letter);
        let second = self.password.chars().nth(self.policy.range.end - 2) == Some(self.policy.letter);
        first ^ second
    }
}

fn main() {
    let items = stdin_to_list();
    let valid = items.iter()
        .filter(|x| x.validate())
        .count();
    println!("{} valid", valid);

    let valid_two = items.iter()
        .filter(|x| x.validate_two())
        .count();
    println!("{} valid two", valid_two);
}

fn stdin_to_list() -> Vec<Item> {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
    let mut list = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = &line.unwrap();
        let data = re.captures(line).unwrap();
        let min: usize = data.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = data.get(2).unwrap().as_str().parse().unwrap();
        let letter = data.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password = data.get(4).unwrap().as_str().to_string();
        let item = Item {
            policy: Policy {
                range: (min..max + 1),
                letter,
            },
            password,
        };

        list.push(item);
    }

    list
}
