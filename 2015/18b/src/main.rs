use adventutil::Input;
use adventutil::grid::{Grid, GridFromError};

#[derive(Clone, Debug, Eq, PartialEq)]
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
        self.0.into_true_coords().count()
    }
}

impl std::str::FromStr for ConwayLights {
    type Err = GridFromError;

    fn from_str(s: &str) -> Result<ConwayLights, Self::Err> {
        Grid::from_drawing(s).map(ConwayLights)
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
    fn example1() {
        let input = Input::from("##.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.#\n");
        assert_eq!(solve(input, 5), 17);
    }
}
