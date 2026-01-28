mod test_cli;
mod test_publisher;

#[cfg(not(target_os = "windows"))]
fn normalize(s: &str) -> String {
  s.to_string()
}

#[cfg(target_os = "windows")]
fn normalize(s: &str) -> String {
  s.replace("\n", "\r\n")
}
