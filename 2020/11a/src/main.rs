use adventutil::grid::Grid;
use adventutil::pullparser::ParseError;
use adventutil::Input;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl FromStr for Tile {
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
        Tile::Empty if cell.adjacent().all(|c| c != Tile::Occupied) => Tile::Occupied,
        Tile::Occupied
            if cell
                .adjacent()
                .filter(|c| *c.get() == Tile::Occupied)
                .count()
                >= 4 =>
        {
            Tile::Empty
        }
        t => t,
    })
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
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
        assert_eq!(solve(input), 37);
    }
}
