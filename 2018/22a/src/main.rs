use adventutil::Input;
use adventutil::grid::{Grid, GridBounds};
use adventutil::pullparser::{PullParser, Token};

fn solve(input: Input) -> usize {
    let s = input.read();
    let mut parser = PullParser::new(s.trim());
    parser.skip("depth: ").unwrap();
    let depth = parser.parse_to::<usize, _>(Token::Newline).unwrap();
    parser.skip("target: ").unwrap();
    let target_x = parser.parse_to::<usize, _>(',').unwrap();
    let target_y = parser.parse_to::<usize, _>(Token::Eof).unwrap();
    let mut grid = Grid::filled(
        GridBounds {
            height: target_y + 1,
            width: target_x + 1,
        },
        0,
    );
    let mut risk = 0;
    for c in grid.iter_coords() {
        let geologic = match (c.x, c.y) {
            (0, 0) => 0,
            (x, y) if x == target_x && y == target_y => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => grid[(y, x - 1)] * grid[(y - 1, x)],
        };
        let erosion = (geologic + depth) % 20183;
        grid[c] = erosion;
        risk += erosion % 3;
    }
    risk
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("depth: 510\ntarget: 10,10\n");
        assert_eq!(solve(input), 114);
    }
}
