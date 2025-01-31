use adventutil::Input;

fn decompress_len(mut s: &str) -> usize {
    let mut length = 0;
    while let Some(i) = s.find('(') {
        length += i;
        s = &s[(i + 1)..];
        let xpos = s.find('x').unwrap();
        let closepos = s.find(')').unwrap();
        let span = s[..xpos].parse::<usize>().unwrap();
        let repeat = s[(xpos + 1)..closepos].parse::<usize>().unwrap();
        s = &s[(closepos + 1)..];
        length += decompress_len(&s[..span]) * repeat;
        s = &s[span..];
    }
    length += s.len();
    length
}

fn solve(input: Input) -> usize {
    let s = input.read();
    decompress_len(s.trim())
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(3x3)XYZ", 9)]
    #[case("X(8x2)(3x3)ABCY", 20)]
    #[case("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920)]
    #[case("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", 445)]
    fn test_example(#[case] s: &'static str, #[case] length: usize) {
        let input = Input::from(s);
        assert_eq!(solve(input), length);
    }
}
