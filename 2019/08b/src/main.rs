use adventutil::grid::{Coords, Grid, GridBounds};
use adventutil::Input;

fn solve(input: Input, width: usize, height: usize) -> String {
    let digits = input
        .read()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let bounds = GridBounds::new(height, width);
    let layers = digits
        .chunks(width * height)
        .map(|layer| Grid::<u32>::from_fn(bounds, |(y, x)| layer[y * width + x]))
        .collect::<Vec<_>>();
    Grid::from_fn(bounds, |c: Coords| {
        for ly in &layers {
            if ly[c] != 2 {
                return ly[c] != 0;
            }
        }
        false
    })
    .draw()
    .to_string()
}

fn main() {
    println!("{}", solve(Input::from_env(), 25, 6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("0222112222120000");
        assert_eq!(solve(input, 2, 2), ".#\n#.");
    }
}
