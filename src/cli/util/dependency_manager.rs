use std::{env, fs, path::PathBuf};

pub struct DependencyManager {
  path_to_package_swift_file: PathBuf,
}

impl DependencyManager {
  fn read_package_swift_file(&self) -> String {
    fs::read_to_string(&self.path_to_package_swift_file).expect("Failed to read the file")
  }

  fn write_package_swift_file(&self, new_content: String) {
    fs::write(&self.path_to_package_swift_file, new_content).expect("Failed to write to file")
  }
  fn add_initial_dependency_section(&self) {
    let dependency_text = "dependencies: [],";
  }
  pub fn setup(&self) {
    self.add_initial_dependency_section();
  }
}

impl DependencyManager {
  pub fn new() -> DependencyManager {
    Self {
      path_to_package_swift_file: env::current_dir()
        .expect("Failed to grab the current dir")
        .join("Package.swift"),
    }
  }
}
