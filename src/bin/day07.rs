use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr, time::Instant};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Strength {
    #[cfg(feature = "part2")]
    fn for_hand(hand: &Hand) -> Self {
        let mut counts = [0; 15];
        let mut jokers = 0;
        for card in hand.cards.iter() {
            if card == &1 {
                jokers += 1;
            } else {
                counts[*card as usize - 2] += 1;
            }
        }
        let mut strength = counts
            .iter()
            .fold(Strength::HighCard, |strength, &count| match count {
                5 => Strength::FiveOfAKind,
                4 => Strength::FourOfAKind,
                3 => match strength {
                    Strength::TwoPair => Strength::FullHouse,
                    Strength::OnePair => Strength::FullHouse,
                    _ => Strength::ThreeOfAKind,
                },
                2 => match strength {
                    Strength::ThreeOfAKind => Strength::FullHouse,
                    Strength::OnePair => Strength::TwoPair,
                    _ => Strength::OnePair,
                },
                _ => strength,
            });
        while jokers > 0 {
            strength = match strength {
                Strength::HighCard => Strength::OnePair,
                Strength::OnePair => Strength::ThreeOfAKind,
                Strength::TwoPair => Strength::FullHouse,
                Strength::ThreeOfAKind => Strength::FourOfAKind,
                Strength::FullHouse => Strength::FourOfAKind,
                Strength::FourOfAKind => Strength::FiveOfAKind,
                Strength::FiveOfAKind => Strength::FiveOfAKind,
            };
            jokers -= 1;
        }
        strength
    }

    #[cfg(feature = "part1")]
    fn for_hand(hand: &Hand) -> Self {
        let mut counts = [0; 15];
        for card in hand.cards.iter() {
            counts[*card as usize - 2] += 1;
        }
        counts
            .iter()
            .fold(Strength::HighCard, |strength, &count| match count {
                5 => Strength::FiveOfAKind,
                4 => Strength::FourOfAKind,
                3 => match strength {
                    Strength::TwoPair => Strength::FullHouse,
                    Strength::OnePair => Strength::FullHouse,
                    _ => Strength::ThreeOfAKind,
                },
                2 => match strength {
                    Strength::ThreeOfAKind => Strength::FullHouse,
                    Strength::OnePair => Strength::TwoPair,
                    _ => Strength::OnePair,
                },
                _ => strength,
            })
    }
}

#[derive(Debug, Builder, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<u32>,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_strength = Strength::for_hand(self);
        let other_strength = Strength::for_hand(other);
        match self_strength.cmp(&other_strength) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(s, o)| s.cmp(o))
                .skip_while(|ord| matches!(ord, Ordering::Equal))
                .next()
                .unwrap_or(Ordering::Equal),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = BoxE;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut splits = s.trim().split_whitespace();
        Ok(HandBuilder::default()
            .cards(
                splits
                    .next()
                    .unwrap()
                    .chars()
                    .map(|c| {
                        if let Some(digit) = c.to_digit(10) {
                            digit
                        } else {
                            match c {
                                #[cfg(feature = "part1")]
                                'A' => 14,
                                #[cfg(feature = "part2")]
                                'A' => 13,
                                #[cfg(feature = "part1")]
                                'K' => 13,
                                #[cfg(feature = "part2")]
                                'K' => 12,
                                #[cfg(feature = "part1")]
                                'Q' => 12,
                                #[cfg(feature = "part2")]
                                'Q' => 11,
                                #[cfg(feature = "part1")]
                                'J' => 11,
                                #[cfg(feature = "part2")]
                                'J' => 1,
                                'T' => 10,
                                _ => panic!("invalid card: {}", c),
                            }
                        }
                    })
                    .collect(),
            )
            .bid(splits.next().unwrap().parse()?)
            .build()?)
    }
}

fn calculate(input: String) -> Result<usize> {
    let start = Instant::now();
    let answer = input
        .lines()
        .map(|line| line.parse::<Hand>().unwrap())
        .sorted()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1))
        .sum();
    let algo_time = start.elapsed();
    println!("Day {DAY}: {answer}");
    println!("\t{algo_time:?}");
    Ok(answer)
}

// TODO come back and revise for a faster solution
#[tokio::main]
async fn main() -> Result<()> {
    println!("Day {DAY}");
    println!("=====");
    let input = utils::aoc::get_puzzle_input(DAY).await?;
    calculate(input.clone())?;
    Ok(())
}

static DAY: u8 = 7;

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test() -> Result<()> {
        #[cfg(feature = "part1")]
        assert_eq!(calculate(DATA.to_owned())?, 6440);
        #[cfg(feature = "part2")]
        assert_eq!(calculate(DATA.to_owned())?, 5905);
        Ok(())
    }
}
