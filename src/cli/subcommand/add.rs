use clap::Args;

/// Adds a new dependency
#[derive(Debug, Args)]
pub struct Add {
  /// The name of the dependency
  pub name: String,

  /// The minimum version of the dependency
  #[arg(short, long)]
  pub version: String,
}

impl Add {
  // Creates a new folder,
  // and ceates a new swift project
  // in that folder
  pub fn run(&self) {
    // fetch dep package and product
    // add it to the Package.swift file
    todo!()
  }
}
