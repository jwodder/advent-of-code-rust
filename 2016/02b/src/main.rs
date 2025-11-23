use adventutil::Input;
use adventutil::grid::Grid;

fn solve(input: Input) -> String {
    let mut digits = String::new();
    let grid = Grid::try_from(vec![
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '2', '3', '4', '0'],
        vec!['5', '6', '7', '8', '9'],
        vec!['0', 'A', 'B', 'C', '0'],
        vec!['0', '0', 'D', '0', '0'],
    ])
    .unwrap();
    let mut cell = grid.get_cell((2, 0)).unwrap();
    for ln in input.lines() {
        for c in ln.chars() {
            let c2 = match c {
                'U' => cell.north(),
                'D' => cell.south(),
                'L' => cell.west(),
                'R' => cell.east(),
                c => panic!("Invalid instruction {c:?}"),
            };
            cell = match c2 {
                Some(c) if c == '0' => cell,
                Some(c) => c,
                None => cell,
            };
        }
        digits.push(*cell.get());
    }
    digits
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("ULL\nRRDDD\nLURDL\nUUUUD\n");
        assert_eq!(solve(input), "5DB3");
    }
}
