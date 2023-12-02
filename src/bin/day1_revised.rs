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

fn part_2(input: String) -> Result<u32> {
    let numbers: [(&'static str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut sum = 0;
    for line in input.lines().into_iter() {
        let mut chars = line.chars();
        let mut peek_index = 0;
        let first_num = loop {
            // I wanted this to work but peek advances the iterator anyways despite saying it
            // doesn't. I don't understand
            //
            // ```
            // if let Some(Some(peek_digit)) = chars.by_ref().peekable().peek().map(|c| c.to_digit(10))
            // ```
            if let Some(Some(peek_digit)) = line.chars().nth(peek_index).map(|c| c.to_digit(10)) {
                break peek_digit;
            }
            if let Some((_, digit)) = numbers.iter().find(|num| chars.as_str().starts_with(num.0)) {
                break *digit;
            }
            if chars.next().is_none() {
                break 0;
            }
            peek_index += 1;
        };
        let last_num = loop {
            if let Some((_, digit)) = numbers.iter().find(|num| chars.as_str().ends_with(num.0)) {
                break *digit;
            }
            if let Some(last_char) = chars.next_back() {
                if let Some(digit) = last_char.to_digit(10) {
                    break digit;
                }
            } else {
                break first_num;
            }
        };
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
