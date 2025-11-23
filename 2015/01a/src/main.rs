use adventutil::Input;

fn solve(input: Input) -> i32 {
    let mut floor = 0;
    for c in input.read().chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn walk(#[case] s: &'static str, #[case] floor: i32) {
        let input = Input::from(s);
        assert_eq!(solve(input), floor);
    }
}
