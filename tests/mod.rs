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

#[cfg(not(target_os = "windows"))]
fn normalize_exe(s: &str) -> String {
  s.replace("{{EXE}}", "")
}

#[cfg(target_os = "windows")]
fn normalize_exe(s: &str) -> String {
  s.replace("{{EXE}}", ".exe")
}

#[cfg(not(target_os = "windows"))]
fn normalize_r(s: &str) -> String {
  s.replace("{{R}}", "")
}

#[cfg(target_os = "windows")]
fn normalize_r(s: &str) -> String {
  s.replace("{{R}}", "\r")
}
