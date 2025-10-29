use adventutil::Input;
use adventutil::grid::{Coords, Direction, Grid};

fn solve(input: Input) -> usize {
    // We can't use input.parse() here, as that trims the input.
    let map = input.read().parse::<Grid<char>>().unwrap();
    let start_x = (0..(map.width())).position(|x| map[(0, x)] == '|').unwrap();
    let mut cell = map.get_cell(Coords { y: 0, x: start_x }).unwrap();
    let mut d = Direction::South;
    let mut steps = 0;
    loop {
        match *cell {
            '|' | '-' => (),
            c if c.is_ascii_alphabetic() => (),
            '+' => {
                d = Direction::cardinals()
                    .find(|&d2| d2 != -d && cell.neighbor(d2).as_deref() != Some(&' '))
                    .unwrap();
            }
            ' ' => break,
            c => panic!("Unexpected tile {c:?}"),
        }
        cell = cell.neighbor(d).unwrap();
        steps += 1;
    }
    steps
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
            "     |          \n",
            "     |  +--+    \n",
            "     A  |  C    \n",
            " F---|----E|--+ \n",
            "     |  |  |  D \n",
            "     +B-+  +--+ \n",
        ));
        assert_eq!(solve(input), 38);
    }
}
