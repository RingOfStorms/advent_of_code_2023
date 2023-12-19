fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn find_lcm(numbers: &[u64]) -> u64 {
    numbers.iter().cloned().fold(1, |acc, num| lcm(acc, num))
}

pub fn orthogonal_u_bounded(root: (usize, usize)) -> Vec<((usize, usize), (isize, isize))> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .filter_map(|delta| {
            root.0
                .checked_add_signed(delta.0)
                .zip(root.1.checked_add_signed(delta.1))
                .map(|new_pos| (new_pos, delta))
        })
        .collect()
}
