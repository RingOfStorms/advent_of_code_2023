use aoc23::prelude::*;
use std::fmt::Display;

#[derive(Debug)]
enum Item {
    Digit(u32),
    Symbol,
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
}

impl Grid {
    fn get_item(&self, row: usize, col: usize) -> Option<&Item> {
        // println!("\t\t\tRow:{} Col:{} {:?}", row, col, self.rows.get(row));
        match self.rows.get(row) {
            Some(row) => row.get(col),
            None => None,
        }
    }

    fn get_adjacent_items(&self, row: usize, start: usize, end: usize) -> Vec<&Item> {
        // println!("\tget adj: r{} s{} e{}", row, start, end);
        let row_start = if row > 0 { row - 1 } else { 0 };
        let col_start = if start > 0 { start - 1 } else { 0 };
        let mut items: Vec<&Item> = vec![];
        for row_i in row_start..=row + 1 {
            for col_i in col_start..=end + 1 {
                if row_i != row || col_i < start || col_i > end {
                    let item = self.get_item(row_i, col_i);

                    // println!("\t\trow: {} col: {} item: {:?}", row_i, col_i, item);
                    if let Some(item) = item {
                        items.push(item);
                    }
                }
            }
        }
        return items;
    }

    fn print(&self) {
        for row in self.rows.iter() {
            for col in row {
                match col {
                    Item::Digit(_) => print!("d"),
                    Item::Symbol => print!("s"),
                    Item::None => print!("."),
                }
            }
            println!();
        }
    }
}

fn part1(input: String) -> Result<impl Display> {
    let mut grid = Grid { rows: vec![] };
    let mut numbers: Vec<Number> = vec![];
    for (row_index, line) in input.lines().enumerate() {
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
                    // println!("Ended number seq, hit dot after {:?} {}", num, char);
                    row.push(Item::None);
                } else {
                    // println!("Ended number seq, hit symbol after {:?} {}", num, char);
                    row.push(Item::Symbol)
                }
                if let Some(value) = num {
                    numbers.push(Number {
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
        grid.rows.push(row);
    }
    let grid = grid;
    let mut sum = 0;
    for number in numbers.iter() {
        let adjacent =
            grid.get_adjacent_items(number.row_index, number.char_start, number.char_end);
        let is_part_no = adjacent.iter().any(|item| !matches!(item, Item::None));
        // println!(
        //     "Checking number: {:?}, is part: {}\tadjacent: {:?}",
        //     number.value, is_part_no, adjacent
        // );
        if is_part_no {
            print!("{}, ", number.value);
            sum += number.value;
        }
    }
    // grid.print();
    Ok(sum)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Day 3");
    println!("=====");
    let input = utils::aoc::get_puzzle_input(3).await?;

    // ```
    // let input = "467..114..
    // ```
    //
    // ..._...... ..35..633. ......#... 617_...... .....+.58. ..592..... ......755.
    // ...$.*.... .664.598.." .to_owned();
    println!("part 1: {}", part1(input.clone())?);

    // println!("part 2: {}", part2(input.clone())?);
    Ok(())
}
