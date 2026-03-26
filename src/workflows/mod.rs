const CHECKOUT_VERSION: &str = "v6";

const BUILD_LINUX: &str = include_str!("BUILD-LINUX.txt");
const BUILD_MACOS: &str = include_str!("BUILD-MACOS.txt");
const BUILD_MACOS_ARM64: &str = include_str!("BUILD-MACOS-ARM64.txt");
const BUILD_WINDOWS: &str = include_str!("BUILD-WINDOWS.txt");

/// Returns the content of the GitHub workflow for Linux platform.
pub fn get_build_linux() -> String {
  BUILD_LINUX.replace("{{CHECKOUT_VERSION}}", CHECKOUT_VERSION)
}

/// Returns the content of the GitHub workflow for macOS platform (x86_64).
pub fn get_build_macos() -> String {
  BUILD_MACOS.replace("{{CHECKOUT_VERSION}}", CHECKOUT_VERSION)
}

/// Returns the content of the GitHub workflow for macOS platform (arm64).
pub fn get_build_macos_arm64() -> String {
  BUILD_MACOS_ARM64.replace("{{CHECKOUT_VERSION}}", CHECKOUT_VERSION)
}

/// Returns the content of the GitHub workflow for Windows platform (x86_64).
pub fn get_build_windows() -> String {
  BUILD_WINDOWS.replace("{{CHECKOUT_VERSION}}", CHECKOUT_VERSION)
}
