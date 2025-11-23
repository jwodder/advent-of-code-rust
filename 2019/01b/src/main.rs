use adventutil::Input;

fn fuel(mass: u32) -> Option<u32> {
    (mass / 3).checked_sub(2)
}

fn fuel4fuel(mass: u32) -> u32 {
    std::iter::successors(Some(mass), |&m| fuel(m))
        .skip(1)
        .sum()
}

fn solve(input: Input) -> u32 {
    input.parse_lines::<u32>().map(fuel4fuel).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(14, 2)]
    #[case(1969, 966)]
    #[case(100756, 50346)]
    fn examples(#[case] mass: u32, #[case] needed: u32) {
        assert_eq!(fuel4fuel(mass), needed);
    }
}
