use adventutil::grid::Grid;
use adventutil::{Input, unit_dijkstra_length};

fn solve(input: Input) -> u32 {
    let grid = input.parse::<Grid<char>>();
    let Some((end, _)) = grid.iter().find(|&(_, &c)| c == 'E') else {
        panic!("End not found");
    };
    unit_dijkstra_length(
        end,
        |&n| matches!(grid[n], 'a' | 'S'),
        |&coords| {
            let mut nextpoints = Vec::new();
            let cell = grid.get_cell(coords).unwrap();
            for c2 in cell.cardinal_neighbors() {
                if can_move(*c2.get(), *cell.get()) {
                    nextpoints.push(c2.coords());
                }
            }
            nextpoints
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
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n");
        assert_eq!(solve(input), 29);
    }
}
