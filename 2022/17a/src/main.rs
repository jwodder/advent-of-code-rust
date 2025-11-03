use adventutil::Input;
use adventutil::grid::{Coords, Direction, Grid, GridBounds, Padding};

// Because y decreases when moving down
const DOWN: Direction = Direction::North;

const TUNNEL_WIDTH: usize = 7;

#[derive(Clone, Debug)]
struct Tunnel {
    rocks: Grid<bool>,
    max_rock_y: Option<usize>,
    vent_dirs: std::iter::Cycle<std::vec::IntoIter<Direction>>,
}

impl Tunnel {
    fn new(vent_dirs: Vec<Direction>) -> Tunnel {
        Tunnel {
            rocks: Grid::filled(
                GridBounds {
                    width: TUNNEL_WIDTH,
                    height: 10,
                },
                false,
            ),
            max_rock_y: None,
            vent_dirs: vent_dirs.into_iter().cycle(),
        }
    }

    fn is_rock(&self, c: Coords) -> bool {
        if c.x >= TUNNEL_WIDTH {
            true
        } else {
            self.rocks.get(c).copied().unwrap_or_default()
        }
    }

    fn settle(&mut self, shape: Shape, ll_corner: Coords) {
        let new_rocks = shape.rock_coords(ll_corner).collect::<Vec<_>>();
        let max_y = new_rocks.iter().map(|c| c.y).max().unwrap();
        if let Some(gain) = (max_y + 1)
            .checked_sub(self.rocks.height())
            .filter(|&diff| diff > 0)
        {
            let padding = Padding {
                left: 0,
                right: 0,
                top: 0,
                bottom: gain,
            };
            self.rocks = self.rocks.embiggened(padding, false);
        }
        for c in new_rocks {
            self.rocks.set(c, true);
        }
        if self.max_rock_y.is_none_or(|y| y < max_y) {
            self.max_rock_y = Some(max_y);
        }
    }

    fn rock_height(&self) -> usize {
        self.max_rock_y.unwrap() + 1
    }

    fn new_ll_corner(&self) -> Coords {
        Coords {
            y: self.max_rock_y.map_or(3, |y| y + 4),
            x: 2,
        }
    }

    fn add_rock(&mut self, shape: Shape) {
        let mut ll = self.new_ll_corner();
        while let Some(d) = self.vent_dirs.next() {
            if shape.can_move(self, ll, d) {
                ll = ll.domove(d).unwrap();
            }
            if shape.can_move(self, ll, DOWN) {
                ll = ll.domove(DOWN).unwrap();
            } else {
                self.settle(shape, ll);
                return;
            }
        }
    }

