use anyhow::{bail, Context};
use futures::stream::{iter, StreamExt};
use serde::Deserialize;
use std::fmt;
use std::fs::read_dir;
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

const WORKERS: usize = 8;
const TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Answer {
    problem: String,
    input: String,
    answer: String,
}

struct TestCase<'a> {
    workspace_dir: &'a Path,
    year: i32,
    answer: Answer,
}

impl<'a> TestCase<'a> {
    async fn run(self) -> bool {
        log::info!("RUNNING: {self}");
        let mut cmd = Command::new("cargo");
        cmd.arg("run")
            .arg("-q")
            .arg("-r")
            .arg("-p")
            .arg(format!(
                "advent-of-code-{}-{}",
                self.year, self.answer.problem
            ))
            .arg("--")
            .arg(format!("{}/inputs/{}", self.year, self.answer.input))
            .current_dir(self.workspace_dir)
            .kill_on_drop(true);
        match timeout(TIMEOUT, cmd.output()).await {
            Ok(Ok(out)) => {
                if out.status.success() {
                    match String::from_utf8(out.stdout) {
                        Ok(s) => {
                            if s.trim() == self.answer.answer {
                                log::info!("PASS: {self}");
                                true
                            } else {
                                log::error!("FAIL: {self}");
                                false
                            }
                        }
                        Err(_) => {
                            log::info!("Problem {self} binary emitted non-UTF-8");
                            false
                        }
                    }
                } else {
                    log::error!("Problem {} binary failed: {}", self, out.status);
                    // TODO: Display stderr?
                    false
                }
            }
            Ok(Err(e)) => {
                log::error!("Problem {} binary failed to execute: {}", self, e);
                false
            }
            Err(_) => {
                log::error!("TIMEOUT: {self}");
                false
            }
        }
    }
}

impl<'a> fmt::Display for TestCase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.year, self.answer.problem)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!("[{:<5}] {}", record.level(), message))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stderr())
        .apply()
        .unwrap();
    let mut cases = Vec::new();
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR lacks parent path")
        .to_owned();
    let diriter = read_dir(&workspace_dir).context("failed to read workspace directory")?;
    for entry in diriter {
        let entry = entry.context("failed reading workspace directory")?;
        let answerpath = entry.path().join("answers.csv");
        if entry
            .file_type()
            .context("could not get entry filetype")?
            .is_dir()
            && answerpath.exists()
        {
            let year = match entry.file_name().into_string() {
                Ok(s) => match s.parse::<i32>() {
                    Ok(year) => year,
                    Err(_) => bail!("Found answers.csv in non-year directory {s:?}"),
                },
                Err(oss) => bail!(
                    "Found answers.csv in directory with undecodable name {:?}",
                    oss.to_string_lossy()
                ),
            };
            log::debug!("Reading answers from {}", answerpath.display());
            let mut reader = csv::Reader::from_path(&answerpath)
                .with_context(|| format!("failed to read {}", answerpath.display()))?;
            for row in reader.deserialize::<Answer>() {
                let row = row.with_context(|| {
                    format!("failed to read entry from {}", answerpath.display())
                })?;
                cases.push(TestCase {
                    workspace_dir: &workspace_dir,
                    year,
                    answer: row,
                });
            }
        }
    }
    if iter(cases)
        .map(|c| c.run())
        .buffer_unordered(WORKERS)
        .all(|r| async move { r })
        .await
    {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}
