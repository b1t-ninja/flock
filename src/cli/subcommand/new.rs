use clap::Args;
use std::process::Command;

use crate::cli::util::dependency_manager::DependencyManager;

/// Creates a new project
#[derive(Debug, Args)]
pub struct New {
  /// The name of the project
  pub name: String,
  /// Make the project a library
  #[arg(short, long)]
  pub library: bool,
}

impl New {
  pub fn run(&self) {
    let project_type = match self.library {
      true => "library",
      false => "executable",
    };

    std::fs::create_dir(&self.name).expect("failed to create folder");

    let status = Command::new("swift")
      .arg("package")
      .arg("init")
      .arg("--name")
      .arg(&self.name)
      .arg("--type")
      .arg(project_type)
      .current_dir(&self.name)
      .status()
      .expect("failed to execute process");

    if !status.success() {
      std::process::exit(status.code().unwrap_or(1));
    }

    // sets up the initial dependency sections in Package.swift
    let manager = DependencyManager::new(self.name.clone());
    manager.setup();
  }
}
