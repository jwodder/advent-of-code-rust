use adventutil::grid::{Coords, Grid, GridBounds};
use adventutil::Input;
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

struct FuelGrid {
    serial_no: isize,
}

impl FuelGrid {
    fn max_square(&self) -> String {
        let bounds = GridBounds::new(300, 300);
        let grid = Grid::<isize>::from_fn(bounds, |c| power_level(c, self.serial_no));
        let (x, y) = (1..=298)
            .cartesian_product(1..=298)
            .max_by_key(|&(x, y)| {
                (x..(x + 3))
                    .cartesian_product(y..(y + 3))
                    .map(|(i, j)| grid[(j - 1, i - 1)])
                    .sum::<isize>()
            })
            .unwrap();
        format!("{},{}", x, y)
    }
}

impl FromStr for FuelGrid {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<FuelGrid, ParseIntError> {
        let serial_no = s.trim().parse::<isize>()?;
        Ok(FuelGrid { serial_no })
    }
}

fn power_level(coords: Coords, serial_no: isize) -> isize {
    let x = isize::try_from(coords.x).unwrap() + 1;
    let y = isize::try_from(coords.y).unwrap() + 1;
    let rack_id = x + 10;
    (rack_id * y + serial_no) * rack_id / 100 % 10 - 5
}

fn main() {
    println!("{}", Input::from_env().parse::<FuelGrid>().max_square());
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(3, 5, 8, 4)]
    #[case(122, 79, 57, -5)]
    #[case(217, 196, 39, 0)]
    #[case(101, 153, 71, 4)]
    fn test_power_level(
        #[case] x: usize,
        #[case] y: usize,
        #[case] serial_no: isize,
        #[case] power: isize,
    ) {
        assert_eq!(power_level(Coords::new(y - 1, x - 1), serial_no), power);
    }

    #[rstest]
    #[case("18", "33,45")]
    #[case("42", "21,61")]
    fn test_max_square(#[case] fuelgrid: FuelGrid, #[case] coord: &str) {
        assert_eq!(fuelgrid.max_square(), coord);
    }
}
