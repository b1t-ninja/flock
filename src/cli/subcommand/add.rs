use clap::Args;

/// Adds a new dependency
#[derive(Debug, Args)]
pub struct Add {
  /// The name of the dependency
  pub name: String,

  /// The version of the dependency, defaults to latest
  #[arg(short, long)]
  pub version: Option<String>,
}

impl Add {
  pub fn run(&self) {
    if let Some(ver) = &self.version {

    }
    // fetch dep package and product
    // add it to the Package.swift file
    todo!()
  }
}
