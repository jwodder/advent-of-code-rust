use serde::Deserialize;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};
use thiserror::Error;

pub fn project_root() -> Result<PathBuf, LocateError> {
    match Command::new("cargo")
        .arg("locate-project")
        .arg("--workspace")
        .stderr(Stdio::inherit())
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
    {
        Ok(output) if output.status.success() => {
            let location = serde_json::from_slice::<LocateProject>(&output.stdout)?;
            if !location.root.is_absolute() {
                return Err(LocateError::InvalidPath(location.root));
            }
            if let Some(root) = location.root.parent() {
                Ok(root.to_owned())
            } else {
                Err(LocateError::InvalidPath(location.root))
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
