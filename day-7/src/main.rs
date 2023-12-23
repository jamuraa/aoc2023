use std::{cmp::Ordering, collections::HashMap, num::ParseIntError, str::FromStr};

use anyhow::Error;

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");

    let parsed_input = input.parse()?;

    println!("Part 1 result: {:?}", part1(&parsed_input));

    println!("Part 2 result: {:?}", part2(&parsed_input));

    Ok(())
}

#[allow(unused)]
fn numbers_ws_delimited(s: &str) -> Result<Vec<usize>, ParseIntError> {
    s.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<usize>, ParseIntError>>()
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn joker_cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == &Self::Jack && other != &Self::Jack {
            return Ordering::Less;
        }
        if other == &Self::Jack && self != &Self::Jack {
            return Ordering::Greater;
        }
        self.cmp(other)
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("nope"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BiddedHand {
    cards: Vec<Card>,
    kind: HandKind,
    bid: usize,
}

impl BiddedHand {
    fn jokerize(&mut self) {
        let mut cards = self.cards.clone();
        cards.as_mut_slice().sort();
        let mut counts = cards.iter().fold(HashMap::new(), |mut m, card| {
            *m.entry(*card).or_insert(0usize) += 1;
            m
        });
        let Some(jokers) = counts.remove(&Card::Jack) else {
            return;
        };
        if jokers == 5 {
            return;
        }
        let top_key = *counts.iter().max_by_key(|(_k, v)| **v).unwrap().0;
        *counts.get_mut(&top_key).unwrap() += jokers;
        let mut values: Vec<usize> = counts.values().copied().collect();
        values.sort();
        values.reverse();
        self.kind = match values[0] {
            5 => HandKind::FiveKind,
            4 => HandKind::FourKind,
            3 => match values[1] {
                2 => HandKind::FullHouse,
                _ => HandKind::ThreeKind,
            },
            2 => match values[1] {
                2 => HandKind::TwoPair,
                _ => HandKind::OnePair,
            },
            1 => HandKind::HighCard,
            _ => panic!("Not possible"),
        }
    }

    fn joker_ord(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => {
                for (mine, theirs) in self.cards.iter().zip(other.cards.iter()) {
                    match mine.joker_cmp(theirs) {
                        Ordering::Equal => continue,
                        v => return v,
                    }
                }
                Ordering::Equal
            }
            v => return v,
        }
    }
}

impl From<&Vec<Card>> for HandKind {
    fn from(value: &Vec<Card>) -> Self {
        assert_eq!(value.len(), 5);
        let mut cards = value.clone();
        cards.as_mut_slice().sort();
        let counts = cards.iter().fold(HashMap::new(), |mut m, card| {
            *m.entry(card).or_insert(0usize) += 1;
            m
        });
        let mut values: Vec<usize> = counts.values().copied().collect();
        values.sort();
        values.reverse();
        match values[0] {
            5 => Self::FiveKind,
            4 => Self::FourKind,
            3 => match values[1] {
                2 => Self::FullHouse,
                _ => Self::ThreeKind,
            },
            2 => match values[1] {
                2 => Self::TwoPair,
                _ => Self::OnePair,
            },
            1 => Self::HighCard,
            _ => panic!("Not possible"),
        }
    }
}

impl PartialOrd for BiddedHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BiddedHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            x => x,
        }
    }
}

impl FromStr for BiddedHand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();
        let bid = bid.parse()?;

        let cards: Vec<Card> = hand.chars().map(Card::from).collect();
        assert_eq!(5, cards.len());
        let kind = HandKind::from(&cards);
        Ok(Self { cards, kind, bid })
    }
}

struct Hands {
    hands: Vec<BiddedHand>,
}

impl FromStr for Hands {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<BiddedHand>, Error>>()?;
        Ok(Hands { hands })
    }
}

type ParsedInput = Hands;

fn part1(input: &ParsedInput) -> Result<usize, Error> {
    let mut ranked = input.hands.clone();
    ranked.sort();
    let mut winnings = 0;
    for (i, hand) in ranked.iter().enumerate() {
        winnings += (i + 1) * hand.bid
    }
    Ok(winnings)
}

fn part2(input: &ParsedInput) -> Result<usize, Error> {
    let mut ranked = input.hands.clone();
    for hand in ranked.iter_mut() {
        hand.jokerize();
    }
    ranked.sort_by(BiddedHand::joker_ord);
    let mut winnings = 0;
    for (i, hand) in ranked.iter().enumerate() {
        winnings += (i + 1) * hand.bid
    }
    println!("{ranked:#?}");
    Ok(winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn parse_input() {
        let parsed: ParsedInput = EX_INPUT.parse().expect("parses ok");
        assert_eq!(5, parsed.hands.len());
    }

    #[test]
    fn ex_part1() {
        assert_eq!(6440, part1(&(EX_INPUT.parse().unwrap())).unwrap());
    }

    #[test]
    fn ex_part2() {
        assert_eq!(5905, part2(&(EX_INPUT.parse().unwrap())).unwrap());
    }
}
