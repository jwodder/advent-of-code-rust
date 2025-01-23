use adventutil::closure::one2many_closure;
use adventutil::grid::{Direction, Grid, GridBounds};
use adventutil::Input;

fn knot_hash(key: &[u8]) -> [u8; 16] {
    let mut values = (0u8..=255).collect::<Vec<_>>();
    let mut pos = 0;
    for (skip, length) in std::iter::repeat(key.iter().copied().chain([17, 31, 73, 47, 23]))
        .take(64)
        .flatten()
        .enumerate()
    {
        let length = usize::from(length);
        for i in 0..(length / 2) {
            values.swap((pos + i) % 256, (pos + length - 1 - i) % 256);
        }
        pos = (pos + length + skip) % 256;
    }
    let mut hash = [0u8; 16];
    for (block, cell) in values
        .chunks(16)
        .map(|chunk| chunk.iter().copied().fold(0, |a, b| a ^ b))
        .zip(hash.iter_mut())
    {
        *cell = block;
    }
    hash
}

fn solve(input: Input) -> u32 {
    let keybase = input.read();
    let keybase = keybase.trim();
    let bounds = GridBounds {
        height: 128,
        width: 128,
    };
    let mut grid = Grid::filled(bounds, false);
    for y in 0..128 {
        let hash = knot_hash(format!("{keybase}-{y}").as_bytes());
        for (i, c) in hash.into_iter().enumerate() {
            for b in 0..8 {
                if c & (1 << (7 - b)) != 0 {
                    grid[(y, i * 8 + b)] = true;
                }
            }
        }
    }
    let mut regions = 0;
    while let Some(start) = grid.enumerate().find_map(|(c, b)| b.then_some(c)) {
        let region = one2many_closure(start, |c| {
            Direction::cardinals()
                .filter_map(move |d| bounds.move_in(c, d))
                .filter(|&c| grid[c])
        });
        for c in region {
            grid[c] = false;
        }
        regions += 1;
    }
    regions
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from("flqrgnkx");
        assert_eq!(solve(input), 1242);
    }
}
