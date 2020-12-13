use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color {
            red: 0,
            green: 0,
            blue: 0,
        })
    }
}

#[derive(Default, Clone, Debug)]
struct Passport {
    birth_year: Option<i32>,
    issue_year: Option<i32>,
    expiration_year: Option<i32>,
    height: Option<String>,
    hair_color: Option<Color>,
    eye_color: Option<Color>,
    passport_id: Option<String>,
    country_id: Option<u64>,
}

impl Passport {
    fn update(&mut self, data: &str) {
        let type_str = &data[0..3];
        let data_str = &data[4..];
        println!("parsing {}", data);
        println!("type is {}", type_str);
        println!("data is {}", data_str);
        match type_str {
            "byr" => self.birth_year = Some(data_str.parse().unwrap()),
            "ecl" => self.eye_color = Some(data_str.parse().unwrap()),
            "pid" => self.passport_id = Some(data_str.parse().unwrap()),
            "eyr" => self.expiration_year = Some(data_str.parse().unwrap()),
            "hcl" => self.hair_color = Some(data_str.parse().unwrap()),
            "iyr" => self.issue_year = Some(data_str.parse().unwrap()),
            "cid" => self.country_id = Some(data_str.parse().unwrap()),
            "hgt" => self.height = Some(data_str[0..data_str.len() - 2].parse().unwrap()),
            _ => panic!("encountered {} type", type_str),
        }
    }

    fn valid(&self) -> bool {
        [
            self.birth_year.is_some(),
            self.issue_year.is_some(),
            self.expiration_year.is_some(),
            self.height.is_some(),
            self.hair_color.is_some(),
            self.eye_color.is_some(),
            self.passport_id.is_some(),
        ]
        .iter()
        .all(|n| *n)
    }
}

fn main() {
    let passports = stdin_to_passports();
    let valid = passports.iter().filter(|p| p.valid()).count();
    println!("valid = {}", valid);
}

fn stdin_to_passports() -> Vec<Passport> {
    let stdin = io::stdin();
    let mut passport = Passport::default();
    let mut passports = Vec::new();
    for line in stdin
        .lock()
        .lines()
        .chain(std::iter::once(Ok("".to_string())))
    {
        let line = line.unwrap();
        if line.len() == 0 {
            passports.push(passport);
            passport = Passport::default();
            continue;
        }

        for segment in line.split_whitespace() {
            passport.update(segment);
        }
    }

    passports
}
