use adventutil::Input;

fn hash(s: &str) -> u32 {
    let mut r = 0;
    for c in s.chars() {
        r = (r + u32::from(c)) * 17 % 256;
    }
    r
}

fn solve(input: Input) -> u32 {
    input.read().trim().split(',').map(hash).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("HASH");
        assert_eq!(solve(input), 52);
    }

    #[test]
    fn example2() {
        let input = Input::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(solve(input), 1320);
    }
}
