use adventutil::Input;

const MODULUS: u64 = 2147483647;

#[derive(Clone, Debug, Eq, PartialEq)]
struct LehmerRng {
    state: u64,
    multiplier: u64,
}

impl LehmerRng {
    fn rand(&mut self) -> u64 {
        let r = (self.state * self.multiplier) % MODULUS;
        self.state = r;
        r
    }
}

impl Iterator for LehmerRng {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        Some(self.rand())
    }
}

fn solve(input: Input, pairs: usize) -> usize {
    let mut lines = input.lines();
    let seed1 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let seed2 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let rng1 = LehmerRng {
        state: seed1,
        multiplier: 16807,
    };
    let rng2 = LehmerRng {
        state: seed2,
        multiplier: 48271,
    };
    std::iter::zip(rng1, rng2)
        .take(pairs)
        .filter(|(r1, r2)| (r1 & 0xFFFF) == (r2 & 0xFFFF))
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env(), 40_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "Generator A starts with 65\n",
            "Generator B starts with 8921\n",
        ));
        assert_eq!(solve(input, 5), 1);
    }
}
