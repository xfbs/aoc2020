use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Terrain {
    Tree,
    Open,
}

struct Map {
    terrain: Vec<Vec<Terrain>>,
}

impl Map {
    fn get_terrain(&self, x: usize, y: usize) -> Option<Terrain> {
        self.terrain
            .get(y)
            .and_then(|row| row.get(x % row.len()))
            .map(|x| *x)
    }

    fn count_trees(&self, x: usize, y: usize) -> usize {
        (0..self.terrain.len())
            .map(|n| (x * n, y * n))
            .map(|(x, y)| self.get_terrain(x, y))
            .take_while(|terrain| terrain.is_some())
            .map(|terrain| terrain.unwrap())
            .filter(|terrain| *terrain == Terrain::Tree)
            .count()
    }
}

fn main() {
    let map = stdin_to_map();
    let count = map.count_trees(3, 1);
    println!("trees = {}", count);

    // part two
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: usize = slopes.iter().map(|(x, y)| map.count_trees(*x, *y)).product();
    println!("product = {}", product);
}

fn stdin_to_map() -> Map {
    let stdin = io::stdin();
    let terrain = stdin
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| match c {
                    '.' => Terrain::Open,
                    '#' => Terrain::Tree,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    Map { terrain }
}
