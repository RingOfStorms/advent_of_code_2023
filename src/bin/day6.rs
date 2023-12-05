use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
use std::{str::FromStr, time::Instant};

static DAY: u8 = 6;

#[derive(Debug, Builder, Clone)]
struct Todo {}

impl FromStr for Todo {
    type Err = BoxE;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut builder = TodoBuilder::default();
        Ok(builder.build()?)
    }
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let todo: Todo = input.parse()?;
    let parsed_time = start.elapsed();

    // algo
    let start = Instant::now();
    let answer = 0;
    let algo_time = start.elapsed();

    // output
    println!("part 1: {answer}");
    println!("\tparse: {parsed_time:?}");
    println!("\talgo: {algo_time:?}");
    Ok(answer)
}

fn part2(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let todo: Todo = input.parse()?;
    let parsed_time = start.elapsed();

    // algo
    let start = Instant::now();
    let answer = 0;
    let algo_time = start.elapsed();

    // output
    println!("part 2: {answer}");
    println!("\tparse: {parsed_time:?}");
    println!("\talgo: {algo_time:?}");
    Ok(answer)
}

#[tokio::main]
async fn main() -> Result<()> {
    let input = utils::aoc::get_puzzle_input(DAY).await?;
    println!("Day {DAY}");
    println!("=====");
    part1(input.clone())?;
    part2(input.clone())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &'static str = "TODO_REPLACE";

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part1(DATA.to_owned())?, 0);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part2(DATA.to_owned())?, 0);
        Ok(())
    }
}