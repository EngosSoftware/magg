//! # Definition of result and errors

use std::process::ExitStatus;

/// Common result type.
pub type Result<T, E = MaggError> = std::result::Result<T, E>;

/// Error definition.
#[derive(Debug, PartialEq, Eq)]
pub struct MaggError(String);

impl std::fmt::Display for MaggError {
  /// Implementation of [Display] trait for [MaggError].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl MaggError {
  /// Creates a new [MaggError] with specified error message.
  pub fn new(message: impl AsRef<str>) -> Self {
    Self(message.as_ref().to_string())
  }
}

pub fn error_spawn_command(program: impl AsRef<str>, reason: impl AsRef<str>) -> MaggError {
  MaggError::new(format!("failed to spawn command: {}, with reason: {}", program.as_ref(), reason.as_ref()))
}

pub fn error_obtain_output(reason: impl AsRef<str>) -> MaggError {
  MaggError::new(format!("failed to obtain command output with reason: {}", reason.as_ref()))
}

pub fn error_execute_command(status: ExitStatus, stdout: impl AsRef<str>, stderr: impl AsRef<str>) -> MaggError {
  MaggError::new(format!(
    "failed to execute command, status {}\nstdout:\n{}\nstderr:\n{}\n",
    status,
    stdout.as_ref(),
    stderr.as_ref()
  ))
}
