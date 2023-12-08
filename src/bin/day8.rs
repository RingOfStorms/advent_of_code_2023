use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashMap, time::Instant};

extern crate regex;

use regex::Regex;
use static_init::dynamic;

#[dynamic]
static RE_PARSE_NODE: Regex =
    Regex::new(r"(?<id>\w{3}).*?(?<left>\w{3}).*?(?<right>\w{3})").expect("re_parse_node invalid");

#[derive(Debug, Builder, Clone)]
struct Node<'a> {
    id: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    fn new(s: &'a str) -> Result<Self> {
        // let re = Regex::new(r"(?`<id>`\w{3}) = ((?`<left>`\w{3}), (?`<right>`\w{3}))")?
        // let re = Regex::new(r"(?`<id>`\w{3})._?(?`<left>`\w{3})._?(?`<right>`\w{3})")?
        let re = RE_PARSE_NODE.captures(s).expect("No match for regex");
        Ok(NodeBuilder::default()
            .id(re.name("id").expect("no id").as_str())
            .left(re.name("left").expect("no left").as_str())
            .right(re.name("right").expect("no right").as_str())
            .build()?)
    }
}

#[derive(Debug, Clone)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn from_char(c: &char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Not valid dir"),
        }
    }
}

#[derive(Debug, Builder, Clone)]
struct Map<'a> {
    dirs: Vec<Dir>,
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Map<'a> {
    fn new(s: &'a str) -> Result<Self> {
        let mut splits = s.trim().split("\n\n");
        Ok(MapBuilder::default()
            .dirs(
                splits
                    .next()
                    .unwrap()
                    .trim()
                    .chars()
                    .map(|c| Dir::from_char(&c))
                    .collect(),
            )
            .nodes(
                splits
                    .next()
                    .unwrap()
                    .trim()
                    .lines()
                    .map(|a| Node::new(a.trim()).map(|n| (n.id, n)))
                    .try_collect()?,
            )
            .build()?)
    }
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let map: Map = Map::new(&input)?;
    let parsed_time = start.elapsed();

    // algo
    let start = Instant::now();
    let mut answer = 0;
    let mut current = map.nodes.get("AAA").expect("no start node");
    let mut dir = map.dirs.iter().cycle();
    while !current.id.eq("ZZZ") {
        match dir.next().unwrap() {
            Dir::Left => current = map.nodes.get(current.left).expect("no left node"),
            Dir::Right => current = map.nodes.get(current.right).expect("no right node"),
        }
        answer += 1;
    }
    let algo_time = start.elapsed();

    // output
    println!("Day 5, part 1: {answer}");
    println!("\tparse: {parsed_time:?}");
    println!("\talgo: {algo_time:?}");
    Ok(answer)
}

fn part2(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let map: Map = Map::new(&input)?;
    let parsed_time = start.elapsed();

    // algo
    let start = Instant::now();
    let answer = 0;
    let algo_time = start.elapsed();

    // output
    println!("Day 5, part 2: {answer}");
    println!("\tparse: {parsed_time:?}");
    println!("\talgo: {algo_time:?}");
    Ok(answer)
}

// TODO come back and revise for a faster solution
#[tokio::main]
async fn main() -> Result<()> {
    println!("Day {DAY}");
    println!("=====");
    let input = utils::aoc::get_puzzle_input(DAY).await?;
    part1(input.clone())?;

    // part2(input.clone())?;
    Ok(())
}

static DAY: u8 = 8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_b() -> Result<()> {
        assert_eq!(
            part1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
                    .to_owned(),
            )?,
            2
        );
        assert_eq!(
            part1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
                    .to_owned()
            )?,
            6
        );
        Ok(())
    }
    // #[test] fn test_part_2() -> Result<()> {
    // assert_eq!(part2("REPLACE".to_owned())?, 0); Ok(()) }
}
