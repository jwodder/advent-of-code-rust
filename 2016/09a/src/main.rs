use adventutil::Input;

fn solve(input: Input) -> usize {
    let s = input.read();
    let mut length = 0;
    let mut s = s.trim();
    while let Some(i) = s.find('(') {
        length += i;
        s = &s[(i + 1)..];
        let xpos = s.find('x').unwrap();
        let closepos = s.find(')').unwrap();
        let span = s[..xpos].parse::<usize>().unwrap();
        let repeat = s[(xpos + 1)..closepos].parse::<usize>().unwrap();
        length += span * repeat;
        s = &s[(closepos + 1 + span)..];
    }
    length += s.len();
    length
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ADVENT", 6)]
    #[case("A(1x5)BC", 7)]
    #[case("(3x3)XYZ", 9)]
    #[case("A(2x2)BCD(2x2)EFG", 11)]
    #[case("(6x1)(1x3)A", 6)]
    #[case("X(8x2)(3x3)ABCY", 18)]
    fn test_example(#[case] s: &'static str, #[case] length: usize) {
        let input = Input::from(s);
        assert_eq!(solve(input), length);
    }
}
