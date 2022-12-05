use adventutil::grid::{Coords, Direction, Grid};
use adventutil::{dijkstra_length, Input};

fn solve(input: Input) -> u32 {
    let grid = input.parse::<Grid<u32>>();
    let start = Coords::new(0, 0);
    let end = Coords::new(grid.height() - 1, grid.width() - 1);
    dijkstra_length(start, end, |&coords| {
        let cell = grid.get_cell(coords).unwrap();
        Direction::cardinals().filter_map(move |d| cell.neighbor(d).map(|c| (c.coords(), *c.get())))
    })
    .expect("No route to end")
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "1163751742\n",
            "1381373672\n",
            "2136511328\n",
            "3694931569\n",
            "7463417111\n",
            "1319128137\n",
            "1359912421\n",
            "3125421639\n",
            "1293138521\n",
            "2311944581\n",
        ));
        assert_eq!(solve(input), 40);
    }
}
