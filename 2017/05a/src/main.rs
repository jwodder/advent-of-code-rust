use adventutil::Input;

fn solve(input: Input) -> usize {
    let mut jumps = input.parse_lines::<i32>().collect::<Vec<_>>();
    let mut steps = 0;
    let mut i = 0i32;
    while let Some(offset) = usize::try_from(i).ok().and_then(|i| jumps.get_mut(i)) {
        i += *offset;
        *offset += 1;
        steps += 1;
    }
    steps
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("0\n3\n0\n1\n-3\n");
        assert_eq!(solve(input), 5);
    }
}
