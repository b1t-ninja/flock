use std::{
  env,
  fs::{self, File},
  io::{BufRead, BufReader},
  path::PathBuf,
};

use crate::cli::util::dependency::Dependency;

pub struct DependencyManager {
  path_to_package_swift_file: PathBuf,
  project_name: String,
}

impl DependencyManager {
  pub fn new(project_name: String) -> DependencyManager {
    Self {
      project_name,
      path_to_package_swift_file: env::current_dir().expect("Failed to grab the current dir"),
    }
  }
}

impl DependencyManager {
  //7
  //12
  //
  //are the lines where dependencies shall be inserted
  //
  pub fn add_dependency(&self, dep: Dependency) {
    let path = &self.path_to_package_swift_file.join("Package.swift");

    dbg!("File is assumed to be at {}", path);

    let file = File::open(&path).unwrap();
    let buf_reader = BufReader::new(file);
    let mut out_lines = Vec::new();

    for line in buf_reader.lines() {
      let ln = line.expect("Failed to read line");
      if ln.trim_start().starts_with("//7") {
        let new_line = format!("{},\n{},", ln, &dep.package);
        out_lines.push(new_line);
      } else if ln.trim_start().starts_with("//12") {
        let new_line = format!("{},\n{},", ln, &dep.product);
        out_lines.push(new_line);
      } else {
        out_lines.push(ln);
      }
    }
    let new_content = out_lines.join("\n");
    fs::write(path, new_content).expect("Failed to write to File");
  }

  fn add_dependecy_slots(&self, path: PathBuf) {
    let mut count = 0; // three name sections, but we only want to add to two, 
    // testing deps later
    let file = File::open(&path).unwrap();
    let buf_reader = BufReader::new(file);
    let mut out_lines = Vec::new();

    for line in buf_reader.lines() {
      count += 1;
      let ln = line.expect("Failed to read line");
      let ln = self.add_slots(ln, count);
      out_lines.push(ln);
    }

    let new_content = out_lines.join("\n");
    fs::write(path, new_content).expect("Failed to write to File");
  }

  fn add_initial_commas(&self, path: PathBuf) {
    // grab any name: "stuff"
    // append , after the name if it doesnt have , already
    let file = File::open(&path).unwrap();
    let buf_reader = BufReader::new(file);

    let mut out_lines = Vec::new();

    for line in buf_reader.lines() {
      let ln = line.expect("Failed to read line");
      let ln = self.add_comma_to_name_line_if_not_yet_there(ln);
      out_lines.push(ln);
    }

    let new_content = out_lines.join("\n");
    fs::write(path, new_content).expect("Failed to write to File");
  }

  fn add_comma_to_name_line_if_not_yet_there(&self, line: String) -> String {
    let trimmed_left = line.trim_start();

    let is_relevant_line = trimmed_left.starts_with("name:");
    let line_ends_with_comma = line.trim_end().ends_with(',');

    if is_relevant_line && !line_ends_with_comma {
      format!("{},", line.trim_end())
    } else {
      line
    }
  }

  fn add_slots(&self, line: String, count: i32) -> String {
    let dep_slot = format!(
      r#"
    dependencies: [
//{}
    ],
      "#,
      count
    );
    let trimmed_left = line.trim_start();

    let is_relevant_line = trimmed_left.starts_with("name:");
    if is_relevant_line && count <= 12 {
      // we have done all relevant lines
      format!("{}\n{}", line, dep_slot)
    } else {
      line
    }
  }

  pub fn setup(&self) {
    let path = &self
      .path_to_package_swift_file
      .join(&self.project_name)
      .join("Package.swift");
    dbg!("File is assumed to be at {}", path);
    self.add_initial_commas(path.clone());
    self.add_dependecy_slots(path.clone());
  }
}

/*
 *
// swift-tools-version: 6.3
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "MyCLI",
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .executableTarget(
            name: "MyCLI"
        ),
        .testTarget(
            name: "MyCLITests",
            dependencies: ["MyCLI"]
        ),
    ],
    swiftLanguageModes: [.v6]
)
 *
 *
 *
 *
 *
 *
// swift-tools-version: 6.3
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "spm",


  dependencies: [
    .package(
      url: "https://github.com/apple/swift-argument-parser",
      from:
        "1.7.0")
  ],

  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .executableTarget(
      name: "spm",

      dependencies: [
        .product(name: "ArgumentParser", package: "swifddt-argument-parser"),
      ]
    ),

    .testTarget(
      name: "spmTests",
      dependencies: ["spm"]
    ),

  ],

  swiftLanguageModes: [.v6]
)
*
 *
 *
*/
