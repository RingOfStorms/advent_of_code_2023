use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
use std::{str::FromStr, time::Instant};

static DAY: u8 = 11;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Vec2D {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Vec2D {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, Clone)]
struct Space {
    galaxies: Vec<Vec2D>,
    x_max: usize,
    y_max: usize,
}

impl Space {
    fn new(s: &str) -> Result<Self> {
        let mut x_max = 0;
        let mut y_max = 0;
        let mut galaxies = vec![];
        for (y, row) in s.trim().lines().enumerate() {
            for (x, col) in row.chars().enumerate() {
                if col == '#' {
                    galaxies.push((x, y).into());
                }
                x_max = x;
            }
            y_max = y;
        }
        Ok(Space {
            galaxies,
            x_max,
            y_max,
        })
    }

    fn expand(&mut self, by: usize) {
        for y in (0..=self.y_max).rev() {
            if self.galaxies.iter().filter(|g| g.y == y).count() == 0 {
                self.galaxies
                    .iter_mut()
                    .filter(|g| g.y > y)
                    .for_each(|g| g.y += by);
            }
        }
        for x in (0..=self.x_max).rev() {
            if self.galaxies.iter().filter(|g| g.x == x).count() == 0 {
                self.galaxies
                    .iter_mut()
                    .filter(|g| g.x > x)
                    .for_each(|g| g.x += by);
            }
        }
    }

    fn galaxy_pairs(&self) -> Vec<(&Vec2D, &Vec2D)> {
        let mut galaxy_pairs = vec![];
        for i in 0..self.galaxies.len() {
            for i2 in i + 1..self.galaxies.len() {
                galaxy_pairs.push((&self.galaxies[i], &self.galaxies[i2]));
            }
        }
        galaxy_pairs
    }
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let mut space = Space::new(&input)?;
    let parsed_time = start.elapsed();
    let step_start = Instant::now();
    space.expand(1);
    let expansion_time = step_start.elapsed();

    // algo
    let step_start = Instant::now();
    let mut answer = 0;
    for pair in space.galaxy_pairs() {
        let distance = pair.0.y.abs_diff(pair.1.y) + pair.0.x.abs_diff(pair.1.x);
        answer += distance;
    }
    let algo_time = step_start.elapsed();

    // output
    println!("part 1: {answer}\t[total: {:?}]", start.elapsed());
    println!("\tparse: {parsed_time:?}");
    println!("\texpansion: {expansion_time:?}");
    println!("\talgo: {algo_time:?}");
    Ok(answer)
}

fn part2(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let mut space = Space::new(&input)?;
    let parsed_time = start.elapsed();
    let step_start = Instant::now();
    space.expand(1000000 - 1);
    let expansion_time = step_start.elapsed();

    // algo
    let step_start = Instant::now();
    let mut answer = 0;
    for pair in space.galaxy_pairs() {
        let distance = pair.0.y.abs_diff(pair.1.y) + pair.0.x.abs_diff(pair.1.x);
        answer += distance;
    }
    let algo_time = step_start.elapsed();

    // output
    println!("part 2: {answer}\t[total: {:?}]", start.elapsed());
    println!("\tparse: {parsed_time:?}");
    println!("\texpansion: {expansion_time:?}");
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

    static DATA: &'static str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part1(DATA.to_owned())?, 374);
        Ok(())
    }

    #[test]
    // Must set the space.expand(1000000 - 1); to (100 - 1) instead for this test
    #[ignore]
    fn test_part_2() -> Result<()> {
        assert_eq!(part2(DATA.to_owned())?, 8410);
        Ok(())
    }
}
