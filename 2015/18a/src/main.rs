use adventutil::grid::{Coords, Grid, ParseGridError};
use adventutil::Input;
use std::char::ParseCharError;
use std::str::FromStr;

struct ConwayLights(Grid<bool>);

impl ConwayLights {
    fn step(self) -> ConwayLights {
        ConwayLights(Grid::from_fn(self.0.bounds(), |coords: Coords| {
            let cell = self.0.get_cell(coords).unwrap();
            let live_neighbors = cell.adjacent().filter(|c| *c.get()).count();
            matches!((cell.get(), live_neighbors), (_, 3) | (true, 2))
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

fn solve(mut lights: ConwayLights, steps: usize) -> usize {
    for _ in 0..steps {
        lights = lights.step();
    }
    lights.lit()
}

fn main() {
    println!("{}", solve(Input::from_env().parse::<ConwayLights>(), 100));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let lights = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..\n"
            .parse::<ConwayLights>()
            .unwrap();
        assert_eq!(solve(lights, 4), 4);
    }
}
