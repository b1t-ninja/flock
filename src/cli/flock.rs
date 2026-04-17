use std::str::Bytes;

use crate::cli::subcommand::{build::Build, new::New, run::Run};
use clap::{Parser, Subcommand};

/// Handle Swift Projects with ease ❤
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Flock {
  #[command(subcommand)]
  pub command: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
  /// Creates a new project
  New(New),
  Run(Run),
  Build(Build),
}

impl SubCommand {
  pub fn run(&self) {
    match self {
      SubCommand::New(new_command) => new_command.run(),
      SubCommand::Run(new_command) => new_command.run(),
      SubCommand::Build(new_command) => new_command.run(),
    }
  }
}
