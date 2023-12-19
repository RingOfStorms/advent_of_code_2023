use aoc23::prelude::*;
use derive_builder::Builder;
use geo::{Area, Coord, Coordinate, LineString, Polygon};
use itertools::Itertools;
use pathfinding::num_traits::{Float, Signed};
use rayon::prelude::*;
use std::{collections::HashSet, str::FromStr, time::Instant};

static DAY: u8 = 18;

#[derive(Debug, Clone)]
struct DigCommand {
    direction: char,
    distance: i32,
    color: String,
}

#[derive(Debug, Clone)]
struct DigPlan {
    commands: Vec<DigCommand>,
}

impl DigPlan {
    fn new(s: &str) -> Result<Self> {
        let commands = s
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() != 3 {
                    return Err("Invalid command format".into());
                }
                Ok(DigCommand {
                    direction: parts[0].chars().next().ok_or("Invalid direction")?,
                    distance: parts[1].parse()?,
                    color: parts[2].to_string(),
                })
            })
            .collect::<Result<Vec<DigCommand>>>()?;
        Ok(Self { commands })
    }

    fn calculate_poly(&self) -> geo::Polygon {
        let mut position = (0, 0);
        let mut visited: Vec<(i32, i32)> = vec![position];
        for command in &self.commands {
            let (dx, dy) = match command.direction {
                'U' => (0, -1),
                'D' => (0, 1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => unreachable!(),
            };
            for _ in 0..command.distance {
                position.0 += dx;
                position.1 += dy;
                visited.push(position);
            }
        }
        if *visited.last().unwrap() != *visited.first().unwrap() {
            panic!("expected a closed loop")
        }
        if visited.iter().sorted_unstable().dedup().count() != visited.len() - 1 {
            panic!("Loop intersected with itself more than the 1 allowed time for the start node");
        }
        println!("visited {}: {visited:?}", visited.len());
        let coordinates: Vec<Coordinate<f64>> = visited
            .into_iter()
            .map(|(x, y)| Coordinate {
                x: x as f64,
                y: y as f64,
            })
            .collect_vec();
        Polygon::new(LineString::from(coordinates), vec![])
        // visited.len()
    }
}

fn part1(input: String) -> Result<usize> {
    println!("Input\n====\n{input}\n\n");

    // parse
    let start = Instant::now();
    let dig_plan = DigPlan::new(&input)?;
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let polygon = dig_plan.calculate_poly();
    let answer = polygon.signed_area().abs();
    let algo_time = a_start.elapsed();

    // output
    println!("part 1: {answer}\t[total: {:?}]", start.elapsed());
    println!("\tparse: {parsed_time:?}");
    println!("\talgo: {algo_time:?}");
    Ok(answer as usize)
}

fn part2(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let dig_plan = DigPlan::new(&input)?;
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let mut answer = 0;
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
    part2(input.clone())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(
            part1(
                "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
                    .to_owned(),
            )?,
            62
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(
            part2(
                "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
                    .to_owned(),
            )?,
            0
        );
        Ok(())
    }
}
