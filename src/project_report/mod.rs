use crate::errors::*;
use crate::model::*;
use crate::utils;
use std::fmt::Write;

pub fn get_project_report(project_owner: String, project_name: String) -> Result<String> {
  let mut output = String::new();
  let projects = get_projects(false, &project_owner)?;
  let Some(project) = projects.iter().find(|project| project.title == project_name) else {
    return Err(MaggError::new(format!("Project '{}' not found for owner '{}'", project_name, project_owner)));
  };
  _ = writeln!(&mut output, "number={} title={} closed={}", project.number, project.title, project.closed);
  Ok(output)
}

/// Retrieves projects for specified owner.
fn get_projects(verbose: bool, project_owner: &str) -> Result<Vec<GHProject>> {
  let owner = format!("--owner={}", project_owner);
  let args = &[
    "project",
    "list",
    owner.as_str(),
    "--limit=9999",
    "--format=json",
    r#"--template='{{range .projects}}{{printf "%v ||| %s ||| %v\n" .number .title .closed}}{{end}}'"#,
  ];
  let stdout = utils::execute_command(verbose, "gh", args, ".")?;
  parse_projects(stdout)
}

/// Parses project details.
fn parse_projects(input: String) -> Result<Vec<GHProject>> {
  let mut projects = vec![];
  let rows = utils::parse_columns(input, 3)?;
  for columns in rows {
    projects.push(GHProject {
      number: columns[0].to_string().parse().unwrap(),
      title: columns[1].to_string(),
      closed: columns[2].to_string().parse().unwrap(),
    });
  }
  Ok(projects)
}
