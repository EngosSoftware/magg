use crate::errors::*;
use crate::model::*;
use crate::utils;
use std::fmt::Write;

pub fn get_project_report(project_owner: &str, project_name: &str) -> Result<String> {
  let mut output = String::new();
  let projects = get_projects(false, &project_owner)?;
  let Some(project) = projects.iter().find(|project| project.title == project_name) else {
    return Err(MaggError::new(format!("Project '{}' not found for owner '{}'", project_name, project_owner)));
  };

  let project_items = get_project_items(false, project_owner, project.number)?;

  let mut count = 0_usize;
  for project_item in project_items {
    if project_item.status == "done" {
      _ = writeln!(
        &mut output,
        "{:>6} | {} | {} | {:?}",
        project_item.number, project_item.title, project_item.url, project_item.labels
      );
      count += 1;
    }
  }
  _ = writeln!(&mut output, "Total count = {}", count);

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

/// Retrieves project items for specified owner.
fn get_project_items(verbose: bool, project_owner: &str, project_number: usize) -> Result<Vec<GHProjectItem>> {
  let project_number = format!("{}", project_number);
  let owner = format!("--owner={}", project_owner);
  let args = &[
    "project",
    "item-list",
    project_number.as_str(),
    owner.as_str(),
    "--limit=9999",
    "--format=json",
    r#"--template='{{range .items}}{{printf "%v ||| %s ||| %s ||| %s ||| %s ||| %v\n" .content.number .content.title .content.url .repository .status .labels}}{{end}}'"#,
  ];
  let stdout = utils::execute_command(verbose, "gh", args, ".")?;
  parse_project_items(stdout)
}

/// Parses project item details.
fn parse_project_items(input: String) -> Result<Vec<GHProjectItem>> {
  let mut project_items = vec![];
  let rows = utils::parse_columns(input, 6)?;
  for columns in rows {
    project_items.push(GHProjectItem {
      number: columns[0].to_string().parse().unwrap(),
      title: columns[1].to_string(),
      url: columns[2].to_string(),
      repository: columns[3].to_string(),
      status: columns[4].to_lowercase(),
      labels: parse_labels(&columns[5]),
    });
  }
  Ok(project_items)
}

fn parse_labels(mut input: &str) -> Vec<String> {
  input = input.trim();
  if input.is_empty() || input == "<nil>" {
    vec![]
  } else {
    if input.starts_with("[") {
      input = input.strip_prefix("[").unwrap();
    }
    if input.ends_with("]") {
      input = input.strip_suffix("]").unwrap();
    }
    input.split(" ").map(|s| s.to_string()).collect::<Vec<String>>()
  }
}

/*

gh project item-list 15 --owner=CosmWasm --limit=2 --format=json --template='{{range .items}}{{printf "%v ||| %s ||| %s ||| %s ||| %s ||| %v\n" .content.number .content.title .content.url .repository .status .labels}}{{end}}'

 */
