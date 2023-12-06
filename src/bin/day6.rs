use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
use std::{str::FromStr, time::Instant};

static DAY: u8 = 6;

#[derive(Debug, Builder, Clone)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn get_roots(&self) -> (usize, usize) {
        let a = -1.0;
        let b = self.time as f64;
        let c = -(self.record_distance as f64);
        let n1 = (-b + f64::sqrt(b * b - (4.0 * a * c))) / (2.0 * a);
        let n2 = (-b - f64::sqrt(b * b - (4.0 * a * c))) / (2.0 * a);
        if n1 < n2 {
            // floor and round up to ensure we win
            let n1 = f64::floor(n1) as usize + 1;

            // ceil and round down to ensure we don't exceed time cap
            let n2 = f64::ceil(n2) as usize - 1;
            (n1, n2)
        } else {
            let n1 = f64::ceil(n1) as usize - 1;
            let n2 = f64::floor(n2) as usize + 1;
            (n2, n1)
        }
    }

    fn get_ways_to_beat_count(&self) -> usize {
        let (n1, n2) = self.get_roots();
        if self.time < n1 {
            0
        } else {
            let n2 = usize::min(self.time, n2);
            n2 - n1 + 1
        }
    }
}

fn part1(races: &[Race]) -> Result<usize> {
    // algo
    let start = Instant::now();
    let mut answer = 1;
    for race in races {
        let ways_to_beat = race.get_ways_to_beat_count();
        println!(
            "Race: {:?}, roots: {:?}, ways to beat: {:?}",
            race,
            race.get_roots(),
            race.get_ways_to_beat_count()
        );
        answer *= ways_to_beat;
    }
    let algo_time = start.elapsed();

    // output
    println!("part 1: {answer}");
    println!("\talgo: {algo_time:?}");
    Ok(answer)
}

fn part2(races: &[Race]) -> Result<usize> {
    // algo
    let start = Instant::now();
    let answer = 0;
    let algo_time = start.elapsed();

    // output
    println!("part 2: {answer}");
    println!("\talgo: {algo_time:?}");
    Ok(answer)
}

#[tokio::main]
async fn main() -> Result<()> {
    // let input = utils::aoc::get_puzzle_input(DAY).await?;
    println!("Day {DAY}");
    println!("=====");
    let races: [Race; 4] = [
        Race {
            time: 53,
            record_distance: 250,
        },
        Race {
            time: 91,
            record_distance: 1330,
        },
        Race {
            time: 67,
            record_distance: 1081,
        },
        Race {
            time: 68,
            record_distance: 1025,
        },
    ];
    let races: [Race; 1] = [Race {
        time: 53916768,
        record_distance: 250133010811025,
    }];
    let races: [Race; 3] = [
        Race {
            time: 7,
            record_distance: 9,
        },
        Race {
            time: 15,
            record_distance: 40,
        },
        Race {
            time: 30,
            record_distance: 200,
        },
    ];
    let races: [Race; 1] = [Race {
        time: 71530,
        record_distance: 940200,
    }];
    part1(&races)?;
    part2(&races)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static RACES: [Race; 3] = [
        Race {
            time: 7,
            record_distance: 9,
        },
        Race {
            time: 15,
            record_distance: 40,
        },
        Race {
            time: 30,
            record_distance: 200,
        },
    ];

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part1(&RACES)?, 288);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part2(&RACES)?, 0);
        Ok(())
    }
}
