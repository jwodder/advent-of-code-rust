use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::{HashMap, HashSet};

type Step = char;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Precondition {
    before: Step,
    after: Step,
}

impl std::str::FromStr for Precondition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Precondition, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Step ")?;
        let before = parser.parse_to::<Step, _>(Token::Whitespace)?;
        parser.skip("must be finished before step ")?;
        let after = parser.parse_to::<Step, _>(Token::Whitespace)?;
        parser.skip("can begin.")?;
        parser.eof()?;
        Ok(Precondition { before, after })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorkTree {
    preconds: HashMap<Step, HashSet<Step>>,
    in_progress: HashSet<Step>,
    done: HashSet<Step>,
}

impl WorkTree {
    fn mark_done(&mut self, s: Step) {
        self.in_progress.remove(&s);
        self.done.insert(s);
    }

    fn next_step(&mut self) -> Option<Step> {
        let c = self
            .preconds
            .iter()
            .filter_map(|(&c, needed)| needed.is_subset(&self.done).then_some(c))
            .min()?;
        self.preconds.remove(&c);
        self.in_progress.insert(c);
        Some(c)
    }

    fn done(&self) -> bool {
        self.preconds.is_empty() && self.in_progress.is_empty()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Job {
    step: Step,
    time_left: u32,
}

fn solve(input: Input, worker_qty: usize, base_time: u32) -> u32 {
    let mut preconds: HashMap<Step, HashSet<Step>> = HashMap::new();
    for p in input.parse_lines::<Precondition>() {
        preconds.entry(p.after).or_default().insert(p.before);
        preconds.entry(p.before).or_default();
    }
    let mut worktree = WorkTree {
        preconds,
        in_progress: HashSet::new(),
        done: HashSet::new(),
    };
    let mut workers: Vec<Option<Job>> = vec![None; worker_qty];
    let mut time = 0;
    while !worktree.done() {
        for work in &mut workers {
            if let Some(w) = work {
                w.time_left -= 1;
                if w.time_left == 0 {
                    worktree.mark_done(w.step);
                    *work = None;
                }
            }
        }
        for work in &mut workers {
            if work.is_none()
                && let Some(step) = worktree.next_step()
            {
                let time_left = u32::from(step) - 0x40 + base_time;
                *work = Some(Job { step, time_left });
            }
        }
        time += 1;
    }
    time - 1
}

fn main() {
    println!("{}", solve(Input::from_env(), 5, 60));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "Step C must be finished before step A can begin.\n",
            "Step C must be finished before step F can begin.\n",
            "Step A must be finished before step B can begin.\n",
            "Step A must be finished before step D can begin.\n",
            "Step B must be finished before step E can begin.\n",
            "Step D must be finished before step E can begin.\n",
            "Step F must be finished before step E can begin.\n",
        ));
        assert_eq!(solve(input, 2, 0), 15);
    }
}
