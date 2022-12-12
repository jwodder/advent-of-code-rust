use adventutil::grid::{Coords, Direction, Grid};
use adventutil::{dijkstra_length, Input};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Point {
    Moving(Coords),
    LowestPoint,
}

fn solve(input: Input) -> u32 {
    let grid = input.parse::<Grid<char>>();
    let mut end = None;
    for (coords, &c) in grid.enumerate() {
        if c == 'E' {
            end = Some(coords);
            break;
        }
    }
    let end = end.expect("End not found");
    dijkstra_length(
        Point::Moving(end),
        Point::LowestPoint,
        |&point| match point {
            Point::Moving(coords) => {
                let mut nextpoints = Vec::new();
                let cell = grid.get_cell(coords).unwrap();
                for d in Direction::cardinals() {
                    if let Some(c2) = cell.neighbor(d) {
                        if can_move(*c2.get(), *cell.get()) {
                            let p = if c2 == 'a' || c2 == 'S' {
                                Point::LowestPoint
                            } else {
                                Point::Moving(c2.coords())
                            };
                            nextpoints.push((p, 1));
                        }
                    }
                }
                nextpoints
            }
            Point::LowestPoint => Vec::new(),
        },
    )
    .expect("No route to lowest point")
}

fn can_move(mut from: char, mut to: char) -> bool {
    if from == 'S' {
        from = 'a';
    }
    if to == 'E' {
        to = 'z';
    }
    (from as u32) + 1 >= (to as u32)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n");
        assert_eq!(solve(input), 29);
    }
}
