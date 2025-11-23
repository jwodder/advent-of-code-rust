use adventutil::Input;

// Sum-of-divisors function; multiplicative, with $\sigma(p^k) = \frac{p^{k+1}
// - 1}{p - 1}$
fn sigma(n: u32) -> u32 {
    factor(n)
        .map(|(p, k)| (p.pow(k + 1) - 1) / (p - 1))
        .product()
}

fn factor(n: u32) -> Factors {
    Factors::new(n)
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
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

fn presents(houseno: u32) -> u32 {
    sigma(houseno) * 10
}

fn solve(input: Input) -> u32 {
    let target = input.parse::<u32>();
    (1..).find(|&i| presents(i) >= target).unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 10)]
    #[case(2, 30)]
    #[case(3, 40)]
    #[case(4, 70)]
    #[case(5, 60)]
    #[case(6, 120)]
    #[case(7, 80)]
    #[case(8, 150)]
    #[case(9, 130)]
    fn examples(#[case] houseno: u32, #[case] gifts: u32) {
        assert_eq!(presents(houseno), gifts);
    }
}
