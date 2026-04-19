use clap::Args;

use crate::cli::util::{dependency::Dependency, dependency_manager::DependencyManager};

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
    if let Some(ver) = &self.version {}

    let dep_name = "test_dep";
    let dep = Dependency {
      package: String::from(
        r#".package(
      url: "https://github.com/apple/swift-argument-parser",
      from:
        "1.7.0")"#,
      ),
      product: String::from(
        r#" .product(name: "ArgumentParser", package: "swift-argument-parser")"#,
      ),
    };

    let manager = DependencyManager::new(String::from(dep_name));
    manager.add_dependency(dep);

    // fetch dep package and product
    // add it to the Package.swift file
  }
}
