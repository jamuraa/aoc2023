#![feature(ascii_char)]

use anyhow::{format_err, Error};

use thiserror::Error;

use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");

    println!("Part 1 result: {}", part1(input)?);

    println!("Part 2 result: {}", part2(input)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize, Error> {
    let games: Vec<_> = input.lines().map(parse_game).collect();
    dbg!(&games);
    let ok_games_sum = games
        .into_iter()
        .filter_map(Result::ok)
        .map(|g| g.number)
        .sum();
    Ok(ok_games_sum)
}

fn part2(input: &str) -> Result<usize, Error> {
    let games: Vec<Game> = input
        .lines()
        .map(parse_game_unbounded)
        .collect::<Result<Vec<_>, GameError>>()?;
    let ok_games_sum = games.into_iter().map(|g| g.min_set().power()).sum();
    Ok(ok_games_sum)
}

#[derive(Debug)]
struct Game {
    number: usize,
    rounds: Vec<GameRound>,
}

impl Game {
    fn min_set(&self) -> GameRound {
        GameRound {
            red: self.rounds.iter().map(|g| g.red).max().unwrap_or(0),
            blue: self.rounds.iter().map(|g| g.blue).max().unwrap_or(0),
            green: self.rounds.iter().map(|g| g.green).max().unwrap_or(0),
        }
    }
}

#[derive(Debug, PartialEq)]
struct GameRound {
    red: usize,
    blue: usize,
    green: usize,
}

impl GameRound {
    fn power(&self) -> usize {
        self.red * self.blue * self.green
    }

    fn from_str_unbounded(s: &str) -> Result<Self, GameError> {
        let mut green = None;
        let mut red = None;
        let mut blue = None;
        for ballstr in s.split(", ") {
            if ballstr.ends_with("green") {
                let num = ballstr[..ballstr.len() - 6].parse()?;
                let None = green.replace(num) else {
                    return Err(GameError::ColorRepeated("green"));
                };
            } else if ballstr.ends_with("red") {
                let num = ballstr[..ballstr.len() - 4].parse()?;
                let None = red.replace(num) else {
                    return Err(GameError::ColorRepeated("redn"));
                };
            } else if ballstr.ends_with("blue") {
                let num = ballstr[..ballstr.len() - 5].parse()?;
                let None = blue.replace(num) else {
                    return Err(GameError::ColorRepeated("blue"));
                };
            } else {
                return Err(GameError::UnrecognizedColor(String::from(ballstr)));
            }
        }
        Ok(GameRound {
            green: green.unwrap_or(0),
            red: red.unwrap_or(0),
            blue: blue.unwrap_or(0),
        })
    }
}

impl std::str::FromStr for GameRound {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut green = None;
        let mut red = None;
        let mut blue = None;
        for ballstr in s.split(", ") {
            if ballstr.ends_with("green") {
                let num = ballstr[..ballstr.len() - 6].parse()?;
                if num > 13 {
                    return Err(GameError::TooManyBalls(String::from("green"), num));
                }
                let None = green.replace(num) else {
                    return Err(GameError::ColorRepeated("green"));
                };
            } else if ballstr.ends_with("red") {
                let num = ballstr[..ballstr.len() - 4].parse()?;
                if num > 12 {
                    return Err(GameError::TooManyBalls(String::from("red"), num));
                }
                let None = red.replace(num) else {
                    return Err(GameError::ColorRepeated("redn"));
                };
            } else if ballstr.ends_with("blue") {
                let num = ballstr[..ballstr.len() - 5].parse()?;
                if num > 14 {
                    return Err(GameError::TooManyBalls(String::from("blue"), num));
                }
                let None = blue.replace(num) else {
                    return Err(GameError::ColorRepeated("blue"));
                };
            } else {
                return Err(GameError::UnrecognizedColor(String::from(ballstr)));
            }
        }
        Ok(GameRound {
            green: green.unwrap_or(0),
            red: red.unwrap_or(0),
            blue: blue.unwrap_or(0),
        })
    }
}

#[derive(Error, Debug)]
enum GameError {
    #[error("There are too many balls in one of the rounds")]
    TooManyBalls(String, usize),
    #[error("More than one collection with the {0} color in a game")]
    ColorRepeated(&'static str),
    #[error("Color unrecognized: {0}")]
    UnrecognizedColor(String),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
    #[error("Parsing Integer Error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

fn parse_game(line: &str) -> Result<Game, GameError> {
    let game_split_at = line
        .find(": ")
        .ok_or(format_err!("Couldn't find game colon"))?;
    let (game_num, rounds) = line.split_at(game_split_at);
    let number = game_num.split(" ").last().unwrap().parse()?;
    let rounds = rounds[2..]
        .split("; ")
        .map(GameRound::from_str)
        .collect::<Result<Vec<_>, GameError>>()?;
    Ok(Game { number, rounds })
}

fn parse_game_unbounded(line: &str) -> Result<Game, GameError> {
    let game_split_at = line
        .find(": ")
        .ok_or(format_err!("Couldn't find game colon"))?;
    let (game_num, rounds) = line.split_at(game_split_at);
    let number = game_num.split(" ").last().unwrap().parse()?;
    let rounds = rounds[2..]
        .split("; ")
        .map(GameRound::from_str_unbounded)
        .collect::<Result<Vec<_>, GameError>>()?;
    Ok(Game { number, rounds })
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    const EXAMPLE_GAMES: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn from_problem() {
        assert_eq!(part1(EXAMPLE_GAMES).unwrap(), 8);
    }

    #[test]
    fn from_problem_p2() {
        assert_eq!(part2(EXAMPLE_GAMES).unwrap(), 2286);
    }
}
