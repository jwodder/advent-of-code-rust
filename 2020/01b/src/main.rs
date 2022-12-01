use adventutil::Input;

fn solve(input: Input) -> u32 {
    let entries = input.parse_lines::<u32>().collect::<Vec<_>>();
    for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            for k in (j + 1)..entries.len() {
                if entries[i] + entries[j] + entries[k] == 2020 {
                    return entries[i] * entries[j] * entries[k];
                }
            }
        }
    }
    panic!("No solution found");
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("1721\n979\n366\n299\n675\n1456");
        assert_eq!(solve(input), 241861950);
    }
}
