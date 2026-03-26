const CHECKOUT_VERSION: &str = "v6";

const BUILD_LINUX: &str = include_str!("BUILD-LINUX.txt");

pub fn get_build_linux() -> String {
  BUILD_LINUX.replace("{{CHECKOUT_VERSION}}", CHECKOUT_VERSION)
}
