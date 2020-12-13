use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let timestamp: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for line in lines {
        let line = line.unwrap();
        let busses: Vec<_> = line.split(",").filter(|x| *x != "x").map(|x| x.parse::<usize>().unwrap()).collect();
        let mut departures: Vec<(usize, usize)> = busses
            .iter()
            .map(|b| match timestamp % b {
                0 => (*b, 0),
                o @ _ => (*b, b - o),
            })
            .collect();
        departures.sort_by_key(|(bus, minutes)| *minutes);
        let departure = departures.first().unwrap();
        println!("departure {:?}", departure);
        println!("result {:?}", departure.0 * departure.1);
    }
}
