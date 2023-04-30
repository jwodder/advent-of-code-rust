use adventutil::Input;

fn repr_delta(s: &str) -> usize {
    let mut iter = s.chars();
    let mut delta = 0;
    while let Some(c) = iter.next() {
        match c {
            '"' => delta += 1,
            '\\' => match iter.next().unwrap() {
                '"' | '\\' => delta += 1,
                'x' => {
                    let _ = iter.next().unwrap();
                    let _ = iter.next().unwrap();
                    delta += 3;
                }
                c => panic!("Invalid escape sequence \\{c}"),
            },
            _ => (),
        }
    }
    delta
}

fn solve(input: Input) -> usize {
    input.lines().map(|s| repr_delta(&s)).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(r#""""#, 2)]
    #[case(r#""abc""#, 2)]
    #[case(r#""aaa\"aaa""#, 3)]
    #[case(r#""\x27""#, 5)]
    fn test_repr_delta(#[case] s: &str, #[case] delta: usize) {
        assert_eq!(repr_delta(s), delta);
    }
}
