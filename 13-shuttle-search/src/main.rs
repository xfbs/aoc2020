use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let timestamp: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for line in lines {
        let line = line.unwrap();
        let busses: Vec<_> = line
            .split(",")
            .filter(|x| *x != "x")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
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

        // second part
        let busses: Vec<Option<usize>> = line
            .split(",")
            .map(|x| match x {
                "x" => None,
                _ => Some(x.parse::<usize>().unwrap()),
            })
            .collect();

        // offset is the actual timestamp, product is the granularity at which
        // we can move. in the beginning, we are at timestamp 0, and we can
        // move by the bus[0] - because any multiple of the first bus id will
        // have the property that it will be divisible by the bus[0] id.
        let mut offset = 0;
        let mut product = busses[0].unwrap();

        // what we do is we increase our offset by the product to find the
        // next biggest number such that the current bus will leave i minutes
        // after.
        for (i, bus) in busses.iter().enumerate().skip(1) {
            if let Some(bus) = bus {
                while offset % bus != bus - (i % bus) {
                    offset += product;
                }

                product *= bus;
            }
        }

        println!("offset {}", offset);
    }
}
