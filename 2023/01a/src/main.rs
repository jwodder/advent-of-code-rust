use adventutil::Input;

fn line2number(s: &str) -> u32 {
    let mut first_digit = None;
    let mut last_digit = None;
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            let d = ch.to_digit(10).unwrap();
            if first_digit.is_none() {
                first_digit = Some(d);
            }
            last_digit = Some(d);
        }
    }
    first_digit.unwrap() * 10 + last_digit.unwrap()
}

fn solve(input: Input) -> u32 {
    input.lines().map(|s| line2number(&s)).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn example1() {
        let input = Input::from("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n");
        assert_eq!(solve(input), 142);
    }

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn line2number(#[case] line: &str, #[case] number: u32) {
        assert_eq!(super::line2number(line), number);
    }
}
