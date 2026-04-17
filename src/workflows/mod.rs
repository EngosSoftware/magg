/// Version of the `checkout` action.
const CHECKOUT_VERSION: &str = "v6";
const UPLOAD_ARTIFACT_VERSION: &str = "v7";

const BUILD_LINUX_GNU: &str = include_str!("BUILD-LINUX-GNU.txt");
const BUILD_LINUX_MUSL: &str = include_str!("BUILD-LINUX-MUSL.txt");
const BUILD_MACOS: &str = include_str!("BUILD-MACOS.txt");
const BUILD_MACOS_AARCH64: &str = include_str!("BUILD-MACOS-AARCH64.txt");
const BUILD_WINDOWS: &str = include_str!("BUILD-WINDOWS.txt");

/// Returns the content of the GitHub workflow for GNU/Linux x86_64 platform.
pub fn get_build_linux_gnu() -> String {
  replace_variables(BUILD_LINUX_GNU)
}

/// Returns the content of the GitHub workflow for Linux musl x86_64 platform.
pub fn get_build_linux_musl() -> String {
  replace_variables(BUILD_LINUX_MUSL)
}

/// Returns the content of the GitHub workflow for macOS x86_64 platform.
pub fn get_build_macos() -> String {
  replace_variables(BUILD_MACOS)
}

/// Returns the content of the GitHub workflow for macOS aarch64 platform.
pub fn get_build_macos_aarch64() -> String {
  replace_variables(BUILD_MACOS_AARCH64)
}

/// Returns the content of the GitHub workflow for Windows x86_64 platform.
pub fn get_build_windows() -> String {
  replace_variables(BUILD_WINDOWS)
}

/// Replaces all variables in specified input.
fn replace_variables(input: &str) -> String {
  input
    .replace("{{CHECKOUT-VERSION}}", CHECKOUT_VERSION)
    .replace("{{UPLOAD-ARTIFACT-VERSION}}", UPLOAD_ARTIFACT_VERSION)
}
