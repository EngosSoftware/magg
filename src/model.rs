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

/// GitHub project item.
#[derive(Default, Clone)]
pub struct GHProjectItem {
  /// Item number.
  pub number: usize,
  /// Item title.
  pub title: String,
  /// Item URL.
  pub url: String,
  /// Repository.
  pub repository: String,
  /// Status.
  pub status: String,
  /// Labels.
  pub labels: Vec<String>,
}
