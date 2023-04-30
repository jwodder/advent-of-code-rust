use adventutil::grid::Grid;
use adventutil::Input;

fn solve(input: Input) -> String {
    let mut digits = String::new();
    let grid = Grid::try_from(vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ])
    .unwrap();
    let mut cell = grid.get_cell((1, 1)).unwrap();
    for ln in input.lines() {
        for c in ln.chars() {
            cell = match c {
                'U' => cell.north().unwrap_or(cell),
                'D' => cell.south().unwrap_or(cell),
                'L' => cell.west().unwrap_or(cell),
                'R' => cell.east().unwrap_or(cell),
                c => panic!("Invalid instruction {c:?}"),
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
    fn test_example1() {
        let input = Input::from("ULL\nRRDDD\nLURDL\nUUUUD\n");
        assert_eq!(solve(input), "1985");
    }
}
