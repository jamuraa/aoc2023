#![feature(ascii_char)]

use anyhow::{format_err, Error};

use std::ascii::Char as AsciiChar;

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");

    println!("Part 1 result: {}", part1(input)?);

    println!("Part 2 result: {}", part2(input)?);

    Ok(())
}

fn char_to_number(c: &AsciiChar) -> Option<u8> {
    let v = *c as u8;

    if v >= 48 && v <= 57 {
        Some(v - 48)
    } else {
        None
    }
}

fn part1(input: &str) -> Result<u64, Error> {
    let mut v = Vec::new();
    for line in input.lines() {
        let ascii = line.as_ascii().ok_or(format_err!("input is not ascii"))?;

        // find the first number from the front
        let first = ascii
            .iter()
            .find_map(char_to_number)
            .ok_or(format_err!("doesn't contain a number"))?;

        // We know that there's at least one number
        let last = ascii.iter().rev().find_map(char_to_number).unwrap();

        let num = first as u64 * 10 + last as u64;
        v.push(num);
    }

    Ok(v.iter().sum())
}

fn find_num_or_written(line: &[AsciiChar], start_idx: usize) -> Option<u8> {
    if let Some(num) = char_to_number(&line[start_idx]) {
        return Some(num);
    }

    let s = line[start_idx..].as_str();
    if s.starts_with("one") {
        return Some(1);
    } else if s.starts_with("two") {
        return Some(2);
    } else if s.starts_with("three") {
        return Some(3);
    } else if s.starts_with("four") {
        return Some(4);
    } else if s.starts_with("five") {
        return Some(5);
    } else if s.starts_with("six") {
        return Some(6);
    } else if s.starts_with("seven") {
        return Some(7);
    } else if s.starts_with("eight") {
        return Some(8);
    } else if s.starts_with("nine") {
        return Some(9);
    }
    None
}

fn part2(input: &str) -> Result<u64, Error> {
    let mut v = Vec::new();
    for line in input.lines() {
        let ascii = line.as_ascii().ok_or(format_err!("input is not ascii"))?;

        let first = (0..ascii.len())
            .find_map(|i| find_num_or_written(ascii, i))
            .ok_or(format_err!("couldn't find a number (or written)"))?;

        let last = (0..ascii.len())
            .rev()
            .find_map(|i| find_num_or_written(ascii, i))
            .unwrap();

        let num = first as u64 * 10 + last as u64;
        v.push(num);
    }

    Ok(v.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn from_problem() {
        let s = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(part1(s).unwrap(), 142);
    }

    #[test]
    fn from_problem_p2() {
        let s = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(part2(s).unwrap(), 281);
    }
}
