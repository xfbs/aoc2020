use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Seat {
    row: usize,
    column: usize,
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s[0..7]
            .chars()
            .map(|c| match c {
                'F' => 0,
                'B' => 1,
                _ => panic!(),
            })
            .fold(0, |r, c| 2 * r + c);
        let column = s[7..10]
            .chars()
            .map(|c| match c {
                'R' => 1,
                'L' => 0,
                _ => panic!(),
            })
            .fold(0, |r, c| 2 * r + c);

        Ok(Seat { row, column })
    }
}

impl Seat {
    fn seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

#[test]
fn test_seat_from_str() {
    assert_eq!(
        "FFFFFFFLLL".parse::<Seat>().unwrap(),
        Seat { row: 0, column: 0 }
    );
    assert_eq!(
        "FFFFFFBLLR".parse::<Seat>().unwrap(),
        Seat { row: 1, column: 1 }
    );
    assert_eq!(
        "BBBBBBBRRR".parse::<Seat>().unwrap(),
        Seat {
            row: 127,
            column: 7
        }
    );
    assert_eq!(
        "FBFBBFFRLR".parse::<Seat>().unwrap(),
        Seat { row: 44, column: 5 }
    );
    assert_eq!(
        "BFFFBBFRRR".parse::<Seat>().unwrap(),
        Seat { row: 70, column: 7 }
    );
    assert_eq!(
        "FFFBBBFRRR".parse::<Seat>().unwrap(),
        Seat { row: 14, column: 7 }
    );
    assert_eq!(
        "BBFFBBFRLL".parse::<Seat>().unwrap(),
        Seat {
            row: 102,
            column: 4
        }
    );
}

fn main() {
    let seats = stdin_to_seats();
    let mut seat_ids: Vec<_> = seats.iter().map(|n| n.seat_id()).collect();
    seat_ids.sort();
    println!("min is {}", seat_ids.first().unwrap());
    println!("max is {}", seat_ids.last().unwrap());
    let missing = seat_ids
        .iter()
        .zip(seat_ids.iter().skip(1))
        .find(|(a, b)| **b - **a != 1)
        .unwrap();
    println!("missing seat is {}", missing.0 + 1);
}

fn stdin_to_seats() -> Vec<Seat> {
    let mut seats = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let seat = line.parse::<Seat>().unwrap();
        seats.push(seat);
    }

    seats
}
