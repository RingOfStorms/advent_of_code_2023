use aoc23::prelude::*;
use grid::Grid;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, time::Instant};

static DAY: u8 = 14;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Rock {
    Round,
    Square,
    None,
}

impl Rock {
    fn from_char(char: char) -> Self {
        match char {
            'O' => Rock::Round,
            '#' => Rock::Square,
            '.' => Rock::None,
            _ => panic!("unknown rock type"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Dish {
    grid: Grid<Rock>,
}

impl std::hash::Hash for Dish {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for r in self.grid.iter() {
            r.hash(state);
        }
    }
}

impl Dish {
    fn new(s: &str) -> Result<Self> {
        let mut grid: Option<Grid<Rock>> = None;
        for line in s.trim().lines() {
            let rocks = line.chars().map(Rock::from_char).collect_vec();
            let len = rocks.len();
            match grid {
                Some(ref mut grid) => {
                    grid.push_row(rocks);
                }
                None => {
                    grid = Some(Grid::from_vec(rocks, len));
                }
            }
        }
        let grid = grid.unwrap();
        Ok(Dish { grid })
    }

    fn tilt_north(mut self) -> Self {
        for col_idx in 0..self.grid.cols() {
            let column = self.grid.iter_col(col_idx);
            let column_length = column.len();

            // start,end,round_count ranges for rolling zones
            let mut rolling_ranges: Vec<(usize, usize, usize)> = vec![];
            let mut in_range: Option<(usize, usize)> = None;
            for (row_idx, rock) in column
                .enumerate()
                // Add one at the end so we close out our range, it should be square so it is
                // treated as a wall
                .chain(std::iter::once((column_length, &Rock::Square)))
            {
                if let Rock::Square = rock {
                    if let Some((start, round_count)) = in_range {
                        // rolling range it only including round or none, not this wall.
                        rolling_ranges.push((start, row_idx - 1, round_count));
                        in_range = None;
                    }
                } else {
                    let is_round = matches!(rock, Rock::Round);
                    if let Some((_, ref mut round_count)) = in_range {
                        if is_round {
                            *round_count += 1;
                        }
                    } else {
                        in_range = Some((row_idx, if is_round { 1 } else { 0 }));
                    }
                }
            }

            // Go through ranges and set them to tilt north.
            rolling_ranges.iter().for_each(|(start, end, round_count)| {
                let mut round_remaining = *round_count;
                for i in *start..=*end {
                    if let Some(rock) = self.grid.get_mut(i, col_idx) {
                        if round_remaining > 0 {
                            *rock = Rock::Round;
                            round_remaining -= 1;
                        } else {
                            *rock = Rock::None;
                        }
                    } else {
                        panic!("no rock");
                    }
                }
            });
        }
        self
    }

    fn calculate_north_load(&self) -> usize {
        let len = self.grid.rows();
        self.grid
            .iter_rows()
            .enumerate()
            .map(|(row_idx, row)| {
                let multiplier = len - row_idx;
                multiplier * row.filter(|rock| matches!(rock, Rock::Round)).count()
            })
            .sum()
    }
}

impl Display for Dish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter_rows() {
            for rock in row {
                match rock {
                    Rock::Round => f.write_str("O"),
                    Rock::Square => f.write_str("#"),
                    Rock::None => f.write_str("."),
                }?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let mut dish = Dish::new(&input)?;
    dish = dish.tilt_north();
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let answer = dish.calculate_north_load();
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
    let mut dish = Dish::new(&input)?;
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let cycles = 1000000000;
    let mut seen = HashMap::new();
    let mut scores = vec![];
    let mut answer = 0;
    for i in 0..cycles {
        for _ in 0..4 {
            dish = dish.tilt_north();
            dish.grid.rotate_right();
        }
        scores.push(dish.calculate_north_load());
        if let Some(repeated_index) = seen.get(&dish) {
            // Figure out final score based on sliding window of known scores within the
            // repeated pattern:
            //
            // (cycles - repeated index) gives us the remaining times we need to run through. We
            // then get the remainder of that divided by the difference of the current index minus
            // all seen (total repeating count).
            answer = scores
                [repeated_index - 1 + (cycles - repeated_index) % (seen.len() - repeated_index)];
            break;
        }
        seen.insert(dish.clone(), i);
    }
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

    static DATA: &'static str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part1(DATA.to_owned())?, 136);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part2(DATA.to_owned())?, 64);
        Ok(())
    }
}
