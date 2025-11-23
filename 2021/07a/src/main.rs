// Key insight: <https://math.stackexchange.com/q/113270/10655>
use adventutil::Input;

fn solve(input: Input) -> u32 {
    let mut positions = input.parse_csv_line::<u32>();
    let m = median(&mut positions);
    positions.into_iter().map(|p| p.abs_diff(m)).sum()
}

fn median(values: &mut [u32]) -> u32 {
    let (_, m, _) = values.select_nth_unstable(values.len() / 2);
    *m
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(solve(input), 37);
    }
}
