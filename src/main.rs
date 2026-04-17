use clap::Parser;

use crate::cli::flock::Flock;

mod cli;

fn main() {
  let flock = Flock::parse();
  // You can check for the existence of subcommands, and if found use their
  // matches just as you would the top level cmd
  if let Some(command) = &flock.command {
    command.run();
  }
}
