use adventutil::Input;
use md5::{Digest, Md5};

fn solve(input: Input) -> usize {
    let salt = input.read();
    let salt = salt.trim();
    let mut hashes = (0..).map(|i| hex::encode(Md5::digest(format!("{salt}{i}"))));
    let mut kiloblock = std::collections::VecDeque::with_capacity(1000);
    for _ in 0..1000 {
        kiloblock.push_back(hashes.next().unwrap());
    }
    let mut key_qty = 0;
    for i in 0.. {
        let candidate = kiloblock.pop_front().unwrap();
        kiloblock.push_back(hashes.next().unwrap());
        let chars = candidate.chars().collect::<Vec<_>>();
        if let Some(j) =
            (0..(chars.len() - 2)).find(|&j| chars[j] == chars[j + 1] && chars[j] == chars[j + 2])
        {
            let needle = format!("{c}{c}{c}{c}{c}", c = chars[j]);
            if kiloblock.iter().any(|hsh| hsh.contains(&needle)) {
                key_qty += 1;
                if key_qty == 64 {
                    return i;
                }
            }
        }
    }
    unreachable!()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from("abc");
        assert_eq!(solve(input), 22728);
    }
}
