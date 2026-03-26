#![doc = include_str!("../docs/README.md")]

mod changelog;
mod cli;
mod code_of_conduct;
mod errors;
mod licenses;
mod model;
mod project_report;
mod readme;
mod utils;
mod workflows;

pub use cli::do_action;
