use aoc23::prelude::*;
use std::time::Instant;

fn part1(input: String) -> Result<usize> {
    let mut answer = 0;
    Ok(answer)
}

fn part2(input: String) -> Result<usize> {
    let mut answer = 0;
    Ok(answer)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Day 5");
    println!("=====");

    // let input = utils::aoc::get_puzzle_input(5).await?;
    let input = "".to_owned();
    let start = Instant::now();
    println!("part 1: {}\t[{:?}]", part1(input.clone())?, start.elapsed());
    let start = Instant::now();
    println!("part 2: {}\t[{:?}]", part2(input.clone())?, start.elapsed());
    Ok(())
}
