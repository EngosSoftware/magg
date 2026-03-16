use std::fmt::{Debug, Display, Formatter};

/// GitHub project.
#[derive(Debug, Default, Clone)]
pub struct GHProject {
  /// Project number.
  pub number: usize,
  /// Project title.
  pub title: String,
  /// Closed flag.
  pub closed: bool,
}

/// GitHub project item.
#[derive(Debug, Default, Clone)]
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
  pub labels: Vec<GHLabel>,
  /// Group label.
  pub group: Option<GHLabel>,
}

/// GitHub label.
#[derive(Clone, PartialEq)]
pub enum GHLabel {
  /// Releases.
  Rel,
  /// New features.
  Fea,
  /// Bug fixes.
  Fix,
  /// Dependency upgrades.
  Dep,
  /// Documentation and website updates.
  Doc,
  /// Research.
  Res,
  /// Security updates.
  Sec,
  /// Custom label.
  Custom(String),
}

impl From<&str> for GHLabel {
  fn from(value: &str) -> Self {
    match value {
      "g:rel" => GHLabel::Rel,
      "g:fea" => GHLabel::Fea,
      "g:fix" => GHLabel::Fix,
      "g:dep" => GHLabel::Dep,
      "g:doc" => GHLabel::Doc,
      "g:res" => GHLabel::Res,
      "g:sec" => GHLabel::Sec,
      other => GHLabel::Custom(other.to_string()),
    }
  }
}

impl Display for GHLabel {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        GHLabel::Rel => "g:rel",
        GHLabel::Fea => "g:fea",
        GHLabel::Fix => "g:fix",
        GHLabel::Dep => "g:dep",
        GHLabel::Doc => "g:doc",
        GHLabel::Res => "g:res",
        GHLabel::Sec => "g:sec",
        GHLabel::Custom(other) => other,
      }
    )
  }
}

impl Debug for GHLabel {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}

impl GHLabel {
  pub fn is_group(&self) -> bool {
    matches!(self, GHLabel::Rel | GHLabel::Fea | GHLabel::Fix | GHLabel::Dep | GHLabel::Doc | GHLabel::Res | GHLabel::Sec)
  }
}
