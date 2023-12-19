use aoc23::prelude::*;
use itertools::Itertools;
use regex::Regex;
use static_init::dynamic;
use std::{collections::HashMap, time::Instant};

static DAY: u8 = 19;
#[dynamic]
static RE_PARSE_PART: Regex = Regex::new(r"x=(?<x>\d+).*m=(?<m>\w+).*a=(?<a>\d+).*s=(?<s>\d+)")
    .expect("re_parse_part invalid");
#[dynamic]
static RE_PARSE_WORKFLOW: Regex =
    Regex::new(r"^(?<name>\w+)\{(?<ops>.*)}").expect("re_parse_workflow invalid");

#[derive(Debug, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new_from_line(line: &str) -> Self {
        let re = RE_PARSE_PART.captures(line).expect("part does not match");
        Self {
            x: re.name("x").unwrap().as_str().parse().unwrap(),
            m: re.name("m").unwrap().as_str().parse().unwrap(),
            a: re.name("a").unwrap().as_str().parse().unwrap(),
            s: re.name("s").unwrap().as_str().parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
enum PartCategory {
    X,
    M,
    A,
    S,
}

impl PartCategory {
    fn new_from_char(c: &char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("wee woo"),
        }
    }

    fn value_of(&self, part: &Part) -> usize {
        match self {
            PartCategory::X => part.x,
            PartCategory::M => part.m,
            PartCategory::A => part.a,
            PartCategory::S => part.s,
        }
    }
}

#[derive(Debug, Clone)]
enum BasicFlowOp {
    Accept,
    Reject,
    Route(String),
}

impl BasicFlowOp {
    fn new_from_line(line: &str) -> Self {
        match line {
            "A" => Self::Accept,
            "R" => Self::Reject,
            route => Self::Route(route.to_owned()),
        }
    }

    fn as_flow_op(&self) -> FlowOp {
        match self {
            BasicFlowOp::Accept => FlowOp::Accept,
            BasicFlowOp::Reject => FlowOp::Reject,
            BasicFlowOp::Route(to) => FlowOp::Route(to.to_owned()),
        }
    }

