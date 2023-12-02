use aoc23::prelude::*;
use std::{collections::HashMap, error::Error, fmt::Display, str::FromStr, string::ParseError};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Cube {
    Blue,
    Red,
    Green,
}

impl FromStr for Cube {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Cube::Blue),
            "red" => Ok(Cube::Red),
            "green" => Ok(Cube::Green),
            _ => Err("asd".into()),
        }
    }
}

impl Cube {
    pub fn as_str(&self) -> &str {
        match self {
            Cube::Blue => "blue",
            Cube::Red => "red",
            Cube::Green => "green",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Game {
    id: u32,
    sets: Vec<HashMap<Cube, u32>>,
}

impl FromStr for Game {
    type Err = String;

    // Game 1: 7 green, 4 blue, 3 red; 4 blue, 10 red, 1 green; 1 blue, 9 red Game 2:
    // 2 red, 4 blue, 3 green; 5 green, 3 red, 1 blue; 3 green, 5 blue, 3 red Game 3:
    // 12 red, 1 blue; 6 red, 2 green, 3 blue; 2 blue, 5 red, 3 green
    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(':').collect();
        let id = parts[0]
            .trim()
            .split_whitespace()
            .last()
            .ok_or("no id")?
            .parse::<u32>()
            .ok()
            .ok_or("not a number")?;
        let sets = parts[1]
            .split(';')
            .map(|set| {
                set.split(',')
                    .filter_map(|part| {
                        let mut iter = part.trim().split_whitespace();
                        let count = iter.next().unwrap().parse::<u32>().unwrap();
                        let color = iter.next().unwrap().parse::<Cube>().ok()?;
                        Some((color, count))
                    })
                    .collect()
            })
            .collect();
        Ok(Game { id, sets })
    }
}

fn part1(input: String) -> Result<impl Display> {
    let games: Vec<Game> = input
        .lines()
        .into_iter()
        .map(|l| l.parse().expect("failed to parse line"))
        .collect();
    let mut possible_game_id_sum = 0;
    let limit_red = 12;
    let limit_green = 13;
    let limit_blue = 14;
    for game in games {
        let out_of_bounds = game.sets.iter().any(|set| match set.get(&Cube::Red) {
            Some(red) => red > &limit_red,
            None => false,
        } || match set.get(&Cube::Green) {
            Some(green) => green > &limit_green,
            None => false,
        } || match set.get(&Cube::Blue) {
            Some(blue) => blue > &limit_blue,
            None => false,
        });
        if !out_of_bounds {
            // println!("Valid game id, {:?}", game);
            possible_game_id_sum += game.id;
        } else {
            // println!("INVALID game {:?}", game);
        }
    }
    Ok(possible_game_id_sum)
}

fn part2(input: String) -> Result<impl Display> {
    let games: Vec<Game> = input
        .lines()
        .into_iter()
        .map(|l| l.parse().expect("failed to parse line"))
        .collect();
    let mut power_sum = 0;
    for game in games {
        let mut maxes: HashMap<Cube, u32> = [(Cube::Red, 1), (Cube::Green, 1), (Cube::Blue, 1)]
            .into_iter()
            .collect();
        for set in game.sets.iter() {
            for (cube, &count) in set {
                let max_count = maxes.get_mut(cube).ok_or("default missing")?;
                *max_count = u32::max(*max_count, count);
            }
        }
        let power = maxes.values().into_iter().fold(1u32, |acc, v| acc * v);
        power_sum += power;
    }
    Ok(power_sum)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Day 1");
    println!("=====");
    let input = utils::aoc::get_puzzle_input(2).await?;

    // ```
    // let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    //     .to_owned();
    // ```
    println!("part 1: {}", part1(input.clone())?);
    println!("part 2: {}", part2(input.clone())?);
    Ok(())
}
