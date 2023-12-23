#![feature(ascii_char)]

use anyhow::Error;
use std::cmp::min;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");

    println!("Part 1 result: {}", part1(input)?);

    println!("Part 2 result: {}", part2(input)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize, Error> {
    let board = Board {
        lines: input.lines().map(String::from).collect(),
    };
    let nums = board.find_part_numbers();
    Ok(nums.into_iter().map(|p| p.number).sum())
}

fn part2(input: &str) -> Result<usize, Error> {
    let board = Board {
        lines: input.lines().map(String::from).collect(),
    };
    let gears = board.find_gear_ratios();
    Ok(gears.into_iter().sum())
}

struct Board {
    lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct PartNum {
    number: usize,
    part_type: char,
    part_loc: (usize, usize),
}

impl Board {
    fn find_part_numbers(&self) -> Vec<PartNum> {
        let mut parts = Vec::new();
        for (row, line) in self.lines.iter().enumerate() {
            for (start, end, num) in Board::find_nums_in_line(line) {
                if let Some((c, x, y)) = self.find_symbol_in_range(
                    row.saturating_sub(1),
                    start.saturating_sub(1),
                    min(row + 1, self.lines.len() - 1),
                    min(end + 2, self.lines[0].len() - 1),
                ) {
                    println!("Found {num} with part {c} at {x}, {y}");
                    parts.push(PartNum {
                        number: num,
                        part_type: c,
                        part_loc: (x, y),
                    });
                }
            }
        }
        parts
    }

    fn find_gear_ratios(&self) -> Vec<usize> {
        let numbers = self.find_part_numbers();
        let mut map = HashMap::new();
        for num in numbers {
            let v = map.entry(num.part_loc).or_insert(Vec::new());
            v.push(num);
        }
        let mut ratios = Vec::new();
        for (loc, numbers) in map {
            if numbers.len() == 2 {
                if numbers[0].part_type == '*' {
                    let one = numbers[0].number;
                    let two = numbers[1].number;
                    let ratio = one * two;
                    println!("Found gear at {loc:?} with {one} x {two} = {ratio}");
                    ratios.push(ratio);
                }
            }
        }
        ratios
    }

    fn find_symbol_in_range(
        &self,
        row_start: usize,
        col_start: usize,
        row_end: usize,
        col_end: usize,
    ) -> Option<(char, usize, usize)> {
        self.symbols_in_range(row_start, col_start, row_end, col_end)
            .find(|c| c.0 != '.' && !c.0.is_ascii_digit())
    }

    fn symbols_in_range(
        &self,
        row_start: usize,
        col_start: usize,
        row_end: usize,
        col_end: usize,
    ) -> impl Iterator<Item = (char, usize, usize)> + '_ {
        let mut iters = Vec::new();
        for row in row_start..row_end + 1 {
            let chars = self.lines[row]
                .chars()
                .skip(col_start)
                .take(col_end - col_start)
                .enumerate()
                .map(move |(col, c)| (c, row, col + col_start));
            iters.push(chars)
        }
        iters.into_iter().flatten()
    }

    fn find_nums_in_line(line: &String) -> Vec<(usize, usize, usize)> {
        let mut r = Vec::new();
        for span in SignalPartition::new(false, line.chars(), |c| char::is_ascii_digit(&c)) {
            if span.signal {
                r.push((
                    span.start,
                    span.end - 1,
                    line.as_str()[span.start..span.end]
                        .parse::<usize>()
                        .unwrap(),
                ));
            }
        }
        r
    }
}

struct SignalPartition<It: Iterator<Item = T>, F, T> {
    last_idx: usize,
    signal: bool,
    signal_fn: F,
    source: It,
    pulled: usize,
}

#[derive(PartialEq, Debug)]
struct SignalSpan {
    start: usize,
    end: usize,
    signal: bool,
}

impl<It: Iterator<Item = T>, F: FnMut(T) -> bool, T> SignalPartition<It, F, T> {
    fn new(initial: bool, source: It, signal_fn: F) -> Self {
        Self {
            last_idx: 0,
            pulled: 0,
            signal: initial,
            signal_fn,
            source,
        }
    }
}

impl<It: Iterator<Item = T>, F: FnMut(T) -> bool, T> Iterator for SignalPartition<It, F, T> {
    type Item = SignalSpan;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(t) = self.source.next() {
            self.pulled += 1;
            let signal_now = (self.signal_fn)(t);
            println!(
                "last_idx: {}, pulled: {}, signal: {}, signal_now: {}",
                self.last_idx, self.pulled, self.signal, signal_now
            );
            if signal_now != self.signal {
                let res = SignalSpan {
                    start: self.last_idx,
                    end: self.pulled - 1,
                    signal: self.signal,
                };
                self.signal = !self.signal;
                self.last_idx = self.pulled - 1;
                return Some(res);
            }
        }
        if self.pulled == self.last_idx {
            return None;
        }
        let r = SignalSpan {
            start: self.last_idx,
            end: self.pulled,
            signal: self.signal,
        };
        self.last_idx = self.pulled;
        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_partition() {
        let t = &[0, 0, 1, 0, 0, 0, 0, 1, 1, 1];
        let mut sp = SignalPartition::new(false, t.iter(), |x| *x == 1);

        assert_eq!(
            SignalSpan {
                start: 0,
                end: 2,
                signal: false
            },
            sp.next().unwrap()
        );
        assert_eq!(
            SignalSpan {
                start: 2,
                end: 3,
                signal: true
            },
            sp.next().unwrap()
        );
        assert_eq!(
            SignalSpan {
                start: 3,
                end: 7,
                signal: false
            },
            sp.next().unwrap()
        );
        assert_eq!(
            SignalSpan {
                start: 7,
                end: 10,
                signal: true
            },
            sp.next().unwrap()
        );
        assert_eq!(None, sp.next());
    }

    #[test]
    fn find_nums_in_line() {
        let nums = Board::find_nums_in_line(&String::from("145"));
        assert_eq!(nums.len(), 1);
        assert_eq!(nums[0], (0, 2, 145));

        let nums = Board::find_nums_in_line(&String::from("123...145"));
        assert_eq!(nums.len(), 2);
        assert_eq!(nums[0], (0, 2, 123));
        assert_eq!(nums[1], (6, 8, 145));
    }

    static SCHEM: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn example_1() {
        let board = Board {
            lines: SCHEM.lines().map(String::from).collect(),
        };

        let numbers = board.find_part_numbers();
        assert_eq!(8, numbers.len());

        assert_eq!(4361usize, numbers.iter().map(|n| n.number).sum());
    }

    #[test]
    fn example_2() {
        let board = Board {
            lines: SCHEM.lines().map(String::from).collect(),
        };

        let gears = board.find_gear_ratios();
        assert_eq!(2, gears.len());
        assert_eq!(467835usize, gears.into_iter().sum());
    }
}
