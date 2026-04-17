use clap::Args;
use std::process::Command;

/// Tests a project
#[derive(Debug, Args)]
pub struct Test;

impl Test {
  pub fn run(&self) {
    let status = Command::new("swift")
      .arg("test")
      .status()
      .expect("failed to execute process");

    if !status.success() {
      std::process::exit(status.code().unwrap_or(1));
    }
  }
}
