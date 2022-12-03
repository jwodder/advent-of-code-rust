use adventutil::Input;

const PAREN_SCORE: u64 = 1;
const BRACKET_SCORE: u64 = 2;
const BRACE_SCORE: u64 = 3;
const ANGLE_SCORE: u64 = 4;

fn score_line<S: AsRef<str>>(s: S) -> Option<u64> {
    let mut stack = Vec::new();
    for c in s.as_ref().chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            c if stack.last() == Some(&c) => {
                stack.pop();
            }
            ')' => return None,
            ']' => return None,
            '}' => return None,
            '>' => return None,
            c => panic!("Unexpected character: {c:?}"),
        }
    }
    if stack.is_empty() {
        None
    } else {
        Some(stack.into_iter().rev().fold(0, |sc, ch| {
            sc * 5
                + match ch {
                    ')' => PAREN_SCORE,
                    ']' => BRACKET_SCORE,
                    '}' => BRACE_SCORE,
                    '>' => ANGLE_SCORE,
                    c => panic!("Unexpected character in stack: {c:?}"),
                }
        }))
    }
}

fn score<I, S>(iter: I) -> u64
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut scores: Vec<_> = iter.into_iter().filter_map(score_line).collect();
    let midpoint = scores.len() / 2;
    let (_, median, _) = scores.select_nth_unstable(midpoint);
    *median
}

fn main() {
    println!("{}", score(Input::from_env().lines()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[({(<(())[]>[[{[]{<()<>>", 288957)]
    #[case("[(()[<>])]({[<{<<[]>>(", 5566)]
    #[case("(((({<>}<{<{<>}{[]{[]{}", 1480781)]
    #[case("{<[[]]>}<{[{[{[]{()[[[]", 995444)]
    #[case("<{([{{}}[<[[[<>{}]]]>[]]", 294)]
    fn test_score_line(#[case] s: &str, #[case] score: u64) {
        assert_eq!(score_line(s).unwrap(), score);
    }

    #[test]
    fn test_example1() {
        let lines = [
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];
        assert_eq!(score(lines), 288957);
    }
}
