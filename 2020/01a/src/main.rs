use adventutil::{unordered_pairs, Input};

fn solve(input: Input) -> u32 {
    let entries = input.parse_lines::<u32>().collect::<Vec<_>>();
    for (&e1, &e2) in unordered_pairs(&entries) {
        if e1 + e2 == 2020 {
            return e1 * e2;
        }
    }
    panic!("No solution found");
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("1721\n979\n366\n299\n675\n1456");
        assert_eq!(solve(input), 514579);
    }
}
