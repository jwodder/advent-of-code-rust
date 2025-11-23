use adventutil::Input;

fn solve(input: Input) -> u32 {
    let mut total_score = 0;
    let mut depth = 0;
    let mut in_garbage = false;
    let mut cancelling = false;
    for ch in input.read().chars() {
        if cancelling {
            cancelling = false;
        } else {
            match (ch, in_garbage) {
                ('{', false) => depth += 1,
                ('}', false) => {
                    total_score += depth;
                    depth -= 1;
                }
                ('<', false) => in_garbage = true,
                ('>', true) => in_garbage = false,
                ('!', true) => cancelling = true,
                _ => (),
            }
        }
    }
    total_score
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("{}", 1)]
    #[case("{{{}}}", 6)]
    #[case("{{{},{},{{}}}}", 16)]
    #[case("{<a>,<a>,<a>,<a>}", 1)]
    #[case("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9)]
    #[case("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9)]
    #[case("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3)]
    fn examples(#[case] s: &'static str, #[case] score: u32) {
        let input = Input::from(s);
        assert_eq!(solve(input), score);
    }
}
