pub mod counter;
pub mod grid;
pub mod index;
pub mod pullparser;
use either::Either;
use std::fs::File;
use std::io::{self, read_to_string, stdin, BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Input {
    Stdin,
    File(PathBuf),
}

impl Input {
    pub fn from_env() -> Input {
        std::env::args_os()
            .nth(1)
            .map(|s| Input::File(s.into()))
            .unwrap_or(Input::Stdin)
    }

    pub fn read(self) -> String {
        match self {
            Input::Stdin => read_to_string(stdin().lock()).expect("Error reading stdin"),
            Input::File(path) => read_to_string(BufReader::new(
                File::open(path).expect("Error opening file"),
            ))
            .expect("Error reading file"),
        }
    }

    pub fn lines(self) -> Lines {
        Lines::new(match self {
            Input::Stdin => Either::Left(stdin().lines()),
            Input::File(path) => {
                Either::Right(BufReader::new(File::open(path).expect("Error opening file")).lines())
            }
        })
    }

    // Yields each paragraph with inner newlines converted to '\n' and trailing
    // newlines removed
    pub fn paragraphs(self) -> Paragraphs {
        Paragraphs::new(self.lines())
    }

    pub fn parse<T: FromStr>(self) -> T
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.read().parse::<T>().expect("Error parsing input")
    }

    pub fn parse_lines<T: FromStr>(self) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.lines()
            .map(|s| s.parse::<T>().expect("Error parsing input"))
    }

    // Assumes that the input is just one line of comma-separated values
    pub fn parse_csv_line<T: FromStr>(self) -> Vec<T>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        parse_csv(&self.read())
    }
}

type LinesInner = Either<io::Lines<io::StdinLock<'static>>, io::Lines<BufReader<File>>>;

pub struct Lines(LinesInner);

impl Lines {
    fn new(inner: LinesInner) -> Lines {
        Lines(inner)
    }
}

impl Iterator for Lines {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.0.next().map(|l| l.expect("Error reading input"))
    }
}

pub struct Paragraphs {
    inner: Lines,
    buffer: Vec<String>,
}

impl Paragraphs {
    fn new(inner: Lines) -> Paragraphs {
        Paragraphs {
            inner,
            buffer: Vec::new(),
        }
    }
}

impl Iterator for Paragraphs {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        #[allow(clippy::while_let_on_iterator)]
        while let Some(ln) = self.inner.next() {
            if ln.is_empty() {
                if !self.buffer.is_empty() {
                    let s = self.buffer.join("\n");
                    self.buffer.clear();
                    return Some(s);
                }
            } else {
                self.buffer.push(ln);
            }
        }
        if !self.buffer.is_empty() {
            let s = self.buffer.join("\n");
            self.buffer.clear();
            return Some(s);
        }
        None
    }
}

pub fn parse_csv<T: FromStr>(s: &str) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    s.trim()
        .split(',')
        .map(|t| t.parse::<T>().expect("Error parsing input"))
        .collect()
}
