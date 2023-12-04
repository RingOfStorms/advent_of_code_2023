use aoc23::prelude::*;
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
    matches: usize,
    score: usize,
}

impl FromStr for Card {
    type Err = BoxE;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut id: Option<usize> = None;
        let mut winning_numbers: Vec<usize> = vec![];
        let mut numbers: Vec<usize> = vec![];
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
                                numbers.push(num);
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
            winning_numbers,
            numbers,
            score,
            matches,
        })
    }
}

fn part1(input: String) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| line.parse::<Card>())
        .filter_map(|card| card.ok())
        .fold(0, |sum, card| sum + card.score))
}

fn part2(input: String) -> Result<usize> {
    let mut sum = 0;
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>())
        .filter_map(|card| card.ok())
        .collect_vec();
    let mut queue: VecDeque<&Card> = cards.iter().collect();
    let mut counts: HashMap<usize, usize> = HashMap::new();
    while let Some(card) = queue.pop_front() {
        let counter = counts.entry(card.id).or_insert(0);
        *counter += 1;
        sum += 1;
        for card_index in card.id..card.id + card.matches {
            cards.get(card_index).map(|c| queue.push_back(&c));
        }
    }
    Ok(sum)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Day 4");
    println!("=====");
    let input = utils::aoc::get_puzzle_input(4).await?;
    println!("part 1: {}", part1(input.clone())?);
    println!("part 2: {}", part2(input.clone())?);
    Ok(())
}
