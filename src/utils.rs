use crate::errors::*;
use std::io::{self, Write};
use std::path::Path;

/// Separator line.
pub const SEPARATOR_LINE: &str = "────────────────────────────────────────────────────────────────────────────────";

pub fn read_file(file_name: impl AsRef<Path>) -> Result<String> {
  let path = file_name.as_ref();
  std::fs::read_to_string(path).map_err(|e| error_read_file(path, e))
}

pub fn write_file(file_name: impl AsRef<Path>, contents: &str) -> Result<()> {
  let file_path = file_name.as_ref();
  std::fs::write(file_path, contents).map_err(|e| MaggError::new(format!("failed to write file {}, reason: {}", file_path.display(), e)))
}

pub fn parse_toml(file_name: impl AsRef<Path>) -> Result<toml::Value> {
  toml::from_str(&read_file(file_name)?).map_err(|e| MaggError::new(e.to_string()))
}

pub fn get_package_name(parsed: &toml::Value) -> &str {
  parsed["package"]["name"].as_str().expect("package.name not found in Cargo.toml")
}
pub fn get_repository(parsed: &toml::Value) -> &str {
  parsed["package"]["repository"].as_str().expect("package.repository not found in Cargo.toml")
}

pub fn step_progress() {
  print!("·");
  io::stdout().flush().unwrap();
}

/// Executes a command with arguments and returns the content od stdout.
pub fn execute_command(verbose: bool, program: &str, args: &[&str], dir: &str) -> Result<String> {
  if verbose {
    println!("{} {}", program, args.join(" "));
  } else {
    step_progress();
  }
  let mut command = std::process::Command::new(program);
  let child = command
    .args(args)
    .current_dir(dir)
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::piped())
    .spawn()
    .map_err(|e| error_spawn_command(program, e.to_string()))?;
  let output = child.wait_with_output().map_err(|e| error_obtain_output(e.to_string()))?;
  let stdout = String::from_utf8_lossy(&output.stdout).to_string();
  let stderr = String::from_utf8_lossy(&output.stderr).to_string();
  let status = output.status;
  if status.success() {
    Ok(stdout)
  } else {
    Err(error_execute_command(status, stdout, stderr))
  }
}

pub fn parse_columns(input: String, col_count: usize) -> Result<Vec<Vec<String>>> {
  let mut rows = vec![];
  for mut line in input.lines().map(|line| line.trim()) {
    if line.starts_with("\"") {
      line = line.strip_prefix("\"").unwrap();
    }
    if line.starts_with("'") {
      line = line.strip_prefix("'").unwrap();
    }
    if line.ends_with("\"") {
      line = line.strip_suffix("\"").unwrap();
    }
    if line.ends_with("'") {
      line = line.strip_suffix("'").unwrap();
    }
    line = line.trim();
    if !line.is_empty() {
      let columns = line.split(" ||| ").map(|s| s.to_string()).collect::<Vec<String>>();
      if columns.len() != col_count {
        return Err(MaggError::new(format!("invalid number of columns, expected: {col_count}, actual: {}", columns.len())));
      }
      rows.push(columns);
    }
  }
  Ok(rows)
}
