/// GitHub project.
#[derive(Default, Clone)]
pub struct GHProject {
  /// Project number.
  pub number: usize,
  /// Project title.
  pub title: String,
  /// Closed flag.
  pub closed: bool,
}
