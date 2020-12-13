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

        let mut offset = 0;
        let mut product = busses[0].unwrap();

        for (i, bus) in busses.iter().enumerate().skip(1) {
            if let Some(bus) = bus {
                let mut i = i;
                while i > *bus {
                    i -= bus;
                }

                while offset % bus != bus - i {
                    offset += product;
                }

                product *= bus;
            }
        }

        println!("offset {}", offset);
    }
}
