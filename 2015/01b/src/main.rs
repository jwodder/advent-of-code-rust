use adventutil::Input;

fn first_basement(s: &str) -> usize {
    let mut floor = 0;
    for (i, c) in (1..).zip(s.chars()) {
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
    println!("{}", first_basement(&Input::from_env().read()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_first_basement(#[case] s: &str, #[case] pos: usize) {
        assert_eq!(first_basement(s), pos);
    }
}
