use super::math::orthogonal_u_bounded;
use grid::Grid;
use std::collections::HashMap;

pub trait GridExtras<T> {
    fn neighbors_all(&self, pos: (usize, usize)) -> HashMap<(usize, usize), &T>;
    fn neighbors_orthogonal(&self, pos: (usize, usize)) -> HashMap<(usize, usize), &T>;
}

impl<T> GridExtras<T> for Grid<T> {
    fn neighbors_all(&self, pos: (usize, usize)) -> HashMap<(usize, usize), &T> {
        let mut n = HashMap::new();
        for r in -1..=1 {
            for c in -1..=1 {
                // skip self
                if r == 0 && c == 0 {
                    continue;
                }
                if let Some(neighbor) = pos.0.checked_add_signed(r).zip(pos.1.checked_add_signed(c))
                {
                    if let Some(t) = self.get(neighbor.0, neighbor.1) {
                        n.insert(neighbor, t);
                    }
                }
            }
        }
        n
    }

    fn neighbors_orthogonal(&self, pos: (usize, usize)) -> HashMap<(usize, usize), &T> {
        orthogonal_u_bounded(pos)
            .into_iter()
            .filter_map(|(pos, _delta)| self.get(pos.0, pos.1).map(|t| (pos, t)))
            .collect()
    }
}
