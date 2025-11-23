use adventutil::Input;

fn chksm<I: IntoIterator<Item = u32>>(iter: I) -> u32 {
    let row = iter.into_iter().collect::<Vec<_>>();
    for i in 0..row.len() {
        for j in 0..row.len() {
            if i != j && row[i] % row[j] == 0 {
                return row[i] / row[j];
            }
        }
    }
    panic!("No evenly-divisible numbers found");
}

fn solve(input: Input) -> u32 {
    input
        .lines()
        .map(|s| {
            chksm(
                s.split_ascii_whitespace()
                    .map(|s2| s2.parse::<u32>().unwrap()),
            )
        })
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("5 9 2 8\n9 4 7 3\n3 8 6 5\n");
        assert_eq!(solve(input), 9);
    }
}
