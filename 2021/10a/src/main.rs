use adventutil::Input;

const PAREN_SCORE: usize = 3;
const BRACKET_SCORE: usize = 57;
const BRACE_SCORE: usize = 1197;
const ANGLE_SCORE: usize = 25137;

#[derive(Debug, Eq, PartialEq)]
enum Classification {
    Valid,
    Corrupt(usize),
    Incomplete,
}

fn classify(s: &str) -> Classification {
    use Classification::*;
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            c if stack.last() == Some(&c) => {
                stack.pop();
            }
            ')' => return Corrupt(PAREN_SCORE),
            ']' => return Corrupt(BRACKET_SCORE),
            '}' => return Corrupt(BRACE_SCORE),
            '>' => return Corrupt(ANGLE_SCORE),
            c => panic!("Unexpected character: {c:?}"),
        }
    }
    if stack.is_empty() {
        Valid
    } else {
        Incomplete
    }
}

fn score<I, S>(iter: I) -> usize
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    iter.into_iter()
        .filter_map(|s| match classify(s.as_ref()) {
            Classification::Corrupt(sc) => Some(sc),
            _ => None,
        })
        .sum()
}

fn main() {
    println!("{}", score(Input::from_env().lines()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use Classification::*;

    #[rstest]
    #[case("()", Valid)]
    #[case("[]", Valid)]
    #[case("([])", Valid)]
    #[case("{()()()}", Valid)]
    #[case("<([{}])>", Valid)]
    #[case("[<>({}){}[([])<>]]", Valid)]
    #[case("(((((((((())))))))))", Valid)]
    #[case("(]", Corrupt(BRACKET_SCORE))]
    #[case("{()()()>", Corrupt(ANGLE_SCORE))]
    #[case("(((()))}", Corrupt(BRACE_SCORE))]
    #[case("<([]){()}[{}])", Corrupt(PAREN_SCORE))]
    #[case("{([(<{}[<>[]}>{[]{[(<()>", Corrupt(BRACE_SCORE))]
    #[case("[[<[([]))<([[{}[[()]]]", Corrupt(PAREN_SCORE))]
    #[case("[{[{({}]{}}([{[{{{}}([]", Corrupt(BRACKET_SCORE))]
    #[case("[<(<(<(<{}))><([]([]()", Corrupt(PAREN_SCORE))]
    #[case("<{([([[(<>()){}]>(<<{{", Corrupt(ANGLE_SCORE))]
    fn test_classify(#[case] s: &str, #[case] c: Classification) {
        assert_eq!(classify(s), c);
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
        assert_eq!(score(lines), 26397);
    }
}
