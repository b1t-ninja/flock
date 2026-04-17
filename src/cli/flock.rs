use crate::cli::subcommand::{add::Add, build::Build, new::New, run::Run, test::Test};
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
  Test(Test),
  Add(Add)
}

impl SubCommand {
  pub fn run(&self) {
    match self {
      SubCommand::New(new_command) => new_command.run(),
      SubCommand::Run(run_command) => run_command.run(),
      SubCommand::Build(build_command) => build_command.run(),
      SubCommand::Test(test_command) => test_command.run(),
      SubCommand::Add(add_command) => add_command.run(),
    }
  }
}
