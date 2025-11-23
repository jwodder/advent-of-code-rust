use adventutil::Input;
use adventutil::counter::Counter;

#[derive(Clone, Debug, Eq, PartialEq)]
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

    fn size(&self) -> u64 {
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

fn solve(input: Input, days: usize) -> u64 {
    let mut pop = Population::new(input.parse_csv_line::<u32>());
    for _ in 0..days {
        pop = pop.step();
    }
    pop.size()
}

fn main() {
    println!("{}", solve(Input::from_env(), 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1a() {
        let input = Input::from("3,4,3,1,2");
        assert_eq!(solve(input, 18), 26);
    }

    #[test]
    fn example1b() {
        let input = Input::from("3,4,3,1,2");
        assert_eq!(solve(input, 80), 5934);
    }

    #[test]
    fn example2() {
        let input = Input::from("3,4,3,1,2");
        assert_eq!(solve(input, 256), 26984457539);
    }
}