    fn part2_to(&self) -> String {
        match self {
            BasicFlowOp::Accept => "A".to_owned(),
            BasicFlowOp::Reject => "R".to_owned(),
            BasicFlowOp::Route(to) => to.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
enum FlowOp {
    LessThan(PartCategory, usize, BasicFlowOp),
    GreaterThan(PartCategory, usize, BasicFlowOp),
    Accept,
    Reject,
    Route(String),
}

impl FlowOp {
    fn new_from_line(line: &str) -> Self {
        let split = line.split([':', ',', '>', '<']).collect_vec();
        if split.len() == 1 {
            BasicFlowOp::new_from_line(split.get(0).unwrap()).as_flow_op()
        } else {
            if line.contains('>') {
                Self::GreaterThan(
                    PartCategory::new_from_char(&split.get(0).unwrap().chars().next().unwrap()),
                    split.get(1).unwrap().parse().unwrap(),
                    BasicFlowOp::new_from_line(split.get(2).unwrap()),
                )
            } else {
                Self::LessThan(
                    PartCategory::new_from_char(&split.get(0).unwrap().chars().next().unwrap()),
                    split.get(1).unwrap().parse().unwrap(),
                    BasicFlowOp::new_from_line(split.get(2).unwrap()),
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    flow: Vec<FlowOp>,
}

impl Workflow {
    fn new_from_line(line: &str) -> Self {
        let re = RE_PARSE_WORKFLOW
            .captures(line)
            .expect("workflow does not match");
        Self {
            name: re.name("name").unwrap().as_str().to_owned(),
            flow: re
                .name("ops")
                .unwrap()
                .as_str()
                .split(',')
                .map(FlowOp::new_from_line)
                .collect(),
        }
    }
}

fn parse_system(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let split = input.split("\n\n").collect_vec();
    (
        split
            .get(0)
            .unwrap()
            .lines()
            .map(Workflow::new_from_line)
            .map(|w| (w.name.clone(), w))
            .collect(),
        split
            .get(1)
            .unwrap()
            .lines()
            .map(Part::new_from_line)
            .collect(),
    )
}

fn check_part_accepted(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current_workflow = workflows.get("in");
    'flows: while let Some(workflow) = current_workflow {
        current_workflow = None;
        for flow_op in workflow.flow.iter() {
            let basic_op = match flow_op {
                FlowOp::LessThan(cat, num, bop) => {
                    if cat.value_of(part) < *num {
                        bop.as_flow_op()
                    } else {
                        continue;
                    }
                }
                FlowOp::GreaterThan(cat, num, bop) => {
                    if cat.value_of(part) > *num {
                        bop.as_flow_op()
                    } else {
                        continue;
                    }
                }
                op => op.clone(),
            };
            match basic_op {
                FlowOp::Accept => return true,
                FlowOp::Reject => return false,
                FlowOp::Route(to) => {
                    current_workflow = workflows.get(&to);
                    continue 'flows;
                }
                _ => panic!("not basic"),
            }
        }
    }
    panic!("why are we here?")
}

fn part1(input: String) -> Result<usize> {
    // parse
    let start = Instant::now();
    let (workflows, parts) = parse_system(&input);
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();
    let answer = parts
        .iter()
        .filter_map(|p| check_part_accepted(p, &workflows).then(|| p.x + p.m + p.a + p.s))
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
    let (workflows, _) = parse_system(&input);
    let parsed_time = start.elapsed();

    // algo
    let a_start = Instant::now();

    // LOL yeah right... no no no to brute force on this one
    //
    // ```
    // let bar = ProgressBar::new(4000 * 4000 * 4000 * 4000);
    // let answer = iproduct!(1..=4000usize, 1..=4000usize, 1..=4000usize, 1..=4000usize)
    //     .par_bridge()
    //     .map(|(x, m, a, s)| {
    //         bar.inc(1);
    //         Part { x, m, a, s }
    //     })
    //     .filter(|part| check_part_accepted(part, &workflows))
    //     .count();
    // bar.finish();
    // ```
    //
    // Honestly copied someone else's approach for this one...
    // https://github.com/klimesf/advent-of-code/blob/master/src/y2023/day19.rs
    let mut stack: Vec<(
        (usize, usize),
        (usize, usize),
        (usize, usize),
        (usize, usize),
        String,
        usize,
    )> = vec![(
        (1, 4000),
        (1, 4000),
        (1, 4000),
        (1, 4000),
        "in".to_owned(),
        0,
    )];
    let mut accepted: Vec<(
        (usize, usize),
        (usize, usize),
        (usize, usize),
        (usize, usize),
    )> = vec![];
    while let Some(range) = stack.pop() {
        let (x, m, a, s, wf_key, rule_key) = range;
        if wf_key == "A" {
            accepted.push((x, m, a, s));
            continue;
        } else if wf_key == "R" {
            continue;
        }

        // Invalid bounds check
        if x.0 > x.1 || m.0 > m.1 || a.0 > a.1 || s.0 > s.1 {
            continue;
        }
        let workflow = workflows.get(&wf_key).unwrap();
        let flow_op = &workflow.flow[rule_key];
        match flow_op {
            FlowOp::Accept => {
                accepted.push((x, m, a, s));
                continue;
            }
            FlowOp::Reject => {
                continue;
            }
            FlowOp::Route(new_wf_key) => {
                stack.push((x, m, a, s, new_wf_key.to_owned(), 0));
                continue;
            }
            FlowOp::GreaterThan(category, number, flow_op) => match category {
                PartCategory::X => {
                    stack.push(((number + 1, x.1), m, a, s, flow_op.part2_to(), 0));
                    stack.push(((x.0, *number), m, a, s, wf_key, rule_key + 1));
                }
                PartCategory::M => {
                    stack.push((x, (number + 1, m.1), a, s, flow_op.part2_to(), 0));
                    stack.push((x, (m.0, *number), a, s, wf_key, rule_key + 1));
                }
                PartCategory::A => {
                    stack.push((x, m, (number + 1, a.1), s, flow_op.part2_to(), 0));
                    stack.push((x, m, (a.0, *number), s, wf_key, rule_key + 1));
                }
                PartCategory::S => {
                    stack.push((x, m, a, (number + 1, s.1), flow_op.part2_to(), 0));
                    stack.push((x, m, a, (s.0, *number), wf_key, rule_key + 1));
                }
            },
            FlowOp::LessThan(category, number, flow_op) => match category {
                PartCategory::X => {
                    stack.push(((x.0, number - 1), m, a, s, flow_op.part2_to(), 0));
                    stack.push(((*number, x.1), m, a, s, wf_key, rule_key + 1));
                }
                PartCategory::M => {
                    stack.push((x, (m.0, number - 1), a, s, flow_op.part2_to(), 0));
                    stack.push((x, (*number, m.1), a, s, wf_key, rule_key + 1));
                }
                PartCategory::A => {
                    stack.push((x, m, (a.0, number - 1), s, flow_op.part2_to(), 0));
                    stack.push((x, m, (*number, a.1), s, wf_key, rule_key + 1));
                }
                PartCategory::S => {
                    stack.push((x, m, a, (s.0, number - 1), flow_op.part2_to(), 0));
                    stack.push((x, m, a, (*number, s.1), wf_key, rule_key + 1));
                }
            },
        }
    }
    let answer = accepted
        .iter()
        .map(|(x, m, a, s)| (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1))
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
    fn test_part_1() -> Result<()> {
        assert_eq!(
            part1(
                "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"
                .to_owned(),
            )?,
            19114
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(
            part2(
                "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"
                .to_owned(),
            )?,
            167409079868000
        );
        Ok(())
    }
}
