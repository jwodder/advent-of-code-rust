use adventutil::Input;

fn fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

fn solve(input: Input) -> u32 {
    input.parse_lines::<u32>().map(fuel).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(12, 2)]
    #[case(14, 2)]
    #[case(1969, 654)]
    #[case(100756, 33583)]
    fn test_fuel(#[case] mass: u32, #[case] needed: u32) {
        assert_eq!(fuel(mass), needed);
    }
}
