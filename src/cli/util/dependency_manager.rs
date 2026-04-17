use std::{fs, path::PathBuf};

pub struct DependencyManager {
  cwd: PathBuf,
}

impl DependencyManager {
  pub fn read_package_swift_file(&self) -> String {
    let path = self.cwd.join("Package.swift");
    fs::read_to_string(path).expect("Failed to read the file")
  }
}

