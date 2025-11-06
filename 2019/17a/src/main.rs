use adventutil::Input;
use adventutil::grid::Grid;
use adventutil::intcode::{Intcode, VecIO};

fn alignment_sum(map: &str) -> usize {
    let grid = map.trim().parse::<Grid<char>>().unwrap();
    grid.iter_cells()
        .filter(|cell| {
            *cell != '.' && cell.cardinal_neighbors().filter(|c2| *c2 != '.').count() == 4
        })
        .map(|cell| cell.coords().y * cell.coords().x)
        .sum()
}

fn solve(input: Input) -> usize {
    let mut program = input.parse::<Intcode>();
    let mut io = VecIO::default();
    program.run(&mut io).unwrap();
    let map = String::from_iter(
        io.output
            .into_iter()
            .map(|i| char::from(u8::try_from(i).unwrap())),
    );
    alignment_sum(&map)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let map = concat!(
            "..#..........\n",
            "..#..........\n",
            "#######...###\n",
            "#.#...#...#.#\n",
            "#############\n",
            "..#...#...#..\n",
            "..#####...^..\n",
        );
        assert_eq!(alignment_sum(map), 76);
    }
}
