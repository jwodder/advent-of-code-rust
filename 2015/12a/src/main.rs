use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct IntParser {
    sign: i32,
    value: u32,
}

impl IntParser {
    fn new(c: char) -> IntParser {
        if c == '-' {
            IntParser { sign: -1, value: 0 }
        } else {
            IntParser {
                sign: 1,
                value: c.to_digit(10).unwrap(),
            }
        }
    }

    fn push(mut self, c: char) -> IntParser {
        self.value = self.value * 10 + c.to_digit(10).unwrap();
        self
    }

    fn get(&self) -> i32 {
        self.sign * i32::try_from(self.value).unwrap()
    }
}

fn solve(input: Input) -> i32 {
    let mut total = 0;
    let mut parser = None;
    for c in input.read().chars() {
        if parser.is_none() && (c == '-' || c.is_ascii_digit()) {
            parser = Some(IntParser::new(c));
        } else if parser.is_some() {
            if c.is_ascii_digit() {
                parser = parser.map(|p| p.push(c));
            } else {
                total += parser.take().unwrap().get();
            }
        }
    }
    if let Some(p) = parser {
        total += p.get();
    }
    total
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"{"a":2,"b":4}"#, 6)]
    #[case("[[[3]]]", 3)]
    #[case(r#"{"a":{"b":4},"c":-1}"#, 3)]
    #[case(r#"{"a":[-1,1]}"#, 0)]
    #[case(r#"[-1,{"a":1}]"#, 0)]
    #[case("[]", 0)]
    #[case("{}", 0)]
    fn sum_nums(#[case] s: &'static str, #[case] total: i32) {
        let input = Input::from(s);
        assert_eq!(solve(input), total);
    }
}
