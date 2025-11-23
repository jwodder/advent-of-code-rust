use adventutil::Input;

fn solve(input: Input) -> u32 {
    let mut total = 0;
    let mut in_garbage = false;
    let mut cancelling = false;
    for ch in input.read().chars() {
        if cancelling {
            cancelling = false;
        } else {
            match (ch, in_garbage) {
                ('<', false) => in_garbage = true,
                ('>', true) => in_garbage = false,
                ('!', true) => cancelling = true,
                (_, true) => total += 1,
                _ => (),
            }
        }
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
    #[case("<>", 0)]
    #[case("<random characters>", 17)]
    #[case("<<<<>", 3)]
    #[case("<{!>}>", 2)]
    #[case("<!!>", 0)]
    #[case("<!!!>>", 0)]
    #[case("<{o\"i!a,<{i<a>", 10)]
    fn examples(#[case] s: &'static str, #[case] score: u32) {
        let input = Input::from(s);
        assert_eq!(solve(input), score);
    }
}
