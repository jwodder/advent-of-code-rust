use adventutil::Input;
use itertools::Itertools;

fn factor(n: u32) -> Factors {
    Factors::new(n)
}

struct Factors {
    n: u32,
    inner: Primeish,
}

impl Factors {
    fn new(n: u32) -> Factors {
        Factors {
            n,
            inner: Primeish::new(),
        }
    }
}

impl Iterator for Factors {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.n == 1 {
            return None;
        }
        loop {
            let p = self.inner.next().unwrap();
            let mut k = 0;
            while self.n.is_multiple_of(p) {
                self.n /= p;
                k += 1;
            }
            if k > 0 {
                return Some((p, k));
            } else if p * p > self.n {
                let r = Some((self.n, 1));
                self.n = 1;
                return r;
            }
        }
    }
}

struct Primeish(u32);

impl Primeish {
    fn new() -> Primeish {
        Primeish(2)
    }
}

impl Iterator for Primeish {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.0;
        if n == 2 {
            self.0 = 3;
        } else {
            self.0 += 2;
        }
        Some(n)
    }
}

fn powers(p: u32, k: u32) -> Powers {
    Powers::new(p, k)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Powers {
    base: u32,
    accum: u32,
    left: u32,
}

impl Powers {
    fn new(base: u32, max_exp: u32) -> Powers {
        Powers {
            base,
            accum: 1,
            left: max_exp + 1,
        }
    }
}

impl Iterator for Powers {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.left == 0 {
            return None;
        }
        let r = Some(self.accum);
        self.accum *= self.base;
        self.left -= 1;
        r
    }
}

fn divisors(n: u32) -> Vec<u32> {
    let mut divs = vec![1];
    for (p, k) in factor(n) {
        divs = divs
            .into_iter()
            .cartesian_product(powers(p, k))
            .map(|(x, y)| x * y)
            .collect::<Vec<_>>();
    }
    divs
}

fn presents(houseno: u32) -> u32 {
    divisors(houseno)
        .into_iter()
        .filter(|n| houseno <= n * 50)
        .sum::<u32>()
        * 11
}

fn solve(input: Input) -> usize {
    let target = input.parse::<u32>();
    (1..).position(|i| presents(i) >= target).unwrap() + 1
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powers() {
        assert_eq!(Powers::new(2, 5).collect::<Vec<_>>(), [1, 2, 4, 8, 16, 32]);
    }
}
