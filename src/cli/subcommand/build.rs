use clap::Args;
use std::process::Command;

/// Builds a project
#[derive(Debug, Args)]
pub struct Build {
  /// Build in release mode
  #[arg(short, long)]
  pub release: bool,
}

impl Build {
  pub fn run(&self) {
    if self.release {
      let status = Command::new("swift")
        .arg("-c")
        .arg("release")
        .status()
        .expect("failed to execute process");

      if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
      }
    }

    let status = Command::new("swift")
      .arg("build")
      .status()
      .expect("failed to execute process");

    if !status.success() {
      std::process::exit(status.code().unwrap_or(1));
    }
  }
}
