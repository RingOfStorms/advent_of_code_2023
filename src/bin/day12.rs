use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashMap, str::FromStr, time::Instant};

static DAY: u8 = 12;

fn count_possibilities(
    layout: &str,
    contiguous: &[usize],
    cache: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    if layout.chars().count() == 0 {
        return if contiguous.len() == 0 {
            // println!("\tLine is variant: {layout}");
            1
        } else {
            0
        };
    }
    let cache_key = (layout.to_owned(), contiguous.to_vec());
    if let Some(&result) = cache.get(&cache_key) {
        return result;
    }

    // Remove leading dots
    let result = if layout.starts_with('.') {
        count_possibilities(layout.to_owned().trim_matches('.'), contiguous, cache)
        // Try both options for ?
    } else if layout.starts_with('?') {
        count_possibilities(&layout.to_owned().replacen('?', &".", 1), contiguous, cache)
            + count_possibilities(&layout.to_owned().replacen('?', &"#", 1), contiguous, cache)
        // If group, check if it matches
    } else if layout.starts_with("#") {
        // no groups left to match || not enough # to make the group || not all of this
        // group are #
        if contiguous.len() == 0
            || layout.chars().count() < contiguous[0]
            || layout.chars().take(contiguous[0]).any(|c| c == '.')
        {
            0
        } else if contiguous.len() > 1 {
            if layout.len() < contiguous[0] + 1
                || layout.chars().skip(contiguous[0]).next().unwrap() == '#'
            {
                0
            } else {
                count_possibilities(
                    &layout.chars().skip(contiguous[0] + 1).collect::<String>(),
                    &contiguous[1..],
                    cache,
                )
            }
        } else {
            count_possibilities(
                &layout.chars().skip(contiguous[0]).collect::<String>(),
                &contiguous[1..],
                cache,
            )
        }
    } else {
        panic!("Should not get here.")
    };
    cache.insert(cache_key, result);
    result
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let answer = input
        .trim()
        .lines()
        .enumerate()
        .par_bridge()
        .map(|(idx, line)| {
            let mut split = line.trim().split_whitespace();
            let layout = split.next().unwrap();
            let contiguous = split
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            let count = count_possibilities(&layout, &contiguous, &mut HashMap::new());

            // println!("Line {idx}: {line} = {count}");
            count
        })
        .sum();

    // output
    println!("part 1: {answer}\t[total: {:?}]", start.elapsed());
    Ok(answer)
}

fn part2(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let answer = input
        .trim()
        .lines()
        .enumerate()
        .par_bridge()
        .map(|(idx, line)| {
            let mut split = line.trim().split_whitespace();
            let layout = split.next().unwrap();
            let contiguous = split
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            let layout = std::iter::repeat(layout).take(5).collect_vec().join("?");
            let contiguous = std::iter::repeat(contiguous)
                .take(5)
                .flatten()
                .collect_vec();
            let count = count_possibilities(&layout, &contiguous, &mut HashMap::new());

            // println!("Line {idx}: {line} = {count}");
            count
        })
        .sum();

    // output
    println!("part 2: {answer}\t[total: {:?}]", start.elapsed());
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

    static DATA: &'static str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part1(DATA.to_owned())?, 21);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part2(DATA.to_owned())?, 525152);
        Ok(())
    }
}
