use adventutil::grid::{Direction, Grid};
use adventutil::{dijkstra_length, Input};

fn solve(input: Input) -> u32 {
    let grid = input.parse::<Grid<char>>();
    let mut start = None;
    let mut end = None;
    for (coords, &c) in &grid {
        if c == 'S' {
            start = Some(coords);
        }
        if c == 'E' {
            end = Some(coords);
        }
    }
    let start = start.expect("Start not found");
    let end = end.expect("End not found");
    dijkstra_length(start, end, |&coords| {
        let cell = grid.get_cell(coords).unwrap();
        Direction::cardinals().filter_map(move |d| {
            let c2 = cell.neighbor(d)?;
            can_move(*cell.get(), *c2.get()).then(|| (c2.coords(), 1))
        })
    })
    .expect("No route to end")
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
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n");
        assert_eq!(solve(input), 31);
    }
}
