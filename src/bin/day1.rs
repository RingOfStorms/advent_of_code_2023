use aoc23::prelude::*;

fn part1(input: String) -> Result<usize> {
    let mut sum = 0;
    for line in input.lines().into_iter() {
        let chars = line.chars();
        let first_num: usize = chars
            .clone()
            .into_iter()
            .skip_while(|c| !c.is_digit(10))
            .take(1)
            // This would get continuous digits: .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse()?;
        let last_num: usize = chars
            .into_iter()
            .rev()
            .skip_while(|c| !c.is_digit(10))
            .take(1)
            .collect::<String>()
            .parse()?;
        let value = first_num * 10 + last_num;
        sum += value;
    }

    // println!("Answer: {}", sum);
    Ok(sum)
}

fn num_to_word(num: u32) -> Option<&'static str> {
    match num {
        1 => Some("one"),
        2 => Some("two"),
        3 => Some("three"),
        4 => Some("four"),
        5 => Some("five"),
        6 => Some("six"),
        7 => Some("seven"),
        8 => Some("eight"),
        9 => Some("nine"),
        _ => None,
    }
}

fn first_occurrence(str: &str, reversed: bool) -> Option<u32> {
    if reversed {
        for (i, char) in str.chars().rev().enumerate() {
            let digit = char.to_digit(10).or_else(|| {
                for num in 1..=9 {
                    if let Some(num_word) = num_to_word(num) {
                        if i + 1 >= num_word.len()
                            && &str[str.len() - i - 1..str.len()][..num_word.len()] == num_word
                        {
                            return Some(num);
                        }
                    }
                }
                None
            });
            if digit.is_some() {
                return digit;
            }
        }
    } else {
        for (i, char) in str.chars().enumerate() {
            let digit = char.to_digit(10).or_else(|| {
                for num in 1..=9 {
                    if let Some(num_word) = num_to_word(num) {
                        if i + 1 >= num_word.len() && &str[i + 1 - num_word.len()..=i] == num_word {
                            return Some(num);
                        }
                    }
                }
                None
            });
            if digit.is_some() {
                return digit;
            }
        }
    }
    None
}

fn part_2(input: String) -> Result<u32> {
    let mut sum = 0;
    for line in input.lines().into_iter() {
        let first_num = first_occurrence(line, false).ok_or("No number found in line")?;
        let last_num = first_occurrence(line, true).ok_or("No number found in line reversed")?;
        let value = first_num * 10 + last_num;
        sum += value;
        println!("Line {line}: a:{first_num}|b:{last_num} = {value} ++ {sum}");
    }
    Ok(sum)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Day 1");
    println!("=====");
    let input = utils::aoc::get_puzzle_input(1).await?;
    println!("part 1: {}", part1(input.clone())?);
    println!("part 2: {}", part_2(input.clone())?);
    Ok(())
}
