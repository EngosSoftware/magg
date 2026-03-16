use crate::errors::*;
use crate::model::*;
use crate::utils;
use std::fmt::Write;

pub fn get_project_report(project_owner: &str, project_name: &str) -> Result<String> {
  let projects = get_projects(false, project_owner)?;
  let Some(project) = projects.iter().find(|project| project.title == project_name) else {
    return Err(MaggError::new(format!("Project '{}' not found for owner '{}'", project_name, project_owner)));
  };
  if project.closed {
    return Err(MaggError::new(format!("Project '{}' is closed", project_name)));
  }
  let project_items = get_project_items(false, project_owner, project.number)?;
  let mut report_items = vec![];
  for project_item in project_items.iter().filter(|i| i.status == "done") {
    let groups = project_item.labels.iter().filter(|l| l.is_group()).collect::<Vec<&GHLabel>>();
    if groups.is_empty() {
      return Err(MaggError::new(format!("Item is not assigned to any group: {}", project_item.url)));
    }
    if groups.len() > 1 {
      return Err(MaggError::new(format!("Item is assigned to multiple groups: {}", project_item.url)));
    }
    let mut report_item = project_item.clone();
    report_item.group = Some(groups[0].clone());
    report_items.push(report_item);
  }
  Ok(get_report(report_items))
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
      group: None,
    });
  }
  Ok(project_items)
}

fn parse_labels(mut input: &str) -> Vec<GHLabel> {
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
    input.split(" ").map(GHLabel::from).collect::<Vec<GHLabel>>()
  }
}

fn get_report(mut report_items: Vec<GHProjectItem>) -> String {
  report_items.sort_by_key(|i| i.repository.clone());
  println!("\nTotal count = {}", report_items.len());
  let mut output = String::new();
  let groups: [GHLabel; 7] = [GHLabel::Rel, GHLabel::Fea, GHLabel::Fix, GHLabel::Dep, GHLabel::Doc, GHLabel::Res, GHLabel::Sec];
  for group in &groups {
    _ = writeln!(&mut output, "{}", group);
    for report_item in report_items.iter().rev() {
      if let Some(item_group) = &report_item.group
        && item_group == group
      {
        _ = writeln!(&mut output, " - {}", report_item.url)
      }
    }
  }
  output
}
