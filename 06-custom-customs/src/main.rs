use std::collections::HashSet;
use std::io::{self, BufRead};
use std::iter::once;

fn main() {
    let mut set_any = HashSet::new();
    let mut set_all: Option<HashSet<char>> = None;
    let mut count_any = 0;
    let mut count_all = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines().chain(once(Ok("".to_string()))) {
        let line = line.unwrap();
        if line.len() == 0 {
            count_any += set_any.len();
            set_any.clear();
            count_all += set_all.as_ref().map(|n| n.len()).unwrap_or(0);
            set_all = None;
            continue;
        }

        let mut set_cur = HashSet::new();
        for c in line.chars() {
            set_any.insert(c);
            set_cur.insert(c);
        }

        set_all = match set_all {
            None => Some(set_cur),
            Some(set) => Some(set.intersection(&set_cur).map(|n| *n).collect()),
        };
    }

    println!("count is {}", count_any);
    println!("count_all is {}", count_all);
}
