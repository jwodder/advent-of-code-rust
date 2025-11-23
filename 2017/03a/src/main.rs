use adventutil::Input;

/// Returns `(layer, layer_floor)` where:
///
/// - `layer` is the zero-indexed layer of the spiral that `n` is on, counting
///   from the center.
///
///     - 1 is on layer 0.
///     - 2 through 9 are on layer 1.
///     - 10 through 25 are on layer 2.
///     - etc.
///
/// - `layer_floor` is the largest value on the layer before `layer`.  (The
///   value when `layer == 0` is irrelevant.)
fn split_layer(n: u32) -> (u32, u32) {
    let mut layer = 0;
    let mut layer_floor = 0;
    loop {
        let next_floor = layer_floor + (if layer == 0 { 1 } else { layer * 8 });
        if next_floor >= n {
            return (layer, layer_floor);
        }
        layer += 1;
        layer_floor = next_floor;
    }
}

fn step_count(n: u32) -> u32 {
    let (layer, layer_floor) = split_layer(n);
    if layer == 0 {
        0
    } else {
        let rem = n - layer_floor;
        let sidepos = rem % (layer * 2);
        layer + sidepos.abs_diff(layer)
    }
}

fn solve(input: Input) -> u32 {
    step_count(input.parse::<u32>())
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 0)]
    #[case(12, 3)]
    #[case(23, 2)]
    #[case(1024, 31)]
    fn examples(#[case] n: u32, #[case] steps: u32) {
        assert_eq!(step_count(n), steps);
    }

    #[rstest]
    #[case(3, 2)]
    #[case(5, 2)]
    #[case(7, 2)]
    #[case(9, 2)]
    #[case(25, 4)]
    fn my_examples(#[case] n: u32, #[case] steps: u32) {
        assert_eq!(step_count(n), steps);
    }

    #[rstest]
    #[case(1, 0, 0)]
    #[case(2, 1, 1)]
    #[case(3, 1, 1)]
    #[case(9, 1, 1)]
    #[case(10, 2, 9)]
    #[case(17, 2, 9)]
    #[case(25, 2, 9)]
    fn split_layer(#[case] n: u32, #[case] layer: u32, #[case] layer_floor: u32) {
        assert_eq!(super::split_layer(n), (layer, layer_floor));
    }
}
