use aoc23::prelude::*;
use derive_builder::Builder;
use grid::Grid;
use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashSet, ops::Add, str::FromStr, time::Instant};

static DAY: u8 = 16;

#[derive(Debug, Clone, Default)]
enum Tile {
    #[default]
    Empty,
    VerticalSplit,
    HorizontalSplit,
    RMirror,
    LMirror,
}

impl Tile {
    fn new_tile_grid(input: &str) -> Grid<Self> {
        let mut grid = Grid::new(0, 0);
        for line in input.lines() {
            let tiles = line
                .trim()
                .chars()
                .map(|c| match c {
                    '.' => Self::Empty,
                    '|' => Self::VerticalSplit,
                    '-' => Self::HorizontalSplit,
                    '/' => Self::RMirror,
                    '\\' => Self::LMirror,
                    unknown => panic!("unknown tile: {unknown}"),
                })
                .collect_vec();
            grid.push_row(tiles);
        }
        grid
    }

    fn direction_to(&self, direction: &Dir) -> Vec<Dir> {
        match self {
            Tile::Empty => vec![direction.clone()],
            Tile::VerticalSplit => match direction {
                Dir::Up | Dir::Down => vec![direction.clone()],
                Dir::Right | Dir::Left => vec![Dir::Up, Dir::Down],
            },
            Tile::HorizontalSplit => match direction {
                Dir::Right | Dir::Left => vec![direction.clone()],
                Dir::Up | Dir::Down => vec![Dir::Right, Dir::Left],
            },
            Tile::RMirror => vec![match direction {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Up,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Down,
            }],
            Tile::LMirror => vec![match direction {
                Dir::Up => Dir::Left,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Up,
            }],
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Beam(usize, usize, Dir);

impl Beam {
    fn add_dir(&self, dir: &Dir) -> Self {
        Self(
            match dir {
                Dir::Up => self.0.saturating_sub(1),
                Dir::Down => self.0.saturating_add(1),
                _ => self.0,
            },
            match dir {
                Dir::Left => self.1.saturating_sub(1),
                Dir::Right => self.1.saturating_add(1),
                _ => self.1,
            },
            dir.clone(),
        )
    }

    fn next_path(&self, tiles: &Grid<Tile>) -> Vec<Beam> {
        tiles
            .get(self.0, self.1)
            .map(|tile| tile.direction_to(&self.2))
            .map(|dirs| {
                dirs.iter()
                    .map(|dir| self.add_dir(dir))
                    .filter(|beam| tiles.get(beam.0, beam.1).is_some())
                    .collect_vec()
            })
            .unwrap_or(vec![])
    }

    fn part2_starts(grid: &Grid<Tile>) -> Vec<Self> {
        let mut starts = vec![];
        let rows = grid.rows();
        let cols = grid.cols();
        starts.push(Beam(0, 0, Dir::Right));
        starts.push(Beam(0, 0, Dir::Down));
        starts.push(Beam(0, cols - 1, Dir::Down));
        starts.push(Beam(0, cols - 1, Dir::Left));
        starts.push(Beam(rows - 1, cols - 1, Dir::Left));
        starts.push(Beam(rows - 1, cols - 1, Dir::Up));
        starts.push(Beam(rows - 1, 0, Dir::Up));
        starts.push(Beam(rows - 1, 0, Dir::Right));
        for row in 1..rows - 2 {
            starts.push(Beam(row, 0, Dir::Right));
            starts.push(Beam(row, cols - 1, Dir::Left));
        }
        for col in 1..cols - 2 {
            starts.push(Beam(0, col, Dir::Down));
            starts.push(Beam(rows - 1, col, Dir::Up));
        }
        starts
    }
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let grid = Tile::new_tile_grid(&input);
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let mut energized = HashSet::new();
    let mut full_beam = HashSet::new();
    let mut beam_path = vec![Beam(0, 0, Dir::Right)];
    while let Some(beam) = beam_path.pop() {
        energized.insert((beam.0, beam.1));
        full_beam.insert(beam.clone());
        beam.next_path(&grid)
            .into_iter()
            .filter(|b| !full_beam.contains(b))
            .for_each(|new_beam| {
                beam_path.push(new_beam);
            });
    }
    let answer = energized.len();
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
    let grid = Tile::new_tile_grid(&input);
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let answer = Beam::part2_starts(&grid)
        .into_iter()
        .map(|start| {
            let mut energized = HashSet::new();
            let mut full_beam = HashSet::new();
            let mut beam_path = vec![start];
            while let Some(beam) = beam_path.pop() {
                energized.insert((beam.0, beam.1));
                full_beam.insert(beam.clone());
                beam.next_path(&grid)
                    .into_iter()
                    .filter(|b| !full_beam.contains(b))
                    .for_each(|new_beam| {
                        beam_path.push(new_beam);
                    });
            }
            energized.len()
        })
        .max()
        .unwrap();
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
                r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
                    .to_owned(),
            )?,
            46
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(
            part2(
                r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
                    .to_owned(),
            )?,
            51
        );
        Ok(())
    }
}
