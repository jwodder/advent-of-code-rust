use adventutil::Input;
use adventutil::grid::{Cell, Direction, Grid};
use adventutil::pullparser::ParseError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl std::str::FromStr for Tile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Tile, ParseError> {
        match s {
            "." => Ok(Tile::Floor),
            "L" => Ok(Tile::Empty),
            "#" => Ok(Tile::Occupied),
            s => Err(ParseError::InvalidToken(s.into())),
        }
    }
}

fn solve(input: Input) -> usize {
    let mut layout = input.parse::<Grid<Tile>>();
    loop {
        let next = step(&layout);
        if next == layout {
            return next.into_values().filter(|&t| t == Tile::Occupied).count();
        }
        layout = next;
    }
}

fn step(layout: &Grid<Tile>) -> Grid<Tile> {
    layout.map_cells(|cell| match *cell {
        Tile::Empty if visible_occupied(&cell) == 0 => Tile::Occupied,
        Tile::Occupied if visible_occupied(&cell) >= 5 => Tile::Empty,
        t => t,
    })
}

fn visible_occupied(cell: &Cell<'_, Tile>) -> usize {
    let mut occupied = 0;
    for d in Direction::adjacent() {
        for c in std::iter::successors(Some(*cell), |c| c.neighbor(d)).skip(1) {
            match *c {
                Tile::Floor => (),
                Tile::Empty => break,
                Tile::Occupied => {
                    occupied += 1;
                    break;
                }
            }
        }
    }
    occupied
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "L.LL.LL.LL\n",
            "LLLLLLL.LL\n",
            "L.L.L..L..\n",
            "LLLL.LL.LL\n",
            "L.LL.LL.LL\n",
            "L.LLLLL.LL\n",
            "..L.L.....\n",
            "LLLLLLLLLL\n",
            "L.LLLLLL.L\n",
            "L.LLLLL.LL\n",
        ));
        assert_eq!(solve(input), 26);
    }
}
