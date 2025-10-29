use adventutil::Input;
use adventutil::grid::{Direction, Grid};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cart {
    dir: Direction,
    turn_index: usize,
}

impl Cart {
    fn new(dir: Direction) -> Cart {
        Cart { dir, turn_index: 0 }
    }

    fn turn(&mut self) {
        match self.turn_index % 3 {
            0 => self.dir = self.dir.turn_left(),
            1 => (),
            2 => self.dir = self.dir.turn_right(),
            _ => unreachable!(),
        }
        self.turn_index += 1;
    }
}

fn solve(input: Input) -> String {
    // We can't use input.parse() here, as that trims the input.
    let mut map = input.read().parse::<Grid<char>>().unwrap();
    let mut carts = BTreeMap::new();
    for c in map.iter_coords() {
        match map[c] {
            '^' => {
                carts.insert(c, Cart::new(Direction::North));
                map[c] = '|';
            }
            'v' => {
                carts.insert(c, Cart::new(Direction::South));
                map[c] = '|';
            }
            '<' => {
                carts.insert(c, Cart::new(Direction::West));
                map[c] = '-';
            }
            '>' => {
                carts.insert(c, Cart::new(Direction::East));
                map[c] = '-';
            }
            _ => (),
        }
    }
    let bounds = map.bounds();
    loop {
        /*
        // BEGIN DEBUG
        for (c, &tile) in &map {
            let chr = if let Some(cart) = carts.get(&c) {
                match cart.dir {
                    Direction::North => '^',
                    Direction::West => '<',
                    Direction::East => '>',
                    Direction::South => 'v',
                    _ => unreachable!(),
                }
            } else {
                tile
            };
            eprint!("{chr}");
            if c.x + 1 == map.width() {
                eprintln!();
            }
        }
        eprintln!();
        // END DEBUG
        */

        for (c, mut cart) in carts.clone() {
            match map[c] {
                '|' | '-' => (),
                '/' => {
                    cart.dir = match cart.dir {
                        Direction::North => Direction::East,
                        Direction::East => Direction::North,
                        Direction::South => Direction::West,
                        Direction::West => Direction::South,
                        _ => unreachable!(),
                    };
                }
                '\\' => {
                    cart.dir = match cart.dir {
                        Direction::North => Direction::West,
                        Direction::East => Direction::South,
                        Direction::South => Direction::East,
                        Direction::West => Direction::North,
                        _ => unreachable!(),
                    }
                }
                '+' => cart.turn(),
                c => panic!("Unexpected tile {c:?}"),
            }
            let c2 = bounds.move_in(c, cart.dir).unwrap();
            carts.remove(&c);
            if carts.insert(c2, cart).is_some() {
                return format!("{},{}", c2.x, c2.y);
            }
        }
    }
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
            "/->-\\        \n",
            "|   |  /----\\\n",
            "| /-+--+-\\  |\n",
            "| | |  | v  |\n",
            "\\-+-/  \\-+--/\n",
            "  \\------/   \n",
        ));
        assert_eq!(solve(input), "7,3");
    }
}
