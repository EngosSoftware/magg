mod decision_toolkit_crate;
mod regular_crate;

use crate::errors::Result;
use crate::utils;
use std::fmt::Write;
use std::path::Path;

/// Color of the license badge.
const LICENSE_COLOR: &str = "9370DB";

/// Color of the human badge.
const HUMAN_COLOR: &str = "F5DEB3";

pub use decision_toolkit_crate::get_readme_for_decision_toolkit_crate;
pub use regular_crate::get_readme_for_regular_crate;
