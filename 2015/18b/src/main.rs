use adventutil::grid::{Grid, ParseGridError};
use adventutil::Input;
use std::char::ParseCharError;
use std::str::FromStr;

struct ConwayLights(Grid<bool>);

impl ConwayLights {
    fn step(self) -> ConwayLights {
        ConwayLights(self.0.map_cells(|cell| {
            ((cell.y() == 0 || cell.y() == self.0.height() - 1)
                && (cell.x() == 0 || cell.x() == self.0.width() - 1))
                || {
                    let live_neighbors = cell.adjacent().filter(|c| *c.get()).count();
                    matches!((cell.get(), live_neighbors), (_, 3) | (true, 2))
                }
        }))
    }

    fn lit(self) -> usize {
        self.0.into_values().filter(|&b| b).count()
    }
}

impl FromStr for ConwayLights {
    type Err = ParseGridError<ParseCharError>;

    fn from_str(s: &str) -> Result<ConwayLights, Self::Err> {
        Ok(ConwayLights(s.parse::<Grid<char>>()?.map(|c| c == '#')))
    }
}

fn solve(input: Input, steps: usize) -> usize {
    let mut lights = input.parse::<ConwayLights>();
    for _ in 0..steps {
        lights = lights.step();
    }
    lights.lit()
}

fn main() {
    println!("{}", solve(Input::from_env(), 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("##.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.#\n");
        assert_eq!(solve(input, 5), 17);
    }
}
