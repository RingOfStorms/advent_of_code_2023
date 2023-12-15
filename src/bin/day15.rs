use aoc23::prelude::*;
use derive_builder::Builder;
use itertools::Itertools;
use rayon::prelude::*;
use std::{str::FromStr, time::Instant};

static DAY: u8 = 15;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Seq {
    chars: Vec<char>,
}

impl Seq {
    fn new(s: &str) -> Result<Self> {
        Ok(Self {
            chars: s.trim().chars().filter(|c| !c.is_whitespace()).collect(),
        })
    }

    fn hash(&self) -> usize {
        self.chars.iter().fold(0, |acc, char| {
            let mut value = acc;
            value += *char as usize;
            value *= 17;
            value = value % 256;
            value
        })
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Remove,
    // focal length
    Insert(usize),
}

#[derive(Debug, Clone)]
struct LensOp {
    label: Seq,
    box_index: usize,
    operation: Operation,
}

impl LensOp {
    fn new(value: &Seq) -> Self {
        let label = Seq {
            chars: value
                .chars
                .iter()
                .take_while(|c| c != &&'=' && c != &&'-')
                .map(|c| *c)
                .collect(),
        };
        let box_index = label.hash();
        let mut op = value.chars.iter().skip(label.chars.len());
        Self {
            label,
            box_index,
            operation: match op.next().unwrap() {
                '-' => Operation::Remove,
                '=' => Operation::Insert(op.next().unwrap().to_digit(10).unwrap() as usize),
                o => panic!("Unknown op {o}"),
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
struct LensBox {
    lenses: Vec<LensOp>,
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let sequences: Vec<Seq> = input.split(',').map(|seq| Seq::new(seq)).try_collect()?;
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let answer = sequences.par_iter().map(Seq::hash).sum();
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
    let lens_ops: Vec<LensOp> = input
        .split(',')
        .map(|seq| Seq::new(seq).unwrap())
        .map(|s| LensOp::new(&s))
        .collect();
    let mut lens_boxes: Vec<LensBox> = std::iter::repeat_with(|| LensBox::default())
        .take(256)
        .collect();
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    for lens_op in lens_ops {
        if let Some(lens_box) = lens_boxes.get_mut(lens_op.box_index) {
            match lens_op.operation {
                Operation::Remove => lens_box.lenses.retain(|lop| lop.label != lens_op.label),
                Operation::Insert(_) => {
                    if let Some(existing) = lens_box
                        .lenses
                        .iter_mut()
                        .find(|lop| lop.label == lens_op.label)
                    {
                        *existing = lens_op;
                    } else {
                        lens_box.lenses.push(lens_op);
                    }
                }
            }
        }
    }
    let answer = lens_boxes
        .iter()
        .enumerate()
        .map(|(box_idx, lens_box)| {
            lens_box
                .lenses
                .iter()
                .enumerate()
                .map(|(lens_idx, lens)| {
                    if let Operation::Insert(focal_length) = lens.operation {
                        (box_idx + 1) * (lens_idx + 1) * focal_length
                    } else {
                        panic!("How did a removal lens get in there?");
                    }
                })
                .sum::<usize>()
        })
        .sum();
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
    fn test_part_1_a() -> Result<()> {
        assert_eq!(part1("HASH".to_owned())?, 52);
        Ok(())
    }

    #[test]
    fn test_part_1_b() -> Result<()> {
        assert_eq!(
            part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_owned())?,
            1320
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(
            part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_owned())?,
            145
        );
        Ok(())
    }
}
