use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
use std::{str::FromStr, time::Instant};

#[derive(Debug, Builder, Clone)]
struct Map {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl FromStr for Map {
    type Err = BoxE;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("failed to parse"));
        Ok(MapBuilder::default()
            .destination_start(parts.next().expect("no dest"))
            .source_start(parts.next().expect("no start"))
            .length(parts.next().expect("no length"))
            .build()?)
    }
}

impl Map {
    fn includes_source(&self, source: usize) -> bool {
        source >= self.source_start && source < self.source_start + self.length
    }

    fn apply(&self, source: usize) -> Option<usize> {
        if self.includes_source(source) {
            let diff = source - self.source_start;
            Some(self.destination_start + diff)
        } else {
            None
        }
    }
}

#[derive(Debug, Builder, Clone)]
struct Mapper {
    source: String,
    destination: String,
    maps: Vec<Map>,
}

impl FromStr for Mapper {
    type Err = BoxE;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut mapper = MapperBuilder::default();
        mapper.maps(vec![]);
        for line in s.trim().lines() {
            if mapper.source.is_none() {
                let mut keys = line
                    .split_whitespace()
                    .next()
                    .expect("no source-dest")
                    .split("-to-");
                mapper.source(keys.next().expect("no source key").to_owned());
                mapper.destination(keys.next().expect("no destination key").to_owned());
            } else if let Some(maps) = &mut mapper.maps {
                maps.push(line.parse()?);
            }
        }
        Ok(mapper.build()?)
    }
}

impl Mapper {
    fn apply(&self, source: usize) -> usize {
        self.maps
            .iter()
            .find_map(|map| map.apply(source))
            .unwrap_or(source)
    }
}

#[derive(Debug, Builder, Clone)]
struct Seeds {
    source: String,
}

#[derive(Debug, Builder)]
struct Almanac {
    seeds: Seeds,
    mappers: Vec<Mapper>,
}

impl Almanac {
    fn map_source(&self, source: usize, start_key: &str, end_key: &str) -> usize {
        let mut current_key = start_key;
        let mut current_value = source;
        while current_key != end_key {
            let mapper = self
                .mappers
                .iter()
                .find(|mapper| mapper.source == current_key)
                .expect("could not find mapper");
            current_value = mapper.apply(current_value);
            current_key = &mapper.destination;
        }
        current_value
    }
}

impl FromStr for Almanac {
    type Err = BoxE;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut almanac = AlmanacBuilder::default();
        let mut parts = s.trim().split("\n\n");
        almanac.seeds(Seeds {
            source: parts.next().expect("seed line missing").to_string(),
        });
        almanac.mappers(vec![]);
        while let Some(mapper_section) = parts.next() {
            if let Some(mappers) = &mut almanac.mappers {
                mappers.push(mapper_section.parse()?);
            }
        }
        Ok(almanac.build()?)
    }
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let almanac: Almanac = input.parse()?;
    let parsed_time = start.elapsed();

    // algo
    let start = Instant::now();
    let answer = almanac
        .seeds
        .source
        .split(":")
        .nth(1)
        .expect("seeds missing")
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("failed to parse seed as number"))
        .map(|source| almanac.map_source(source, "seed", "location"))
        .min()
        .ok_or("failed to get min location")?;
    let algo_time = start.elapsed();

    // output
    println!("Day 5, part 1: {answer}");
    println!("\tparse: {parsed_time:?}");
    println!("\talgo: {algo_time:?}");
    Ok(answer)
}

fn part2(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let almanac: Almanac = input.parse()?;
    let parsed_time = start.elapsed();

    // algo
    let start = Instant::now();
    let answer = almanac
        .seeds
        .source
        .split(":")
        .nth(1)
        .expect("seeds missing")
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("failed to parse seed as number"))
        .tuples()
        .flat_map(|(start, length)| start..start + length)
        .take(3000)
        // Squeeze with rayon for brute force approach
        .par_bridge()
        .map(|source| almanac.map_source(source, "seed", "location"))
        .min()
        .ok_or("failed to get min location")?;
    let algo_time = start.elapsed();

    // output
    println!("Day 5, part 2: {answer}");
    println!("\tparse: {parsed_time:?}");
    println!("\talgo: {algo_time:?}");
    Ok(answer)
}

// TODO come back and revise for a faster solution
#[tokio::main]
async fn main() -> Result<()> {
    let input = utils::aoc::get_puzzle_input(5).await?;
    part1(input.clone())?;
    part2(input.clone())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part1(DATA.to_owned())?, 35);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part2(DATA.to_owned())?, 46);
        Ok(())
    }
}
