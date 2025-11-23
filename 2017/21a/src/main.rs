use adventutil::grid::{Grid, GridBounds};
use adventutil::{FromBits, Input};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Bitmap {
    size: usize, // 2..=4
    bits: u16,
}

impl Bitmap {
    fn transforms(self) -> [Bitmap; 8] {
        match self.size {
            2 => {
                /*
                 * [a][b]
                 * [c][d]
                 */
                let a = self.bits & 0b1000 != 0;
                let b = self.bits & 0b0100 != 0;
                let c = self.bits & 0b0010 != 0;
                let d = self.bits & 0b0001 != 0;
                [
                    [a, b, c, d], // 1
                    [a, c, b, d], // s
                    [b, d, a, c], // r
                    [b, a, d, c], // rs
                    [d, c, b, a], // r^2
                    [d, b, c, a], // r^2s
                    [c, a, d, b], // r^3
                    [c, d, a, b], // r^3s
                ]
                .map(|bs| Bitmap {
                    size: 2,
                    bits: u16::from_bits(bs),
                })
            }
            3 => {
                /*
                 * [a][b][c]
                 * [d][e][f]
                 * [g][h][i]
                 */
                let a = self.bits & 0b100_000_000 != 0;
                let b = self.bits & 0b010_000_000 != 0;
                let c = self.bits & 0b001_000_000 != 0;
                let d = self.bits & 0b000_100_000 != 0;
                let e = self.bits & 0b000_010_000 != 0;
                let f = self.bits & 0b000_001_000 != 0;
                let g = self.bits & 0b000_000_100 != 0;
                let h = self.bits & 0b000_000_010 != 0;
                let i = self.bits & 0b000_000_001 != 0;
                [
                    [a, b, c, d, e, f, g, h, i], // 1
                    [a, d, g, b, e, h, c, f, i], // s
                    [c, f, i, b, e, h, a, d, g], // r
                    [c, b, a, f, e, d, i, h, g], // rs
                    [i, h, g, f, e, d, c, b, a], // r^2
                    [i, f, c, h, e, b, g, d, a], // r^2s
                    [g, d, a, h, e, b, i, f, c], // r^3
                    [g, h, i, d, e, f, a, b, c], // r^3s
                ]
                .map(|bs| Bitmap {
                    size: 3,
                    bits: u16::from_bits(bs),
                })
            }
            sz => panic!("Bitmap::transform() not supported for size {sz}"),
        }
    }

    fn extract(pattern: &Grid<bool>, (uly, ulx): (usize, usize), size: usize) -> Bitmap {
        let mut bits = 0u16;
        for y in uly..(uly + size) {
            for x in ulx..(ulx + size) {
                bits <<= 1;
                if pattern[(y, x)] {
                    bits += 1;
                }
            }
        }
        Bitmap { size, bits }
    }

    fn apply(self, pattern: &mut Grid<bool>, (uly, ulx): (usize, usize)) {
        let mut mask = 1 << (self.size * self.size - 1);
        for y in uly..(uly + self.size) {
            for x in ulx..(ulx + self.size) {
                pattern[(y, x)] = (self.bits & mask) != 0;
                mask >>= 1;
            }
        }
    }
}

impl std::str::FromStr for Bitmap {
    type Err = ParseBitmapError;

    fn from_str(s: &str) -> Result<Bitmap, ParseBitmapError> {
        let mut size = None;
        let mut row_qty = 0;
        let mut bits = 0u16;
        for row in s.split('/') {
            row_qty += 1;
            if let Some(sz) = size {
                if sz != row.len() {
                    return Err(ParseBitmapError::Ragged(sz, row.len()));
                }
            } else {
                size = Some(row.len());
            }
            for c in row.chars() {
                bits <<= 1;
                match c {
                    '#' => bits += 1,
                    '.' => (),
                    c => return Err(ParseBitmapError::Unexpected(c)),
                }
            }
        }
        let size = size.ok_or(ParseBitmapError::Empty)?;
        if size != row_qty {
            Err(ParseBitmapError::NotSquare {
                width: size,
                height: row_qty,
            })
        } else if !(2..=4).contains(&size) {
            Err(ParseBitmapError::BadSize(size))
        } else {
            Ok(Bitmap { size, bits })
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
enum ParseBitmapError {
    #[error("empty bitmap pattern")]
    Empty,
    #[error("unexpected character in pattern: {0:?}")]
    Unexpected(char),
    #[error("ragged pattern: row of width {0} followed by row of length {1}")]
    Ragged(usize, usize),
    #[error("pattern is not square: width={width}, height={height}")]
    NotSquare { width: usize, height: usize },
    #[error("unsupported pattern size: {0}")]
    BadSize(usize),
}

fn solve(input: Input, iterations: usize) -> usize {
    let mut enhancements = HashMap::new();
    for ln in input.lines() {
        let (before, after) = ln.trim().split_once(" => ").unwrap();
        let before = before.parse::<Bitmap>().unwrap();
        let after = after.parse::<Bitmap>().unwrap();
        enhancements.insert(before, after);
    }
    let mut pattern = Grid::try_from(vec![
        vec![false, true, false], // .#.
        vec![false, false, true], // ..#
        vec![true, true, true],   // ###
    ])
    .unwrap();
    for _ in 0..iterations {
        let (old_sq_size, new_sq_size, sub_size) = if pattern.width() % 2 == 0 {
            (2, 3, pattern.width() / 2)
        } else if pattern.width() % 3 == 0 {
            (3, 4, pattern.width() / 3)
        } else {
            panic!("Pattern has unexpected width {}", pattern.width());
        };
        let mut new_pattern = Grid::filled(
            GridBounds {
                width: sub_size * new_sq_size,
                height: sub_size * new_sq_size,
            },
            false,
        );
        for sqy in 0..sub_size {
            for sqx in 0..sub_size {
                let sq = Bitmap::extract(
                    &pattern,
                    (sqy * old_sq_size, sqx * old_sq_size),
                    old_sq_size,
                );
                let newsq = sq
                    .transforms()
                    .into_iter()
                    .find_map(|xform| enhancements.get(&xform).copied())
                    .unwrap();
                newsq.apply(&mut new_pattern, (sqy * new_sq_size, sqx * new_sq_size));
            }
        }
        pattern = new_pattern;
    }
    pattern.into_true_coords().count()
}

fn main() {
    println!("{}", solve(Input::from_env(), 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "../.# => ##./#../...\n",
            ".#./..#/### => #..#/..../..../#..#\n",
        ));
        assert_eq!(solve(input, 2), 12);
    }
}
