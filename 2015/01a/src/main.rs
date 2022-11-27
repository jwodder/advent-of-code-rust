use adventutil::Input;

fn walk(s: &str) -> i32 {
    let mut floor = 0;
    for c in s.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

fn main() {
    println!("{}", walk(&Input::from_env().read()));
}

#[cfg(test)]
mod test {
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
    fn test_walk(#[case] s: &str, #[case] floor: i32) {
        assert_eq!(walk(s), floor);
    }
}
