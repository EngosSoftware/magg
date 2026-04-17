mod crate_decision_toolkit;
mod crate_regular;

use crate::errors::*;
use crate::utils;
use std::fmt::Write;
use std::path::Path;

/// Two consecutive spaces.
const TWO_SPACES: &str = "  ";

/// Color of the license badge.
const LICENSE_COLOR: &str = "4169E1";

/// Color of the human badge.
const HUMAN_COLOR: &str = "DC143C";

/// Color of the company badge.
const ENGOS_COLOR: &str = "32CD32";

pub use crate_decision_toolkit::get_readme_for_decision_toolkit_crate;
pub use crate_regular::get_readme_for_regular_crate;
