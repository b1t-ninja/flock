use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dependency {
  package: String,
  product: String,
}
