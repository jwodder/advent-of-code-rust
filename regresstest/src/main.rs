use anyhow::{bail, Context};
use futures::stream::{iter, StreamExt};
use serde::Deserialize;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
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

struct TestCase {
    year: i32,
    problem: String,
    input: String,
    answer: String,
}

impl TestCase {
    fn id(&self) -> String {
        format!("{}-{}", self.year, self.problem)
    }

    async fn run(self) -> bool {
        log::info!("RUNNING: {}", self.id());
        let mut cmd = Command::new("cargo");
        cmd.arg("run")
            .arg("-q")
            .arg("-r")
            .arg("-p")
            .arg(format!("advent-of-code-{}-{}", self.year, self.problem))
            .arg("--")
            .arg(format!("{}/inputs/{}", self.year, self.input))
            .current_dir(workspace_dir())
            .kill_on_drop(true);
        match timeout(TIMEOUT, cmd.output()).await {
            Ok(Ok(out)) => {
                if out.status.success() {
                    match String::from_utf8(out.stdout) {
                        Ok(s) => {
                            if s.trim() == self.answer {
                                log::info!("PASS: {}", self.id());
                                true
                            } else {
                                log::error!("FAIL: {}", self.id());
                                false
                            }
                        }
                        Err(_) => {
                            log::info!("Problem {} binary emitted non-UTF-8", self.id());
                            false
                        }
                    }
                } else {
                    log::error!("Problem {} binary failed: {}", self.id(), out.status);
                    // TODO: Display stderr?
                    false
                }
            }
            Ok(Err(e)) => {
                log::error!("Problem {} binary failed to execute: {}", self.id(), e);
                false
            }
            Err(_) => {
                log::error!("TIMEOUT: {}", self.id());
                false
            }
        }
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
    let diriter = read_dir(workspace_dir()).context("failed to read workspace directory")?;
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
                    year,
                    problem: row.problem,
                    input: row.input,
                    answer: row.answer,
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

fn workspace_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR lacks parent path")
        .into()
}
