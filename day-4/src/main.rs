#![feature(ascii_char)]

use anyhow::Error;
use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    str::FromStr,
};

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");

    println!("Part 1 result: {:?}", part1(input));

    println!("Part 2 result: {:?}", part2(input));

    Ok(())
}

fn part1(input: &str) -> Result<usize, Error> {
    let cards: Result<Vec<Card>, Error> = input.lines().map(str::parse).collect();
    Ok(cards?.iter().fold(0, |acc, c| acc + c.worth()))
}

fn part2(input: &str) -> Result<usize, Error> {
    let cards: Result<Vec<Card>, Error> = input.lines().map(str::parse).collect();
    let cards = cards?;
    let mut final_cards: HashMap<usize, Card> = HashMap::new();
    let max_card_num = cards.iter().max_by_key(|c| c.number).unwrap().number;
    for card in cards {
        if let Some(e) = final_cards.insert(card.number, card) {
            return Err(anyhow::format_err!("Duplicate card {e:?}"));
        }
    }
    for i in 1..=max_card_num {
        let card = final_cards.get(&i).unwrap();
        let won_copies = final_cards.get(&i).unwrap().copies;
        let worth = final_cards.get(&i).unwrap().matches();
        if worth == 0 {
            continue;
        }
        println!(
            "{won_copies} instances of card {} get us the next {worth} cards",
            card.number
        );
        for won_card in i + 1..=std::cmp::min(max_card_num, i + worth) {
            // Duplicate the card won_copies_times
            final_cards.get_mut(&won_card).unwrap().copies += won_copies;
        }
    }
    println!("{final_cards:#?}");
    Ok(final_cards.into_iter().fold(0, |acc, c| acc + c.1.copies))
}

#[derive(Debug)]
struct Card {
    number: usize,
    copies: usize,
    winners: HashSet<usize>,
    have: Vec<usize>,
}

impl Card {
    fn worth(&self) -> usize {
        let count = self
            .have
            .iter()
            .filter(|num| self.winners.contains(num))
            .count();
        if count == 0 {
            0
        } else {
            2usize.pow(count.saturating_sub(1) as u32)
        }
    }

    fn matches(&self) -> usize {
        self.have
            .iter()
            .filter(|num| self.winners.contains(num))
            .count()
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((card_num, numbers)) = s.split_once(':') else {
            return Err(anyhow::format_err!("Couldn't find card number"));
        };
        let number = card_num
            .split_whitespace()
            .skip(1)
            .next()
            .ok_or(anyhow::format_err!("Couldn't find card number"))?
            .parse()?;
        let Some((winners, have)) = numbers.split_once('|') else {
            return Err(anyhow::format_err!("Couldn't find winners delimiter"));
        };
        let winners: Result<HashSet<usize>, ParseIntError> = winners
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect();
        let have: Result<Vec<usize>, ParseIntError> = have
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect();
        Ok(Self {
            number,
            copies: 1,
            winners: winners?,
            have: have?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_parse() {
        let c: Result<Card, Error> = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse();

        let c = c.expect("card shouuld parse ok");
        assert_eq!(5, c.winners.len());
        assert_eq!(8, c.have.len());
        assert_eq!(8, c.worth());

        let c: Result<Card, Error> = "Card 1 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse();
        assert!(c.is_err());

        let c: Result<Card, Error> = "Card 1: 41 48 83 86 17 83 86  6 31 17  9 48 53".parse();
        assert!(c.is_err());
    }

    const EX_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_part1() {
        let res = part1(EX_INPUT);
        assert_eq!(13, res.unwrap());
    }

    #[test]
    fn ex_part2() {
        let res = part2(EX_INPUT);
        assert_eq!(30, res.unwrap());
    }
}
