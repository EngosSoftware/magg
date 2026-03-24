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
  #[allow(unused)]
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
  /// Improvements and refactoring.
  Imp,
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
      "g:imp" => GHLabel::Imp,
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
        GHLabel::Rel => "g:rel - Releases",
        GHLabel::Fea => "g:fea - New features",
        GHLabel::Imp => "g:imp - Improvements and refactoring",
        GHLabel::Fix => "g:fix - Bug fixes",
        GHLabel::Dep => "g:dep - Dependency upgrades",
        GHLabel::Doc => "g:doc - Documentation and website updates",
        GHLabel::Res => "g:res - Research",
        GHLabel::Sec => "g:sec - Security updates",
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
    matches!(
      self,
      GHLabel::Rel | GHLabel::Fea | GHLabel::Imp | GHLabel::Fix | GHLabel::Dep | GHLabel::Doc | GHLabel::Res | GHLabel::Sec
    )
  }

  pub fn groups() -> Vec<GHLabel> {
    vec![
      GHLabel::Rel,
      GHLabel::Fea,
      GHLabel::Imp,
      GHLabel::Fix,
      GHLabel::Dep,
      GHLabel::Doc,
      GHLabel::Res,
      GHLabel::Sec,
    ]
  }
}
