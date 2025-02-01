use num_traits::int::PrimInt;
use num_traits::ops::euclid::Euclid;
use num_traits::sign::Signed;

/// `modinverse(a, n)` returns the [modular multiplicative inverse][1] of `a`
/// *modulo* `n`, i.e., the smallest positive integer `x` such that `(a *
/// x).rem_euclid(n) == 1`.  Returns `None` if `a` is not relatively prime to
/// `n` or if `n.abs() < 2`.
///
/// [1]: https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
pub fn modinverse<T: PrimInt + Euclid + Signed>(a: T, n: T) -> Option<T> {
    let (mut upper, mut uc) = (n.abs(), T::zero());
    if upper < (T::one() + T::one()) {
        return None;
    }
    let (mut lower, mut lc) = (a.rem_euclid(&upper), T::one());
    while lower > T::one() {
        (upper, uc, lower, lc) = (
            lower,
            lc,
            upper.rem_euclid(&lower),
            uc - lc * (upper / lower),
        );
    }
    lower.is_one().then_some(lc.rem_euclid(&n))
}

/// Chinese Remainder Theorem
///
/// Takes a sequence of pairs `(a, m)`, each representing a congruence `x ≡ a
/// (mod m)`, and returns a pair `(k, n)` representing the solution set of all
/// integers congruent to `k` *modulo* `n`.  Returns `None` if the `m`'s are
/// not pairwise relatively prime.
pub fn crt<T: PrimInt + Euclid + Signed, I: IntoIterator<Item = (T, T)>>(
    iter: I,
) -> Option<(T, T)> {
    let congruences = iter.into_iter().collect::<Vec<_>>();
    let mm = congruences
        .iter()
        .map(|&(_, m)| m)
        .fold(T::one(), |a, b| a * b);
    let mut total = T::zero();
    for (a, m) in congruences {
        total = total + a * (mm / m) * modinverse(mm / m, m)?;
    }
    Some((total % mm, mm))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(3, 5, Some(2))]
    #[case(-2, 5, Some(2))]
    #[case(3, -5, Some(2))]
    #[case(-2, -5, Some(2))]
    #[case(8, 5, Some(2))]
    #[case(1, 5, Some(1))]
    #[case(2, 6, None)]
    #[case(0, 3, None)]
    #[case(5, 1, None)]
    #[case(5, 0, None)]
    fn test_modinverse(#[case] a: i32, #[case] n: i32, #[case] inv: Option<i32>) {
        assert_eq!(modinverse(a, n), inv);
    }
}
