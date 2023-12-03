use aoc23::prelude::*;
use std::str::FromStr;

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
                        // println!("Ended number seq, hit dot after {:?} {}", num, char);
                        row.push(Item::None);
                    } else {
                        // println!("Ended number seq, hit symbol after {:?} {}", num, char);
                        row.push(Item::Symbol)
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
        let is_part_no = adjacent.iter().any(|item| matches!(item, Item::Symbol));

        // ```
        // println!(
        //     "Checking number: {:?}, is part: {}\tadjacent: {:?}",
        //     number.value, is_part_no, adjacent
        // );
        // ```
        if is_part_no {
            // print!("{}, ", number.value);
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
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_owned();
    let part_1 = part1(input.clone())?;
    println!("part 1: {} == 520019 !! {}", part_1, part_1 == 520019);

    // println!("part 2: {}", part2(input.clone())?);
    Ok(())
}
