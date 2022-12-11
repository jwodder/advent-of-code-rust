use adventutil::Input;

fn solve(input: Input) -> usize {
    let mut floor = 0;
    for (i, c) in (1..).zip(input.read().chars()) {
        match c {
            '(' => floor += 1,
            ')' => {
                floor -= 1;
                if floor < 0 {
                    return i;
                }
            }
            _ => (),
        }
    }
    panic!("Basement never entered");
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_first_basement(#[case] s: &'static str, #[case] pos: usize) {
        let input = Input::from(s);
        assert_eq!(solve(input), pos);
    }
}
