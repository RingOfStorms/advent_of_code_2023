use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
use std::{arch::x86_64::_MM_FROUND_CUR_DIRECTION, str::FromStr, time::Instant};

static DAY: u8 = 9;

fn part1(input: String) -> Result<i64> {
    let start = Instant::now();
    let answer: i64 = input
        .lines()
        .par_bridge()
        .map(|line| {
            let mut sequences: Vec<Vec<i64>> = vec![];
            line.trim()
                .split_whitespace()
                .map(|s| s.parse::<i64>().expect("Failed to parse number in input"))
                .enumerate()
                .for_each(|(idx, num)| {
                    // every new number of primary history can result in a new row of depth
                    sequences.push(vec![]);
                    for depth in 0..sequences.len() {
                        if depth == 0 {
                            sequences
                                .get_mut(depth)
                                .expect("top history expected")
                                .push(num);
                        } else {
                            let len = sequences[depth].len();
                            let above = &sequences[depth - 1];
                            let left = *above.get(len).expect("expected value left");
                            let right = *above.get(len + 1).expect("expected value right");
                            sequences
                                .get_mut(depth)
                                .expect("seq current depth expect")
                                .push(right - left);
                        }
                    }
                });
            sequences
                .iter()
                .rev()
                .skip_while(|seq| seq.iter().all(|n| n == &0))
                .fold(0, |acc, seq| seq.last().expect("expected last value") + acc)
        })
        .sum();

    // output
    println!("part 1: {answer}\t[total: {:?}]", start.elapsed());
    Ok(answer)
}

// algo
fn part2(input: String) -> Result<i64> {
    let start = Instant::now();
    let answer: i64 = input
        .lines()
        .par_bridge()
        .map(|line| {
            let mut sequences: Vec<Vec<i64>> = vec![];
            line.trim()
                .split_whitespace()
                .map(|s| s.parse::<i64>().expect("Failed to parse number in input"))
                .enumerate()
                .for_each(|(idx, num)| {
                    // every new number of primary history can result in a new row of depth
                    sequences.push(vec![]);
                    for depth in 0..sequences.len() {
                        if depth == 0 {
                            sequences
                                .get_mut(depth)
                                .expect("top history expected")
                                .push(num);
                        } else {
                            let len = sequences[depth].len();
                            let above = &sequences[depth - 1];
                            let left = *above.get(len).expect("expected value left");
                            let right = *above.get(len + 1).expect("expected value right");
                            sequences
                                .get_mut(depth)
                                .expect("seq current depth expect")
                                .push(right - left);
                        }
                    }
                });
            sequences
                .iter()
                .rev()
                .skip_while(|seq| seq.iter().all(|n| n == &0))
                .fold(0, |acc, seq| {
                    seq.first().expect("expected last value") - acc
                })
        })
        .sum();

    // output
    println!("part 2: {answer}\t[total: {:?}]", start.elapsed());
    Ok(answer)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Day {DAY}");
    println!("=====");
    let input = utils::aoc::get_puzzle_input(DAY).await?;
    part1(input.clone())?;
    part2(input.clone())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part1(DATA.to_owned())?, 114);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part2(DATA.to_owned())?, 2);
        Ok(())
    }
}
