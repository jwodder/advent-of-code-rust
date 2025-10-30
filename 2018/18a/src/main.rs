use adventutil::Input;
use adventutil::grid::{Coords, Grid, ParseGridError};
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
struct State(Grid<Cell>);

impl State {
    fn advance(&self) -> State {
        let mut adj_tree_counts = Grid::filled(self.0.bounds(), 0);
        let mut adj_lumber_counts = Grid::filled(self.0.bounds(), 0);
        for cell in self.0.iter_cells() {
            let counter = match *cell {
                Cell::Open => continue,
                Cell::Trees => &mut adj_tree_counts,
                Cell::Lumberyard => &mut adj_lumber_counts,
            };
            for c2 in cell.adjacent() {
                counter[c2.coords()] += 1;
            }
        }
        let output = Grid::from_fn(self.0.bounds(), |c: Coords| {
            match (self.0[c], adj_tree_counts[c], adj_lumber_counts[c]) {
                (Cell::Open, trees, _) if trees >= 3 => Cell::Trees,
                (Cell::Open, _, _) => Cell::Open,
                (Cell::Trees, _, lumber) if lumber >= 3 => Cell::Lumberyard,
                (Cell::Trees, _, _) => Cell::Trees,
                (Cell::Lumberyard, trees, lumber) if trees >= 1 && lumber >= 1 => Cell::Lumberyard,
                (Cell::Lumberyard, _, _) => Cell::Open,
            }
        });
        State(output)
    }

    fn resource_value(&self) -> usize {
        let mut trees = 0;
        let mut lumber = 0;
        for (_, cell) in &self.0 {
            match cell {
                Cell::Open => (),
                Cell::Trees => trees += 1,
                Cell::Lumberyard => lumber += 1,
            }
        }
        trees * lumber
    }
}

impl std::str::FromStr for State {
    type Err = ParseGridError<ParseCellError>;

    fn from_str(s: &str) -> Result<State, Self::Err> {
        s.parse::<Grid<Cell>>().map(State)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Open,
    Trees,
    Lumberyard,
}

impl std::str::FromStr for Cell {
    type Err = ParseCellError;

    fn from_str(s: &str) -> Result<Cell, ParseCellError> {
        match s {
            "." => Ok(Cell::Open),
            "|" => Ok(Cell::Trees),
            "#" => Ok(Cell::Lumberyard),
            _ => Err(ParseCellError(s.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid cell: {0:?}")]
struct ParseCellError(String);

fn solve(input: Input) -> usize {
    let mut state = input.parse::<State>();
    for _ in 0..10 {
        state = state.advance();
    }
    state.resource_value()
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
            ".#.#...|#.\n",
            ".....#|##|\n",
            ".|..|...#.\n",
            "..|#.....#\n",
            "#.#|||#|#|\n",
            "...#.||...\n",
            ".|....|...\n",
            "||...#|.#|\n",
            "|.||||..|.\n",
            "...#.|..|.\n",
        ));
        assert_eq!(solve(input), 1147);
    }
}
