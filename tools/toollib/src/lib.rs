use serde::Deserialize;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};
use thiserror::Error;

pub fn project_root() -> Result<PathBuf, LocateError> {
    match Command::new("cargo")
        .arg("locate-project")
        .arg("--workspace")
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(output) if output.status.success() => {
            match serde_json::from_slice::<LocateProject>(&output.stdout) {
                Ok(location) => {
                    if !location.root.is_absolute() {
                        return Err(LocateError::InvalidPath(location.root));
                    }
                    if let Some(root) = location.root.parent() {
                        Ok(root.to_owned())
                    } else {
                        Err(LocateError::InvalidPath(location.root))
                    }
                }
                Err(e) => Err(LocateError::Deserialize(e)),
            }
        }
        Ok(output) => Err(LocateError::Exit(output.status)),
        Err(e) => Err(LocateError::Startup(e)),
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
struct LocateProject {
    root: PathBuf,
}

#[derive(Debug, Error)]
pub enum LocateError {
    #[error("failed to run `cargo locate-project`")]
    Startup(#[source] std::io::Error),
    #[error("command `cargo locate-project` failed: {0}")]
    Exit(ExitStatus),
    #[error("could not deserialize `cargo locate-project` output")]
    Deserialize(#[from] serde_json::Error),
    #[error("manifest path is absolute or parentless: {}", .0.display())]
    InvalidPath(PathBuf),
}
