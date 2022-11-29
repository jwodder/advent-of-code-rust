use adventutil::counter::Counter;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
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

impl FromStr for Event {
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

impl FromStr for Timestamp {
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

#[derive(Debug)]
enum State {
    Awake {
        guard_id: usize,
    },
    Asleep {
        guard_id: usize,
        fell_asleep: Timestamp,
    },
}

fn solve<I: IntoIterator<Item = Event>>(iter: I) -> usize {
    let mut events = iter.into_iter().collect::<Vec<_>>();
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
    let (guard_id, minute, _) = guards
        .into_iter()
        .flat_map(|(g, st)| st.into_iter().map(move |(m, qty)| (g, m, qty)))
        .reduce(|g1 @ (_, _, qty1), g2 @ (_, _, qty2)| if qty1 > qty2 { g1 } else { g2 })
        .unwrap();
    guard_id * minute
}

fn main() {
    println!("{}", solve(Input::from_env().parse_lines::<Event>()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let events = [
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ]
        .into_iter()
        .map(|s| s.parse::<Event>().unwrap());
        assert_eq!(solve(events), 4455);
    }
}
