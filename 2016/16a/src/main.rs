use adventutil::Input;
use itertools::Itertools;

fn dragon(initial: &str, disklen: usize) -> String {
    let mut bits = initial.chars().map(|ch| ch == '1').collect::<Vec<_>>();
    while bits.len() < disklen {
        let length = bits.len();
        bits.push(false);
        for i in (0..length).rev() {
            bits.push(!bits[i]);
        }
    }
    bits.truncate(disklen);
    loop {
        bits = bits.into_iter().tuples().map(|(a, b)| a == b).collect();
        if bits.len() % 2 == 1 {
            break;
        }
    }
    bits.into_iter()
        .map(|b| if b { '1' } else { '0' })
        .collect()
}

fn solve(input: Input) -> String {
    dragon(input.read().trim(), 272)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(dragon("10000", 20), "01100");
    }
}
