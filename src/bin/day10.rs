use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
use std::{str::FromStr, time::Instant};

static DAY: u8 = 10;

#[derive(Debug, Clone)]
enum PipeDirection {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    Ground,
}

#[derive(Debug, Clone)]
struct Pipe {
    position: (usize, usize),
    direction: PipeDirection,
}

impl Pipe {
    fn for_char(c: char, position: (usize, usize)) -> Self {
        match c {
            '|' => Self {
                position,
                direction: PipeDirection::Vertical,
            },
            '-' => Self {
                position,
                direction: PipeDirection::Horizontal,
            },
            'L' => Self {
                position,
                direction: PipeDirection::NorthEast,
            },
            'J' => Self {
                position,
                direction: PipeDirection::NorthWest,
            },
            '7' => Self {
                position,
                direction: PipeDirection::SouthWest,
            },
            'F' => Self {
                position,
                direction: PipeDirection::SouthEast,
            },
            '.' => Self {
                position,
                direction: PipeDirection::Ground,
            },
            'S' => Self {
                position,
                direction: PipeDirection::Start,
            },
            _ => panic!("invalid pipe char"),
        }
    }

    fn connects_to(&self) -> Vec<(usize, usize)> {
        let deltas = match self.direction {
            PipeDirection::Vertical => Some([(-1i64, 0i64), (1, 0)]),
            PipeDirection::Horizontal => Some([(0, -1), (0, 1)]),
            PipeDirection::NorthEast => Some([(-1, 0), (0, 1)]),
            PipeDirection::NorthWest => Some([(-1, 0), (0, -1)]),
            PipeDirection::SouthWest => Some([(1, 0), (0, -1)]),
            PipeDirection::SouthEast => Some([(1, 0), (0, 1)]),
            _ => None,
        };
        match deltas {
            Some(deltas) => {
                let mut to = vec![];
                for delta in deltas {
                    let (row_delta, col_delta) = delta;
                    let (row, col) = self.position;
                    if row == 0 && row_delta < 0 {
                        continue;
                    }
                    if col == 0 && col_delta < 0 {
                        continue;
                    }
                    to.push((
                        (row as i64 + row_delta).try_into().unwrap(),
                        (col as i64 + col_delta).try_into().unwrap(),
                    ));
                }
                to
            }
            None => vec![],
        }
    }
}

#[derive(Debug, Clone)]
struct Pipes {
    grid: Vec<Vec<Pipe>>,
    start: Pipe,
}

impl Pipes {
    fn new(s: &str) -> Result<Self> {
        let mut start = None;
        let mut grid = vec![];
        for (row_index, line) in s.trim().lines().enumerate() {
            let mut row = vec![];
            for (col_index, char) in line.trim().chars().enumerate() {
                row.push(Pipe::for_char(char, (row_index, col_index)));
                if char == 'S' {
                    start = Some((row_index, col_index))
                }
            }
            grid.push(row);
        }

        // Get actual start pipe variant for easier code on the rest of the walking code
        // later
        let start = start
            .map(|position| {
                let mut up = true;
                let mut left = true;

                // We assume start always has two connections by definition of the problem. Will
                // assume it is up and left and swap to the opposite after checking those.
                let (row, col) = position;
                if let Some(down) = grid.get(row + 1) {
                    match down[col].direction {
                        PipeDirection::Vertical
                        | PipeDirection::NorthEast
                        | PipeDirection::NorthWest => {
                            up = false;
                        }
                        _ => {}
                    }
                }
                if let Some(right) = grid[row].get(col + 1) {
                    match right.direction {
                        PipeDirection::Horizontal
                        | PipeDirection::NorthWest
                        | PipeDirection::SouthWest => {
                            left = false;
                        }
                        _ => {}
                    }
                }
                if up && left {
                    Pipe {
                        position,
                        direction: PipeDirection::NorthWest,
                    }
                } else if up && !left {
                    Pipe {
                        position,
                        direction: PipeDirection::NorthEast,
                    }
                } else if !up && left {
                    Pipe {
                        position,
                        direction: PipeDirection::SouthWest,
                    }
                } else {
                    Pipe {
                        position,
                        direction: PipeDirection::SouthEast,
                    }
                }
            })
            .expect("start");
        Ok(Self { grid, start })
    }

    fn get(&self, position: &(usize, usize)) -> &Pipe {
        &self.grid[position.0][position.1]
    }
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let pipes = Pipes::new(&input)?;
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let mut answer = 1;
    let mut starters = pipes.start.connects_to().into_iter();
    let mut paths: [(&Pipe, &Pipe); 2] = [
        (&pipes.start, pipes.get(&starters.next().unwrap())),
        (&pipes.start, pipes.get(&starters.next().unwrap())),
    ];
    loop {
        // reached end
        if paths[0].1.position.0 == paths[1].1.position.0
            && paths[0].1.position.1 == paths[1].1.position.1
        {
            break;
        }
        answer += 1;
        for path in paths.iter_mut() {
            let next = path
                .1
                .connects_to()
                .into_iter()
                .filter(|pipe| pipe.0 != path.0.position.0 || pipe.1 != path.0.position.1)
                .next()
                .map(|pos| pipes.get(&pos))
                .unwrap();
            *path = (path.1, next);
        }
    }
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
    let pipes = Pipes::new(&input)?;
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let answer = 0;
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

    // part2(input.clone())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &'static str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part1(DATA.to_owned())?, 8);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part2(DATA.to_owned())?, 0);
        Ok(())
    }
}
