use adventutil::Input;

struct Partition {
    // The weights of the packages in the passenger compartment, in ascending
    // order
    passenger: Vec<u64>,
    // The weights of the rest of the packages, in ascending order
    remainder: Vec<u64>,
}

impl Partition {
    fn is_valid(&self, target_weight: u64) -> bool {
        self.passenger.iter().copied().sum::<u64>() == target_weight
            && partitionable(&self.remainder, target_weight)
    }

    fn next_rank(self) -> impl Iterator<Item = Partition> {
        let i = self
            .remainder
            .binary_search(self.passenger.last().unwrap())
            .unwrap_err();
        (i..(self.remainder.len())).map(move |j| {
            let mut passenger = self.passenger.clone();
            let mut remainder = self.remainder.clone();
            passenger.push(remainder.remove(j));
            Partition {
                passenger,
                remainder,
            }
        })
    }

    fn entanglement(&self) -> u64 {
        self.passenger.iter().copied().product()
    }
}

fn solve(input: Input) -> u64 {
    // The input weights are unique and in sorted/ascending order.
    let weights = input.parse_lines::<u64>().collect::<Vec<_>>();
    let target_weight = weights.iter().copied().sum::<u64>() / 3;
    let mut partitions = (0..(weights.len()))
        .map(|i| {
            let mut weights = weights.clone();
            let x = weights.remove(i);
            Partition {
                passenger: vec![x],
                remainder: weights,
            }
        })
        .collect::<Vec<_>>();
    while !partitions.is_empty() {
        let valid = partitions
            .iter()
            .filter(|p| p.is_valid(target_weight))
            .collect::<Vec<_>>();
        if !valid.is_empty() {
            return valid
                .into_iter()
                .map(Partition::entanglement)
                .min()
                .unwrap();
        }
        partitions = partitions
            .into_iter()
            .flat_map(Partition::next_rank)
            .collect::<Vec<_>>();
    }
    panic!("No solution found");
}

/// Tests whether the values in `values` can be partitioned into two groups
/// that each sum up to `target_weight`.
///
/// # Precondition (Not Checked)
///
/// The sum of the elements in `values` must be twice `target_weight`.
fn partitionable(values: &[u64], target_weight: u64) -> bool {
    let t = usize::try_from(target_weight).unwrap();
    let n = values.len();
    let mut tbl = vec![vec![false; t + 1]; n + 1];
    for row in &mut tbl {
        row[0] = true;
    }
    for (m, &am) in values.iter().enumerate() {
        let am = usize::try_from(am).unwrap();
        for b in 1..=t {
            if b < am {
                tbl[m + 1][b] = tbl[m][b];
            } else {
                tbl[m + 1][b] = tbl[m][b] || tbl[m][b - am];
            }
        }
    }
    tbl[n][t]
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("1\n2\n3\n4\n5\n7\n8\n9\n10\n11\n");
        assert_eq!(solve(input), 99);
    }
}
