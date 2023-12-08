use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
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
        let mut possible_counts: Vec<[i32; 14]> = vec![[0; 14]];

        // let mut counts = [0; 14]; let mut jokers = 0;
        for card in hand.cards.iter() {
            if card == &1 {
                let mut new_possible_counts: Vec<[i32; 14]> = vec![];
                for possible_count in possible_counts.iter() {
                    for i in 0..14 {
                        let mut new_counts = possible_count.clone();
                        new_counts[i] += 1;
                        new_possible_counts.push(new_counts);
                    }
                }
                possible_counts = new_possible_counts;
            } else {
                for counts in possible_counts.iter_mut() {
                    counts[*card as usize - 2] += 1;
                }
            }
        }
        possible_counts.iter().map(|counts| {
            counts.iter().fold(Strength::HighCard, |strength, &count| {
                std::cmp::max(
                    strength.clone(),
                    match count {
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
                    },
                )
            })
        }).sorted().next().unwrap()
    }

    #[cfg(not(feature = "part2"))]
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
                                #[cfg(not(feature = "part2"))]
                                'A' => 14,
                                #[cfg(feature = "part2")]
                                'A' => 13,
                                #[cfg(not(feature = "part2"))]
                                'K' => 13,
                                #[cfg(feature = "part2")]
                                'K' => 12,
                                #[cfg(not(feature = "part2"))]
                                'Q' => 12,
                                #[cfg(feature = "part2")]
                                'Q' => 11,
                                #[cfg(not(feature = "part2"))]
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
        .sorted().collect_vec();
        // .enumerate()
        // .map(|(idx, hand)| hand.bid * (idx + 1))
        // .sum();
    let algo_time = start.elapsed();

    // ```
    for i in answer {
        println!("{:?}: {:?}", i, Strength::for_hand(&i));
    }
    let answer = 0;
    // ```
    //
    // output
    println!("Day 5: {answer}");
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
        #[cfg(not(feature = "part2"))]
        assert_eq!(calculate(DATA.to_owned())?, 6440);
        #[cfg(feature = "part2")]
        assert_eq!(calculate(DATA.to_owned())?, 5905);
        Ok(())
    }
}
