use adventutil::Input;
use std::collections::HashMap;

struct Population {
    // Mapping from timer values to the number of lanterfish with that value
    timers: HashMap<u32, usize>,
}

impl Population {
    fn new<I: IntoIterator<Item = u32>>(iter: I) -> Population {
        let mut timers = HashMap::new();
        for t in iter {
            *timers.entry(t).or_insert(0) += 1;
        }
        Population { timers }
    }

    fn size(&self) -> usize {
        self.timers.values().copied().sum()
    }

    fn step(self) -> Population {
        let mut timers = HashMap::new();
        for (t, qty) in self.timers {
            if t == 0 {
                *timers.entry(6).or_insert(0) += qty;
                *timers.entry(8).or_insert(0) += qty;
            } else {
                *timers.entry(t - 1).or_insert(0) += qty;
            }
        }
        Population { timers }
    }
}

fn main() {
    let pop = Population::new(
        Input::from_env()
            .read()
            .trim()
            .split(',')
            .map(|s| s.parse::<u32>().expect("Error parsing input")),
    );
    println!("{}", size_after_days(pop, 256));
}

fn size_after_days(mut pop: Population, days: usize) -> usize {
    for _ in 0..days {
        pop = pop.step();
    }
    pop.size()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1a() {
        let pop = Population::new([3, 4, 3, 1, 2]);
        assert_eq!(size_after_days(pop, 18), 26);
    }

    #[test]
    fn test_example1b() {
        let pop = Population::new([3, 4, 3, 1, 2]);
        assert_eq!(size_after_days(pop, 80), 5934);
    }

    #[test]
    fn test_example2() {
        let pop = Population::new([3, 4, 3, 1, 2]);
        assert_eq!(size_after_days(pop, 256), 26984457539);
    }
}
