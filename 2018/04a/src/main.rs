use adventutil::Input;
use adventutil::counter::Counter;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Event {
    BeginShift {
        timestamp: Timestamp,
        guard_id: usize,
    },
    FallAsleep {
        timestamp: Timestamp,
    },
    WakeUp {
        timestamp: Timestamp,
    },
}

impl Event {
    fn timestamp(&self) -> Timestamp {
        match self {
            Event::BeginShift { timestamp, .. } => *timestamp,
            Event::FallAsleep { timestamp } => *timestamp,
            Event::WakeUp { timestamp } => *timestamp,
        }
    }
}

impl std::str::FromStr for Event {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Event, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip('[')?;
        let timestamp = parser.parse_to::<Timestamp, _>(']')?;
        parser.skip(Token::Whitespace)?;
        match parser.scan_to(Token::Whitespace)? {
            "Guard" => {
                parser.skip('#')?;
                let guard_id = parser.parse_to::<usize, _>(Token::Whitespace)?;
                parser.skip("begins shift")?;
                parser.eof()?;
                Ok(Event::BeginShift {
                    timestamp,
                    guard_id,
                })
            }
            "falls" => {
                parser.skip("asleep")?;
                parser.eof()?;
                Ok(Event::FallAsleep { timestamp })
            }
            "wakes" => {
                parser.skip("up")?;
                parser.eof()?;
                Ok(Event::WakeUp { timestamp })
            }
            s => Err(ParseError::InvalidToken(s.into())),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Timestamp {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl std::str::FromStr for Timestamp {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Timestamp, ParseError> {
        let mut parser = PullParser::new(s);
        let year = parser.parse_to::<usize, _>('-')?;
        let month = parser.parse_to::<usize, _>('-')?;
        let day = parser.parse_to::<usize, _>(Token::Whitespace)?;
        let hour = parser.parse_to::<usize, _>(':')?;
        let minute = parser.parse_to::<usize, _>(Token::Eof)?;
        Ok(Timestamp {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Awake {
        guard_id: usize,
    },
    Asleep {
        guard_id: usize,
        fell_asleep: Timestamp,
    },
}

fn solve(input: Input) -> usize {
    let mut events = input.parse_lines::<Event>().collect::<Vec<_>>();
    events.sort_by_key(Event::timestamp);
    let mut iter = events.into_iter();
    let mut state = match iter.next() {
        Some(Event::BeginShift { guard_id, .. }) => State::Awake { guard_id },
        Some(e) => panic!("First event is unexpected {e:?}"),
        None => panic!("No events supplied"),
    };
    let mut guards: HashMap<usize, Counter<usize>> = HashMap::new();
    for e in iter {
        state = match (state, e) {
            (State::Awake { .. }, Event::BeginShift { guard_id, .. }) => State::Awake { guard_id },
            (State::Awake { guard_id }, Event::FallAsleep { timestamp }) => State::Asleep {
                guard_id,
                fell_asleep: timestamp,
            },
            (
                State::Asleep {
                    guard_id,
                    fell_asleep,
                },
                Event::WakeUp { timestamp },
            ) => {
                guards
                    .entry(guard_id)
                    .or_default()
                    .extend(fell_asleep.minute..timestamp.minute);
                State::Awake { guard_id }
            }
            (state, e) => panic!("Unexpected event {e:?} in state {state:?}"),
        }
    }
    let (guard_id, sleepytimes) = guards.into_iter().max_by_key(|(_, st)| st.total()).unwrap();
    let (minute, _) = sleepytimes.into_iter().max_by_key(|&(_, st)| st).unwrap();
    guard_id * minute
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "[1518-11-01 00:00] Guard #10 begins shift\n",
            "[1518-11-01 00:05] falls asleep\n",
            "[1518-11-01 00:25] wakes up\n",
            "[1518-11-01 00:30] falls asleep\n",
            "[1518-11-01 00:55] wakes up\n",
            "[1518-11-01 23:58] Guard #99 begins shift\n",
            "[1518-11-02 00:40] falls asleep\n",
            "[1518-11-02 00:50] wakes up\n",
            "[1518-11-03 00:05] Guard #10 begins shift\n",
            "[1518-11-03 00:24] falls asleep\n",
            "[1518-11-03 00:29] wakes up\n",
            "[1518-11-04 00:02] Guard #99 begins shift\n",
            "[1518-11-04 00:36] falls asleep\n",
            "[1518-11-04 00:46] wakes up\n",
            "[1518-11-05 00:03] Guard #99 begins shift\n",
            "[1518-11-05 00:45] falls asleep\n",
            "[1518-11-05 00:55] wakes up\n",
        ));
        assert_eq!(solve(input), 240);
    }
}