    #[cfg(test)]
    fn draw(&self) -> String {
        let mut lines = vec![String::from("+-------+")];
        for y in 0..=(self.max_rock_y.unwrap_or_default() + 1) {
            let mut s = String::from("|");
            for x in 0..TUNNEL_WIDTH {
                if self.is_rock(Coords { y, x }) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('|');
            lines.push(s);
        }
        lines.reverse();
        lines.join("\n")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Shape {
    Bar,
    Plus,
    Ell,
    Pipe,
    Block,
}

impl Shape {
    fn iter() -> impl Iterator<Item = Shape> + Clone {
        [
            Shape::Bar,
            Shape::Plus,
            Shape::Ell,
            Shape::Pipe,
            Shape::Block,
        ]
        .into_iter()
    }

    fn can_move(self, tunnel: &Tunnel, ll_coords: Coords, d: Direction) -> bool {
        self.rock_coords(ll_coords)
            .all(|c| c.domove(d).is_some_and(|c2| !tunnel.is_rock(c2)))
    }

    fn rock_coords(self, ll_coords: Coords) -> impl Iterator<Item = Coords> {
        let y = ll_coords.y;
        let x = ll_coords.x;
        match self {
            Shape::Bar => vec![
                Coords { y, x },
                Coords { y, x: x + 1 },
                Coords { y, x: x + 2 },
                Coords { y, x: x + 3 },
            ],
            Shape::Plus => vec![
                Coords { y: y + 1, x: x + 1 },
                Coords { y, x: x + 1 },
                Coords { y: y + 1, x },
                Coords { y: y + 2, x: x + 1 },
                Coords { y: y + 1, x: x + 2 },
            ],
            Shape::Ell => vec![
                Coords { y, x },
                Coords { y, x: x + 1 },
                Coords { y, x: x + 2 },
                Coords { y: y + 1, x: x + 2 },
                Coords { y: y + 2, x: x + 2 },
            ],
            Shape::Pipe => vec![
                Coords { y, x },
                Coords { y: y + 1, x },
                Coords { y: y + 2, x },
                Coords { y: y + 3, x },
            ],
            Shape::Block => vec![
                Coords { y, x },
                Coords { y: y + 1, x },
                Coords { y, x: x + 1 },
                Coords { y: y + 1, x: x + 1 },
            ],
        }
        .into_iter()
    }
}

fn parse_vent_dirs(input: Input) -> Vec<Direction> {
    input
        .read()
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Direction::West,
            '>' => Direction::East,
            c => panic!("Unexpected input character: {c:?}"),
        })
        .collect()
}

fn solve(input: Input) -> usize {
    let mut tunnel = Tunnel::new(parse_vent_dirs(input));
    for shape in Shape::iter().cycle().take(2022) {
        tunnel.add_rock(shape);
    }
    tunnel.rock_height()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n");
        let mut tunnel = Tunnel::new(parse_vent_dirs(input));
        let mut shapes = Shape::iter().cycle();
        for _ in 0..3 {
            tunnel.add_rock(shapes.next().unwrap());
        }
        assert_eq!(
            tunnel.draw(),
            concat!(
                "|.......|\n",
                "|..#....|\n",
                "|..#....|\n",
                "|####...|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|..####.|\n",
                "+-------+",
            )
        );

        tunnel.add_rock(shapes.next().unwrap());
        assert_eq!(
            tunnel.draw(),
            concat!(
                "|.......|\n",
                "|....#..|\n",
                "|..#.#..|\n",
                "|..#.#..|\n",
                "|#####..|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|..####.|\n",
                "+-------+",
            )
        );

        tunnel.add_rock(shapes.next().unwrap());
        assert_eq!(
            tunnel.draw(),
            concat!(
                "|.......|\n",
                "|....##.|\n",
                "|....##.|\n",
                "|....#..|\n",
                "|..#.#..|\n",
                "|..#.#..|\n",
                "|#####..|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|..####.|\n",
                "+-------+",
            )
        );

        tunnel.add_rock(shapes.next().unwrap());
        assert_eq!(
            tunnel.draw(),
            concat!(
                "|.......|\n",
                "|.####..|\n",
                "|....##.|\n",
                "|....##.|\n",
                "|....#..|\n",
                "|..#.#..|\n",
                "|..#.#..|\n",
                "|#####..|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|..####.|\n",
                "+-------+",
            )
        );

        tunnel.add_rock(shapes.next().unwrap());
        assert_eq!(
            tunnel.draw(),
            concat!(
                "|.......|\n",
                "|..#....|\n",
                "|.###...|\n",
                "|..#....|\n",
                "|.####..|\n",
                "|....##.|\n",
                "|....##.|\n",
                "|....#..|\n",
                "|..#.#..|\n",
                "|..#.#..|\n",
                "|#####..|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|..####.|\n",
                "+-------+",
            )
        );

        tunnel.add_rock(shapes.next().unwrap());
        assert_eq!(
            tunnel.draw(),
            concat!(
                "|.......|\n",
                "|.....#.|\n",
                "|.....#.|\n",
                "|..####.|\n",
                "|.###...|\n",
                "|..#....|\n",
                "|.####..|\n",
                "|....##.|\n",
                "|....##.|\n",
                "|....#..|\n",
                "|..#.#..|\n",
                "|..#.#..|\n",
                "|#####..|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|..####.|\n",
                "+-------+",
            )
        );

        tunnel.add_rock(shapes.next().unwrap());
        assert_eq!(
            tunnel.draw(),
            concat!(
                "|.......|\n",
                "|....#..|\n",
                "|....#..|\n",
                "|....##.|\n",
                "|....##.|\n",
                "|..####.|\n",
                "|.###...|\n",
                "|..#....|\n",
                "|.####..|\n",
                "|....##.|\n",
                "|....##.|\n",
                "|....#..|\n",
                "|..#.#..|\n",
                "|..#.#..|\n",
                "|#####..|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|..####.|\n",
                "+-------+",
            )
        );

        tunnel.add_rock(shapes.next().unwrap());
        assert_eq!(
            tunnel.draw(),
            concat!(
                "|.......|\n",
                "|....#..|\n",
                "|....#..|\n",
                "|....##.|\n",
                "|##..##.|\n",
                "|######.|\n",
                "|.###...|\n",
                "|..#....|\n",
                "|.####..|\n",
                "|....##.|\n",
                "|....##.|\n",
                "|....#..|\n",
                "|..#.#..|\n",
                "|..#.#..|\n",
                "|#####..|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|..####.|\n",
                "+-------+",
            )
        );
    }

    #[test]
    fn example2() {
        let input = Input::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n");
        assert_eq!(solve(input), 3068);
    }
}
