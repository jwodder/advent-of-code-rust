use adventutil::counter::Counter;
use adventutil::Input;

struct Population {
    // Counts of the number of lanternfish with each timer value
    timers: Counter<u32>,
}

impl Population {
    fn new<I: IntoIterator<Item = u32>>(iter: I) -> Population {
        Population {
            timers: iter.into_iter().collect(),
        }
    }

    fn size(&self) -> usize {
        self.timers.total()
    }

    fn step(self) -> Population {
        let mut timers = Counter::new();
        for (t, qty) in self.timers {
            if t == 0 {
                timers.add_qty(6, qty);
                timers.add_qty(8, qty);
            } else {
                timers.add_qty(t - 1, qty);
            }
        }
        Population { timers }
    }
}

fn main() {
    let pop = Population::new(Input::from_env().parse_csv_line::<u32>());
    println!("{}", size_after_days(pop, 80));
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
}
