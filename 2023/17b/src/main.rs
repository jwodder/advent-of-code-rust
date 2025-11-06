use adventutil::grid::{Coords, Direction, Grid};
use adventutil::{Input, dijkstra_length};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Node {
    pos: Coords,
    dir: Direction,
    straightness: usize,
}

fn solve(input: Input) -> u32 {
    let grid = input.parse::<Grid<u32>>();
    let start = Node {
        pos: Coords { y: 0, x: 0 },
        dir: Direction::East,
        straightness: 0,
    };
    let end = Coords {
        y: grid.height() - 1,
        x: grid.width() - 1,
    };
    dijkstra_length(
        start,
        |&n| n.pos == end && n.straightness >= 4,
        |&n| {
            let cell = grid.get_cell(n.pos).unwrap();
            let mut next_steps = Vec::with_capacity(3);
            if n.straightness >= 4 {
                let left = n.dir.turn_left();
                if let Some(c2) = cell.neighbor(left) {
                    next_steps.push((
                        Node {
                            pos: c2.coords(),
                            dir: left,
                            straightness: 1,
                        },
                        *c2,
                    ));
                }
                let right = n.dir.turn_right();
                if let Some(c2) = cell.neighbor(right) {
                    next_steps.push((
                        Node {
                            pos: c2.coords(),
                            dir: right,
                            straightness: 1,
                        },
                        *c2,
                    ));
                }
            }
            if n.straightness < 10
                && let Some(c2) = cell.neighbor(n.dir)
            {
                next_steps.push((
                    Node {
                        pos: c2.coords(),
                        dir: n.dir,
                        straightness: n.straightness + 1,
                    },
                    *c2,
                ));
            }
            next_steps
        },
    )
    .unwrap()
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
            "2413432311323\n",
            "3215453535623\n",
            "3255245654254\n",
            "3446585845452\n",
            "4546657867536\n",
            "1438598798454\n",
            "4457876987766\n",
            "3637877979653\n",
            "4654967986887\n",
            "4564679986453\n",
            "1224686865563\n",
            "2546548887735\n",
            "4322674655533\n",
        ));
        assert_eq!(solve(input), 94);
    }

    #[test]
    fn example2() {
        let input = Input::from(concat!(
            "111111111111\n",
            "999999999991\n",
            "999999999991\n",
            "999999999991\n",
            "999999999991\n",
        ));
        assert_eq!(solve(input), 71);
    }
}
