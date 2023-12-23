use std::{collections::HashMap, num::ParseIntError, str::FromStr, time::Instant};

use anyhow::{format_err, Error};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, line_ending},
    sequence::{pair, terminated, tuple},
    Finish,
};

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");

    let parsed_input = input.parse()?;

    let start = Instant::now();
    let res = part1(&parsed_input);
    let end = Instant::now();
    println!(
        "Part 1 result: {:?} in {:?}",
        res,
        end.duration_since(start)
    );

    let start = Instant::now();
    let res = part2(&parsed_input);
    let end = Instant::now();
    println!(
        "Part 2 result: {:?} took {:?}",
        res,
        end.duration_since(start)
    );

    Ok(())
}

#[allow(unused)]
fn numbers_ws_delimited<T: FromStr<Err = ParseIntError>>(s: &str) -> Result<Vec<T>, ParseIntError> {
    s.split_whitespace()
        .map(str::parse::<T>)
        .collect::<Result<Vec<T>, ParseIntError>>()
}

type ParsedInput = Pyramids;

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

struct Pyramids {
    pyrs: Vec<Pyramid>,
}

impl FromStr for Pyramids {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pyrs: s
                .lines()
                .map(str::parse)
                .collect::<Result<Vec<Pyramid>, Error>>()?,
        })
    }
}

struct Pyramid {
    numbers: Vec<i64>,
}

impl Pyramid {
    fn predict(&self) -> (i64, i64) {
        let mut layers = Vec::new();
        layers.push(self.numbers.clone());
        let mut layer = layers.last().unwrap();
        loop {
            let next_layer: Vec<i64> = layer
                .iter()
                .zip(layer.iter().skip(1))
                .map(|(first, second)| second - first)
                .collect();
            if next_layer.iter().all(|v| v == &0) {
                break;
            }
            layers.push(next_layer.clone());
            layer = layers.last().unwrap();
        }
        // Fill from the bottom
        layers.reverse();
        for i in 0..layers.len() - 1 {
            let next_d = *layers.get(i).unwrap().last().unwrap();
            let first_d = *layers.get(i).unwrap().first().unwrap();
            let next_layer = layers.get_mut(i + 1).unwrap();
            let last_of_next = *next_layer.last().unwrap();
            let first_of_next = *next_layer.first().unwrap();
            next_layer.insert(0, first_of_next - first_d);
            next_layer.push(last_of_next + next_d);
        }
        // prediction is the last num of the last layer
        (
            *layers.last().unwrap().first().unwrap(),
            *layers.last().unwrap().last().unwrap(),
        )
    }
}

impl FromStr for Pyramid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: numbers_ws_delimited(s)?,
        })
    }
}

fn part1(input: &ParsedInput) -> Result<i64, Error> {
    let predictions_sum = input.pyrs.iter().fold(0, |acc, p| acc + p.predict().1);
    Ok(predictions_sum)
}

fn part2(input: &ParsedInput) -> Result<i64, Error> {
    let predictions_sum = input.pyrs.iter().fold(0, |acc, p| acc + p.predict().0);
    Ok(predictions_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn parse_input() {
        let parsed: ParsedInput = EX_INPUT.parse().expect("parses ok");
        assert_eq!(3, parsed.pyrs.len());
        assert_eq!(6, parsed.pyrs[0].numbers.len());
    }

    #[test]
    fn ex_part1() {
        assert_eq!(114, part1(&(EX_INPUT.parse().unwrap())).unwrap());
    }

    #[test]
    fn ex_part2() {
        assert_eq!(2, part2(&(EX_INPUT.parse().unwrap())).unwrap());
    }
}
