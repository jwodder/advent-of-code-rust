// Key insight: <https://math.stackexchange.com/q/113270/10655>
use adventutil::Input;

fn minfuel(mut positions: Vec<u32>) -> u32 {
    let m = median(&mut positions);
    positions.into_iter().map(|p| p.abs_diff(m)).sum()
}

fn median(values: &mut [u32]) -> u32 {
    let (_, m, _) = values.select_nth_unstable(values.len() / 2);
    *m
}

fn main() {
    let positions = Input::from_env().parse_csv_line::<u32>();
    println!("{}", minfuel(positions));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(minfuel(positions), 37);
    }
}
