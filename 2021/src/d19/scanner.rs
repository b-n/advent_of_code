use std::str::FromStr;
use super::beacon::Beacon;

#[derive(Debug)]
pub struct Scanner {
    name: String,
    pings: Vec<Beacon>,
}

impl FromStr for Scanner {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut parts = input.split("\n")
            .filter(|x| !x.is_empty());

        let name = String::from(parts.next().unwrap());

        let pings = parts
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<Beacon>().unwrap())
            .collect::<Vec<Beacon>>();

        Ok(Self {
            name, 
            pings,
        })
    }
}

