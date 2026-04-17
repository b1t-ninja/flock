use clap::Args;
use std::process::Command;

/// Runs a project
#[derive(Debug, Args)]
pub struct Run;

impl Run {
  pub fn run(&self) {
    let status = Command::new("swift")
      .arg("run")
      .status()
      .expect("failed to execute process");

    if !status.success() {
      std::process::exit(status.code().unwrap_or(1));
    }
  }
}
