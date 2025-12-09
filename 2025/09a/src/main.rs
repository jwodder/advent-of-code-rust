use adventutil::{Input, unordered_pairs};

fn solve(input: Input) -> u64 {
    let tiles = input
        .lines()
        .map(|ln| {
            let (x, y) = ln.split_once(',').unwrap();
            let x = x.parse::<u64>().unwrap();
            let y = y.parse::<u64>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();
    unordered_pairs(&tiles)
        .map(|(&t1, &t2)| (t1.0.abs_diff(t2.0) + 1) * (t1.1.abs_diff(t2.1) + 1))
        .max()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n");
        assert_eq!(solve(input), 50);
    }
}
