use crate::cli::subcommand::new::New;
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
}

impl SubCommand {
  pub fn run(&self) {
    match self {
      SubCommand::New(new_command) => new_command.run(),
    }
  }
}
