use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
use std::{str::FromStr, time::Instant};

static DAY: u8 = TODO;

#[derive(Debug, Clone)]
struct Todo {}

impl Todo {
    fn new(s: &str) -> Result<Self> {
        Ok(Todo {})
    }
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let todo = Todo::new(&input)?;
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let answer = 0;
    let algo_time = a_start.elapsed();

    // output
    println!("part 1: {answer}\t[total: {:?}]", start.elapsed());
    println!("\tparse: {parsed_time:?}");
    println!("\talgo: {algo_time:?}");
    println!("Input\n====\n{input}");
    Ok(answer)
}

fn part2(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let todo = Todo::new(&input)?;
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let answer = 0;
    let algo_time = a_start.elapsed();

    // output
    println!("part 2: {answer}\t[total: {:?}]", start.elapsed());
    println!("\tparse: {parsed_time:?}");
    println!("\talgo: {algo_time:?}");
    Ok(answer)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Day {DAY}");
    println!("=====");
    let input = utils::aoc::get_puzzle_input(DAY).await?;
    part1(input.clone())?;

    // part2(input.clone())?;
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
