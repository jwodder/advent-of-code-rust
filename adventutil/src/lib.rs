pub mod closure;
pub mod counter;
pub mod grid;
pub mod index;
pub mod maxn;
pub mod pullparser;
use std::fs::File;
use std::io::{self, read_to_string, stdin, BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Input {
    Stdin,
    File(PathBuf),
    Str(&'static str),
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
            Input::Str(s) => s.to_string(),
        }
    }

    pub fn lines(self) -> Lines {
        match self {
            Input::Stdin => Lines::Stdin(stdin().lines()),
            Input::File(path) => {
                Lines::File(BufReader::new(File::open(path).expect("Error opening file")).lines())
            }
            Input::Str(s) => Lines::Str(s.lines()),
        }
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
        match self {
            Input::Str(s) => s.parse::<T>(),
            input => input.read().trim().parse::<T>(),
        }
        .expect("Error parsing input")
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

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Input {
        Input::Str(s)
    }
}

pub enum Lines {
    Stdin(io::Lines<io::StdinLock<'static>>),
    File(io::Lines<BufReader<File>>),
    Str(std::str::Lines<'static>),
}

impl Iterator for Lines {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self {
            Lines::Stdin(i) => i.next().map(|r| r.expect("Error reading input")),
            Lines::File(i) => i.next().map(|r| r.expect("Error reading input")),
            Lines::Str(i) => i.next().map(|s| s.to_string()),
        }
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
