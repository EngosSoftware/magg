mod test_cli;

#[cfg(not(target_os = "windows"))]
fn normalize_exe(s: &str) -> String {
  s.replace("||EXE||", "")
}

#[cfg(target_os = "windows")]
fn normalize_exe(s: &str) -> String {
  s.replace("||EXE||", ".exe")
}
