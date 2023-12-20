use aoc23::prelude::*;
use derive_builder::Builder;
use grid::Grid;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    time::Instant,
};

static DAY: u8 = 17;
type Pos = (usize, usize);

fn delta_pos(from: &Pos, to: &Pos) -> (isize, isize) {
    return (
        to.0 as isize - from.0 as isize,
        to.1 as isize - from.1 as isize,
    );
}

fn grid(input: &str) -> Grid<usize> {
    let mut grid = Grid::new(0, 0);
    for line in input.lines() {
        let weights = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).expect("invalid weight") as usize)
            .collect_vec();
        grid.push_row(weights);
    }
    grid
}

struct Graph {
    edges: HashMap<Pos, HashMap<Pos, usize>>,
}

impl Graph {
    fn new_from_weighted_grid(weights: &Grid<usize>) -> Self {
        let mut edges = HashMap::new();
        for r in 0..weights.rows() {
            for c in 0..weights.cols() {
                let pos = (r, c);
                for (n_pos, n_weight) in weights.neighbors_orthogonal(pos) {
                    edges
                        .entry(pos)
                        .or_insert_with(|| HashMap::new())
                        .insert(n_pos, *n_weight);
                }
            }
        }
        Self { edges }
    }

    fn shortest_path_with_movement_limit(
        &self,
        start: Pos,
        end: Pos,
        move_limit_min: usize,
        move_limit_max: usize,
    ) -> usize {
        // TODO min move limit?
        let valid_move_in_limit =
            |delta: (isize, isize), since: &Pos, predecessors: &HashMap<Pos, Option<Pos>>| {
                let mut current = since;
                let mut previous = vec![];
                for i in 0..move_limit_max {
                    if let Some(Some(prev)) = predecessors.get(current) {
                        let previous_movement = delta_pos(prev, current);
                        if previous_movement != delta {
                            // movement is different we are okay
                            break;
                        } else if i == (move_limit_max - 1) {
                            // movement is the same and we are not at the movement limit, break out of this
                            // path, not allowed
                            previous.push(*current);
                            return (false, previous);
                        } else {
                            // check the prior element
                            previous.push(*current);
                            current = prev;
                        }
                    } else {
                        // no path we are good
                        break;
                    }
                }
                (true, previous)
            };

        // based on https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
        let mut queue: HashMap<Pos, usize> = HashMap::new();
        let mut distances: HashMap<Pos, usize> = HashMap::new();
        let mut predecessors: HashMap<Pos, Option<Pos>> = HashMap::new();
        self.edges.iter().for_each(|(pos, _)| {
            distances.insert(*pos, usize::MAX);
            predecessors.insert(*pos, None);
            queue.insert(*pos, 0);
        });
        distances.insert(start, 0);

        // go from start node and check all other nodes
        while queue.len() > 0 {
            let ((current, backtrack), distance) = queue
                .iter()
                .map(|item| (item, distances.get(item.0).unwrap()))
                .sorted_by(|(_, distance_a), (_, distance_b)| distance_a.cmp(distance_b))
                .next()
                .unwrap();
            let current = *current;
            let distance = *distance;
            queue.remove(&current);

            // Can terminate dijkstra early if we hit our end value.
            if current == end {
                break;
            }
            self.edges
                .get(&current)
                .unwrap()
                .iter()
                .filter(|(neighbor, n_dist)| {
                    let movement = delta_pos(&current, neighbor);
                    let in_queue = queue.contains_key(neighbor);
                    let is_not_backwards = {
                        let pred = predecessors.get(&current).unwrap().unwrap();
                        let pred_movement = delta_pos(&pred, &current);

                        // not 180 degree turn
                        !(pred_movement.0 * -1 == movement.0 && pred_movement.1 * -1 == movement.1)
                    };
                    let (valid_movement, _) =
                        valid_move_in_limit(movement, &current, &predecessors);
                    if in_queue && is_not_backwards && !valid_movement {
                        // if not valid movement in limits then backtrack and try those again ... todo
                        // TODO Do this thing?
                    }
                    return in_queue && valid_movement;
                })
                .for_each(|(neighbor, n_dist)| {
                    let current_n_dist = *distances.get(neighbor).unwrap();
                    let alt = distance + n_dist;
                    if alt < current_n_dist {
                        distances.insert(*neighbor, alt);
                        predecessors.insert(*neighbor, Some(current.clone()));
                    }
                });
        }
        *distances.get(&end).unwrap()
    }
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let weights = grid(&input);
    let graph = Graph::new_from_weighted_grid(&weights);
    let root = (0, 0);
    let goal = (weights.rows() - 1, weights.cols() - 1);
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let answer = graph.shortest_path_with_movement_limit(root, goal, 1, 3);
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
    let weights = grid(&input);
    let graph = Graph::new_from_weighted_grid(&weights);
    let root = (0, 0);
    let goal = (weights.rows() - 1, weights.cols() - 1);
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
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
                    .to_owned(),
            )?,
            102
        );
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part_2() -> Result<()> {
        assert_eq!(
            part2(
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
                    .to_owned(),
            )?,
            0
        );
        Ok(())
    }
}
