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
fn numbers_ws_delimited(s: &str) -> Result<Vec<usize>, ParseIntError> {
    s.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<usize>, ParseIntError>>()
}

struct Node {
    this: usize,
    start_pos: bool,
    end_pos: bool,
    left: usize,
    right: usize,
}

impl Node {
    fn follow(&self, inst: char) -> usize {
        match inst {
            'L' => self.left,
            'R' => self.right,
            _ => panic!("bad instruction"),
        }
    }
}

fn codepoint_above_a(c: char) -> usize {
    (u32::from(c) - u32::from('A')) as usize
}

fn strnode_to_usize(s: &str) -> usize {
    assert_eq!(3, s.len());
    let mut chars = s.chars();
    26usize.pow(3) * codepoint_above_a(chars.next().unwrap())
        + 26usize.pow(2) * codepoint_above_a(chars.next().unwrap())
        + 26usize * codepoint_above_a(chars.next().unwrap())
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_i, (this, _, left, _, right, _)) = match tuple((
            alphanumeric1::<&str, nom::error::Error<&str>>,
            tag(" = ("),
            alphanumeric1,
            tag(", "),
            alphanumeric1,
            tag(")"),
        ))(s)
        .finish()
        {
            Ok(c) => c,
            Err(e) => return Err(format_err!("Failed parsing node: {e}")),
        };

        Ok(Self {
            end_pos: this.ends_with("Z"),
            start_pos: this.ends_with("A"),
            this: strnode_to_usize(this),
            left: strnode_to_usize(left),
            right: strnode_to_usize(right),
        })
    }
}

struct Graph {
    nodes: HashMap<usize, Node>,
}

impl FromStr for Graph {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes = s
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Node>, Error>>()?;
        Ok(Graph {
            nodes: nodes.into_iter().map(|n| (n.this, n)).collect(),
        })
    }
}

impl Graph {
    fn find_period_end(&self, start: usize, inst: impl Iterator<Item = char> + Clone) -> usize {
        let inst_count = inst.clone().count();
        let mut pos = start;
        let mut times_through = 0;
        loop {
            let inst_copy = inst.clone();
            for inst in inst_copy {
                pos = self.nodes.get(&pos).unwrap().follow(inst);
            }
            times_through += 1;
            if times_through % 100000 == 0 {
                println!("times through from {start}: {times_through}");
            }
            if self.nodes.get(&pos).unwrap().end_pos {
                println!("going through the inst {times_through} times gets to an end node");
                return times_through * inst_count;
            }
        }
    }
}

struct GraphWithInstructions {
    instructions: String,
    graph: Graph,
}

impl GraphWithInstructions {
    fn follow_instructions(&self) -> usize {
        let mut loc = strnode_to_usize("AAA");
        let endnode = strnode_to_usize("ZZZ");
        let mut steps = 0;
        let instructions = self.instructions.chars().cycle();
        for inst in instructions {
            loc = self.one_inst_from(loc, inst).0;
            steps += 1;
            if loc == endnode {
                return steps;
            }
        }
        return 0;
    }

    fn one_inst_from(&self, node: usize, inst: char) -> (usize, bool) {
        let next_label = self.graph.nodes.get(&node).unwrap().follow(inst);
        let end_pos = self.graph.nodes.get(&next_label).unwrap().end_pos;
        (next_label, end_pos)
    }

    fn start_nodes(&self) -> Vec<usize> {
        self.graph
            .nodes
            .iter()
            .filter_map(|(k, v)| v.start_pos.then_some(*k))
            .collect()
    }
}

impl FromStr for GraphWithInstructions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (i, instructions) = match terminated(
            alpha1::<&str, nom::error::Error<&str>>,
            pair(line_ending, line_ending),
        )(s)
        .finish()
        {
            Ok(x) => x,
            Err(e) => return Err(format_err!("Failed parsing {e}")),
        };
        let graph = i.parse()?;
        Ok(Self {
            instructions: instructions.to_string(),
            graph,
        })
    }
}

type ParsedInput = GraphWithInstructions;

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

fn part1(input: &ParsedInput) -> Result<usize, Error> {
    Ok(input.follow_instructions())
}

fn part2(input: &ParsedInput) -> Result<usize, Error> {
    let positions = input.start_nodes();

    println!("Ghosts at {} nodes", positions.len());

    let instructions = input.instructions.chars();
    let periods: Vec<usize> = positions
        .iter()
        .map(|p| input.graph.find_period_end(*p, instructions.clone()))
        .collect();

    let lcm = lcm(periods.as_slice());
    Ok(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_INPUT: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const EX_INPUT2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    const EX_INPUT3: &str = r#"LR

GGA = (GGB, XXX)
GGB = (XXX, GGZ)
GGZ = (GGB, XXX)
HHA = (HHB, XXX)
HHB = (HHC, HHC)
HHC = (HHZ, HHZ)
HHZ = (HHB, HHB)
XXX = (XXX, XXX)"#;

    #[test]
    fn parse_input() {
        let parsed: ParsedInput = EX_INPUT.parse().expect("parses ok");
        assert_eq!(7, parsed.graph.nodes.len());
        assert_eq!(2, parsed.instructions.len());
    }

    #[test]
    fn ex_part1() {
        assert_eq!(2, part1(&(EX_INPUT.parse().unwrap())).unwrap());
        assert_eq!(6, part1(&(EX_INPUT2.parse().unwrap())).unwrap());
    }

    #[test]
    fn ex_part2() {
        assert_eq!(6, part2(&(EX_INPUT3.parse().unwrap())).unwrap());
    }
}
