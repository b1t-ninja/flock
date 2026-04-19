use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
  pub package: String,
  pub product: String,
}
