use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
use std::{fmt::Debug, str::FromStr, time::Instant};

static DAY: u8 = 13;

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<bool>>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            f.write_str("\t");
            for b in row {
                f.write_str(if *b { "#" } else { "." });
            }
            f.write_str("\n");
        }
        Ok(())
    }
}

type Split<'a> = (&'a [Vec<bool>], &'a [Vec<bool>]);

impl Grid {
    fn new(s: &str) -> Result<Self> {
        Ok(Self {
            cells: s
                .trim()
                .lines()
                .map(|l| l.chars().map(|c| c == '#').collect_vec())
                .collect(),
        })
    }

    fn all_vertical_splits(&self) -> Vec<Split> {
        let mut splits = vec![];
        for i in 1..self.cells.len() {
            splits.push(self.vertical_split(i));
        }
        splits
    }

    fn vertical_split(&self, index: usize) -> Split {
        (&self.cells[0..index], &self.cells[index..])
    }

    fn is_vertical_mirror(top: &[Vec<bool>], bot: &[Vec<bool>]) -> bool {
        let tl = top.len();
        let bl = bot.len();
        for i in 0..usize::min(tl, bl) {
            let t = &top[tl - 1 - i];
            let b = &bot[i];

            // println!("\tChecking {i}: {t:?} <=> {b:?} || {}", t == b);
            if t != b {
                return false;
            }
        }
        true
    }

    fn is_vertical_mirror_with_smudge(top: &[Vec<bool>], bot: &[Vec<bool>]) -> bool {
        let tl = top.len();
        let bl = bot.len();
        let mut claimed_smudge = false;
        for i in 0..usize::min(tl, bl) {
            let t = &top[tl - 1 - i];
            let b = &bot[i];

            // println!( "\tChecking {i}: {t:?} <=> {b:?} || {} || {}", t == b, claimed_smudge
            // );
            if t != b {
                if !claimed_smudge && t.iter().zip(b.iter()).filter(|(z, x)| z != x).count() == 1 {
                    claimed_smudge = true
                } else {
                    return false;
                }
            }
        }

        // FFS I had `true` here but all mirrors have EXACTLY ONE smudge, not at most
        // one...
        claimed_smudge
    }

    fn clone_and_rotate(&self) -> Self {
        Self {
            cells: (0..self.cells[0].len())
                .map(|i| self.cells.iter().map(|row| row[i]).collect())
                .collect(),
        }
    }
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let grids = input
        .split("\n\n")
        .map(|g| Grid::new(g).unwrap())
        .collect_vec();
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let answer: usize = grids
        .iter()
        .map(|grid| {
            let mut vertical_mirror_at = None;
            for (idx, (top, bot)) in grid.all_vertical_splits().iter().enumerate() {
                if Grid::is_vertical_mirror(top, bot) {
                    vertical_mirror_at = Some(idx);
                    break;
                }
            }
            let mut horizontal_mirror_at = None;
            if vertical_mirror_at.is_none() {
                let rotated = grid.clone_and_rotate();
                for (idx, (top, bot)) in rotated.all_vertical_splits().iter().enumerate() {
                    if Grid::is_vertical_mirror(top, bot) {
                        horizontal_mirror_at = Some(idx);
                        break;
                    }
                }
            }

            // index + 1 = number of rows to the left or above the mirror index
            let value = horizontal_mirror_at
                .map(|h| h + 1)
                .or(vertical_mirror_at.map(|v| (v + 1) * 100))
                .unwrap_or(0);

            // println!( "Grid: vertical mirror at: {vertical_mirror_at:?}\thorizontal at:
            // {horizontal_mirror_at:?} == {value}" );
            value
        })
        .sum();
    let algo_time = a_start.elapsed();

    // output
    println!("part 1: {answer}\t[total: {:?}]", start.elapsed());
    println!("\tparse: {parsed_time:?}");
    println!("\talgo: {algo_time:?}");
    Ok(answer)
}

fn part2(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let grids = input
        .split("\n\n")
        .map(|g| Grid::new(g).unwrap())
        .collect_vec();
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let answer: usize = grids
        .iter()
        .map(|grid| {
            // println!("GRID: {grid:?}");
            let mut vertical_mirror_at = None;
            for (idx, (top, bot)) in grid.all_vertical_splits().iter().enumerate() {
                if Grid::is_vertical_mirror_with_smudge(top, bot) {
                    // println!("GOT VERT: {idx}");
                    vertical_mirror_at = Some(idx);
                    break;
                }
            }
            let mut horizontal_mirror_at = None;
            if vertical_mirror_at.is_none() {
                let rotated = grid.clone_and_rotate();

                // println!("ROTATED: {grid:?}");
                for (idx, (top, bot)) in rotated.all_vertical_splits().iter().enumerate() {
                    if Grid::is_vertical_mirror_with_smudge(top, bot) {
                        horizontal_mirror_at = Some(idx);
                        break;
                    }
                }
            }

            // index + 1 = number of rows to the left or above the mirror index
            let value = horizontal_mirror_at
                .map(|h| h + 1)
                .or(vertical_mirror_at.map(|v| (v + 1) * 100))
                .unwrap_or(0);

            // println!( "Grid: vertical mirror at: {vertical_mirror_at:?}\thorizontal at:
            // {horizontal_mirror_at:?} == {value}" );
            value
        })
        .sum();
    let algo_time = a_start.elapsed();

    // output
    println!("part 2: {answer} == 31836\t[total: {:?}]", start.elapsed());
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

    static DATA: &'static str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part1(DATA.to_owned())?, 405);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part2(DATA.to_owned())?, 400);
        Ok(())
    }
}
