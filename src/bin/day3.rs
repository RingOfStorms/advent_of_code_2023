use aoc23::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
enum Item {
    Digit(u32),
    Symbol(bool),
    None,
}

#[derive(Debug)]
struct Number {
    pub value: usize,
    pub row_index: usize,
    pub char_start: usize,
    pub char_end: usize,
}

struct Grid {
    pub rows: Vec<Vec<Item>>,
    pub numbers: Vec<Number>,
}

impl Grid {
    fn get_item(&self, row: usize, col: usize) -> Option<&Item> {
        // println!("\t\t\tRow:{} Col:{} {:?}", row, col, self.rows.get(row));
        match self.rows.get(row) {
            Some(row) => row.get(col),
            None => None,
        }
    }

    fn get_adjacent_items(
        &self,
        row: usize,
        start: usize,
        end: usize,
    ) -> Vec<(&Item, usize, usize)> {
        // println!("\tget adj: r{} s{} e{}", row, start, end);
        let row_start = if row > 0 { row - 1 } else { 0 };
        let col_start = if start > 0 { start - 1 } else { 0 };
        let mut items: Vec<(&Item, usize, usize)> = vec![];
        for row_i in row_start..=row + 1 {
            for col_i in col_start..=end + 1 {
                if row_i != row || col_i < start || col_i > end {
                    let item = self.get_item(row_i, col_i);

                    // println!("\t\trow: {} col: {} item: {:?}", row_i, col_i, item);
                    if let Some(item) = item {
                        items.push((item, row_i, col_i));
                    }
                }
            }
        }
        return items;
    }

    fn get_adjacent_numbers(&self, row: usize, col: usize) -> Vec<&Number> {
        let mut unique_numbers: Vec<&Number> = vec![];
        for item in self.get_adjacent_items(row, col, col) {
            match item {
                (Item::Digit(_), r, c) => {
                    if let Some(number_at) = self
                        .numbers
                        .iter()
                        .find(|n| n.row_index == r && c >= n.char_start && c <= n.char_end)
                    {
                        if unique_numbers.iter().all(|n| n.value != number_at.value) {
                            unique_numbers.push(number_at);
                        }
                    }
                }
                _ => {}
            }
        }
        unique_numbers
    }
}

impl FromStr for Grid {
    type Err = BoxE;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut grid = Grid {
            rows: vec![],
            numbers: vec![],
        };
        for (row_index, line) in s.lines().enumerate() {
            let mut row: Vec<Item> = vec![];
            let mut num: Option<usize> = None;
            let mut char_start = 0;
            for (char_index, char) in line.trim().chars().enumerate() {
                if let Some(digit) = char.to_digit(10) {
                    row.push(Item::Digit(digit));
                    num = if let Some(existing_num) = num {
                        Some(existing_num * 10 + digit as usize)
                    } else {
                        char_start = char_index;
                        Some(digit as usize)
                    };
                } else {
                    if char == '.' {
                        row.push(Item::None);
                    } else {
                        row.push(Item::Symbol(char == '*'))
                    }
                    if let Some(value) = num {
                        grid.numbers.push(Number {
                            value,
                            row_index,
                            char_start,
                            char_end: char_index - 1,
                        });
                        num = None;
                        char_start = 0;
                    }
                }
            }

            // This killed me, I forgot to add a check after the end of the line to see if I
            // was still "scanning" for a number and adding it. There was 1 single number not
            // getting parsed that was on the right edge of the grid and I could not figure it
            // out for a while.
            if let Some(value) = num {
                grid.numbers.push(Number {
                    value,
                    row_index,
                    char_start,
                    char_end: line.len() - 1,
                });
            }
            grid.rows.push(row);
        }
        Ok(grid)
    }
}

fn part1(input: String) -> Result<usize> {
    let grid: Grid = input.parse()?;
    let mut sum = 0;
    for number in grid.numbers.iter() {
        let adjacent =
            grid.get_adjacent_items(number.row_index, number.char_start, number.char_end);
        let is_part_no = adjacent
            .iter()
            .any(|item| matches!(item, (Item::Symbol(_), _, _)));
        if is_part_no {
            sum += number.value;
        }
    }
    Ok(sum)
}

fn part2(input: String) -> Result<usize> {
    let grid: Grid = input.parse()?;
    let mut sum = 0;
    let mut used: Vec<(usize, usize)> = vec![];
    for number in grid.numbers.iter() {
        let is_used = used
            .iter()
            .any(|(row, char_start)| number.row_index == *row && number.char_start == *char_start);
        if !is_used {
            let adjacent =
                grid.get_adjacent_items(number.row_index, number.char_start, number.char_end);
            if let Some((_, row, col)) = adjacent
                .iter()
                .find(|item| matches!(item.0, Item::Symbol(true)))
            {
                let gear_nums = grid.get_adjacent_numbers(*row, *col);
                if gear_nums.len() >= 2 {
                    // this is lazy.. adds duplicates but w/e
                    let power = gear_nums.iter().fold(1, |pow, gear| pow * gear.value);
                    gear_nums
                        .iter()
                        .for_each(|g| used.push((g.row_index, g.char_start)));
                    sum += power;
                }
            }
        }
    }
    Ok(sum)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Day 3");
    println!("=====");
    let input = utils::aoc::get_puzzle_input(3).await?;
    let part_1 = part1(input.clone())?;
    println!("part 1: {}", part_1);
    println!("part 2: {}", part2(input.clone())?);
    Ok(())
}
