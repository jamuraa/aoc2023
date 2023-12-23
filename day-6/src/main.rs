use std::{num::ParseIntError, str::FromStr};

use anyhow::{format_err, Error};

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");

    let parsed_input = input.parse()?;

    println!("Part 1 result: {:?}", part1(&parsed_input));

    let reparsed_input = keming_input(&parsed_input);

    println!("Part 2 result: {:?}", part2(&reparsed_input));

    Ok(())
}

fn keming_input(input: &RaceRecords) -> RaceRecords {
    let concated_time = input.races.iter().fold(String::new(), |mut acc, x| {
        acc.push_str(format!("{}", x.time).as_str());
        acc
    });
    let concated_dist = input.races.iter().fold(String::new(), |mut acc, x| {
        acc.push_str(format!("{}", x.distance).as_str());
        acc
    });
    RaceRecords {
        races: vec![Race {
            time: concated_time.parse().unwrap(),
            distance: concated_dist.parse().unwrap(),
        }],
    }
}

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn winning_ways(&self) -> usize {
        (1..self.time)
            .filter(|held| (self.time - held) * held > self.distance)
            .count()
    }
}

struct RaceRecords {
    races: Vec<Race>,
}

fn numbers_ws_delimited(s: &str) -> Result<Vec<usize>, ParseIntError> {
    s.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<usize>, ParseIntError>>()
}

impl FromStr for RaceRecords {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (_, times) = lines
            .next()
            .unwrap()
            .split_once(':')
            .ok_or(format_err!("missing colon"))?;
        let times = numbers_ws_delimited(times)?;
        let (_, distances) = lines
            .next()
            .unwrap()
            .split_once(':')
            .ok_or(format_err!("no colon"))?;
        let distances = numbers_ws_delimited(distances)?;
        let mut records = Vec::new();
        for (time, distance) in times.into_iter().zip(distances.into_iter()) {
            records.push(Race { time, distance });
        }
        Ok(RaceRecords { races: records })
    }
}

type ParsedInput = RaceRecords;

fn part1(input: &ParsedInput) -> Result<usize, Error> {
    Ok(input
        .races
        .iter()
        .fold(1, |acc, race| acc * race.winning_ways()))
}

fn part2(input: &ParsedInput) -> Result<usize, Error> {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn parse_input() {
        let parsed: ParsedInput = EX_INPUT.parse().expect("parses ok");
        assert_eq!(3, parsed.races.len());
    }

    #[test]
    fn ex_part1() {
        assert_eq!(288, part1(&(EX_INPUT.parse().unwrap())).unwrap());
    }

    #[test]
    fn ex_part2() {
        assert_eq!(46, part2(&(EX_INPUT.parse().unwrap())).unwrap());
    }
}
