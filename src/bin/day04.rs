use aoc23::prelude::*;
use itertools::Itertools;
use std::{collections::VecDeque, str::FromStr, time::Instant};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Card {
    id: usize,
    part_1_score: usize,
    part_2_matches: usize,
    part_2_count: usize,
}

impl FromStr for Card {
    type Err = BoxE;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut id: Option<usize> = None;
        let mut winning_numbers: Vec<usize> = vec![];
        let mut score = 0;
        let mut matches = 0;
        for part in s.split(':') {
            if id.is_some() {
                for (num_mode, wins_or_nums) in part.split('|').enumerate() {
                    wins_or_nums
                        .trim()
                        .split_whitespace()
                        .into_iter()
                        .map(|num| {
                            num.parse::<usize>()
                                .expect(&format!("could not parse number: {}", num))
                        })
                        .for_each(|num| {
                            if num_mode == 0 {
                                winning_numbers.push(num);
                            } else {
                                if winning_numbers.iter().any(|winner| winner == &num) {
                                    matches += 1;
                                    score = if score == 0 { 1 } else { score * 2 };
                                }
                            }
                        });
                }
            } else {
                id = Some(
                    part.split_whitespace()
                        .last()
                        .ok_or("Failed to get last item")?
                        .parse()?,
                )
            }
        }
        Ok(Card {
            id: id.ok_or("no id found")?,
            part_1_score: score,
            part_2_matches: matches,
            part_2_count: 1,
        })
    }
}

fn part1(input: String) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| line.parse::<Card>())
        .filter_map(|card| card.ok())
        .fold(0, |sum, card| sum + card.part_1_score))
}

fn part2(input: String) -> Result<usize> {
    let mut sum = 0;
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>())
        .filter_map(|card| card.ok())
        .collect_vec();
    let mut queue: VecDeque<&Card> = cards.iter().collect();
    while let Some(card) = queue.pop_front() {
        sum += 1;
        for card_index in card.id..card.id + card.part_2_matches {
            cards.get(card_index).map(|c| queue.push_back(&c));
        }
    }
    Ok(sum)
}

fn part2_revised(input: String) -> Result<usize> {
    let mut sum = 0;
    let mut cards = input
        .lines()
        .map(|line| line.parse::<Card>())
        .filter_map(|card| card.ok())
        .collect_vec();
    for i in 0..cards.len() {
        let (count, matches) = {
            let card = cards.get(i).ok_or("card not found")?;
            sum += card.part_2_count;
            (card.part_2_count, card.part_2_matches)
        };
        for card_index in 1..=matches {
            cards.get_mut(i + card_index).map(|card_below| {
                card_below.part_2_count += count;
            });
        }
    }
    Ok(sum)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Day 4");
    println!("=====");
    let input = utils::aoc::get_puzzle_input(4).await?;
    let start = Instant::now();
    println!("part 1: {}\t[{:?}]", part1(input.clone())?, start.elapsed());
    let start = Instant::now();
    println!("part 2: {}\t[{:?}]", part2(input.clone())?, start.elapsed());
    let start = Instant::now();
    println!(
        "part 2: {}\t[{:?}]\t[revised]",
        part2_revised(input.clone())?,
        start.elapsed()
    );
    Ok(())
}
