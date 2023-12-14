use aoc23::prelude::*;

#[derive(Debug, Clone)]
enum Tile {
    None,
    Round,
    Square,
}

fn get_columns<'a>(matrix: &'a mut Vec<Vec<Tile>>) -> Vec<Vec<&'a mut Tile>> {
    let mut columns = vec![];
    let cols = matrix.first().map_or(0, Vec::len);
    let rows = matrix.len();
    for col_idx in 0..cols {
        let mut column = vec![];
        for row_idx in 0..rows {
            column.push(&mut matrix[row_idx][col_idx]);
        }
        columns.push(column);
    }
    columns
}

fn tilt(rocks: &mut Vec<&mut Tile>) {
    for rock in rocks.iter_mut() {
        **rock = Tile::Round;
    }
}

fn main() -> Result<()> {
    // 10 x 10 grid of None
    let mut matrix: Vec<Vec<Tile>> = vec![vec![Tile::None; 10]; 10];
    println!("BEFORE:\n{:?}", matrix);
    let mut columns = get_columns(&mut matrix);
    tilt(columns.get_mut(0).unwrap());
    println!("AFTER:\n{:?}", matrix);
    Ok(())
}
