#![doc = include_str!("../docs/README.md")]

mod changelog;
mod cli;
mod code_of_conduct;
mod errors;
mod licenses;
mod new_cli;
mod readme;
mod utils;

pub use cli::do_action;
pub use new_cli::new_do_action;
