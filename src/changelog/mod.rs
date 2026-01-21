//! # Changelog generator

use crate::errors::{MaggError, Result, error_execute_command, error_obtain_output, error_spawn_command};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write;
use std::path::Path;

struct Commit {
  hash: String,
  subject: String,
}

struct PullRequest {
  number: String,
  title: String,
  url: String,
  commits: Vec<Commit>,
}

pub fn get_changelog() -> Result<String> {
  let repository = "CosmWasm/wasmvm";
  let mut commits = get_commits("/Users/ddepta/Work/CosmWasm/wasmvm", "v2.2.3", "v2.2.4")?;
  let mut pull_requests = get_pull_requests("2.2.4", repository)?;

  println!("COMMITS:");
  for commit in &commits {
    println!("{} | {}", commit.hash, commit.subject);
  }
  println!("PULL REQUESTS:");
  for pull_request in &pull_requests {
    println!("{} | {} | {}", pull_request.number, pull_request.title, pull_request.url);
    for commit in &pull_request.commits {
      println!("  {} | {}", commit.hash, commit.subject);
    }
  }

  // Move all commits to the map.
  let mut commit_map: HashMap<String, Commit> = HashMap::new();
  for commit in commits.drain(..) {
    commit_map.insert(commit.hash.clone(), commit);
  }

  // Move all pull requests to sorted map.
  let mut pull_request_tree: BTreeMap<String, PullRequest> = BTreeMap::new();
  for pull_request in pull_requests.drain(..) {
    // From commit map remove commits that are included in pull request.
    for commit in &pull_request.commits {
      commit_map.remove(&commit.hash);
    }
    pull_request_tree.insert(pull_request.number.clone(), pull_request);
  }

  // Prepare the string buffer for the changelog content.
  let mut changelog = String::new();
  // Write a list of pull requests.
  for pull_request in pull_request_tree.values().rev() {
    let _ = writeln!(&mut changelog, "- {} ([#{}])", pull_request.title, pull_request.number);
  }
  // Write a list of commits.
  for commit in commit_map.values() {
    let _ = writeln!(&mut changelog, "- {} ([0x{}])", commit.subject, &commit.hash[..7]);
  }
  // Separate the list of changes from links.
  let _ = writeln!(&mut changelog);
  // Write links to pull requests.
  for pull_request in pull_request_tree.values().rev() {
    let _ = writeln!(&mut changelog, "[#{}]: {}", pull_request.number, pull_request.url);
  }
  // Write links to commits.
  for commit in commit_map.values() {
    let _ = writeln!(&mut changelog, "[0x{}]: https://github.com/{repository}/commit/{}", &commit.hash[..7], commit.hash);
  }
  Ok(changelog)
}

fn get_pull_request_commits(number: &str, repository: &str) -> Result<Vec<Commit>> {
  // gh pr view 661 --repo=cosmwasm/wasmvm --json=commits --jq='.commits[] | "\(.oid) ||| \(.messageHeadline)"'
  let repo = format!("--repo={}", repository);
  let args = &[
    "pr",
    "view",
    number,
    repo.as_str(),
    "--json=commits",
    r#"--jq=.commits[]|"\(.oid) ||| \(.messageHeadline)""#,
  ];
  let stdout = execute_command("gh", args, ".")?;
  parse_commits(stdout)
}

fn parse_pull_requests(input: String, repository: &str) -> Result<Vec<PullRequest>> {
  let mut pull_requests = vec![];
  let rows = parse_columns(input, 3)?;
  for columns in rows {
    let number = columns[0].to_string();
    let commits = get_pull_request_commits(&number, repository)?;
    pull_requests.push(PullRequest {
      number,
      title: columns[1].to_string(),
      url: columns[2].to_string(),
      commits,
    });
  }
  Ok(pull_requests)
}

fn get_pull_requests(milestone: &str, repository: &str) -> Result<Vec<PullRequest>> {
  let search = format!(r#"--search=milestone:{}"#, milestone);
  let repo = format!("--repo={}", repository);
  let args = &[
    "pr",
    "list",
    search.as_str(),
    "--state=all",
    repo.as_str(),
    "--json=number,title,url",
    r#"--template='{{range .}}{{printf "%v ||| %s ||| %s\n" .number .title .url}}{{end}}'"#,
  ];
  let stdout = execute_command("gh", args, ".")?;
  parse_pull_requests(stdout, repository)
}

fn parse_commits(input: String) -> Result<Vec<Commit>> {
  let mut commits = vec![];
  let rows = parse_columns(input, 2)?;
  for columns in rows {
    commits.push(Commit {
      hash: columns[0].to_string(),
      subject: columns[1].to_string(),
    });
  }
  Ok(commits)
}

fn get_commits(dir: impl AsRef<Path>, start_revision: impl AsRef<str>, end_revision: impl AsRef<str>) -> Result<Vec<Commit>> {
  let program = "git";
  let revisions = format!("{}...{}", start_revision.as_ref(), end_revision.as_ref());
  let args = &["log", r#"--format="%H ||| %s""#, revisions.as_str(), "--"];
  let stdout = execute_command(program, args, dir)?;
  parse_commits(stdout)
}

fn execute_command(program: &str, args: &[&str], dir: impl AsRef<Path>) -> Result<String> {
  println!("COMMAND: {} {}", program, args.join(" "));
  let mut command = std::process::Command::new(program);
  let child = command
    .args(args)
    .current_dir(dir)
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::piped())
    .spawn()
    .map_err(|e| error_spawn_command(program, e.to_string()))?;

  let output = child.wait_with_output().map_err(|e| error_obtain_output(e.to_string()))?;
  let stdout = String::from_utf8_lossy(&output.stdout).to_string();
  let stderr = String::from_utf8_lossy(&output.stderr).to_string();
  let status = output.status;
  if status.success() {
    Ok(stdout)
  } else {
    Err(error_execute_command(status, stdout, stderr))
  }
}

fn parse_columns(input: String, col_count: usize) -> Result<Vec<Vec<String>>> {
  let mut rows = vec![];
  for mut line in input.lines().map(|line| line.trim()) {
    if line.starts_with("\"") {
      line = line.strip_prefix("\"").unwrap();
    }
    if line.starts_with("'") {
      line = line.strip_prefix("'").unwrap();
    }
    if line.ends_with("\"") {
      line = line.strip_suffix("\"").unwrap();
    }
    if line.ends_with("'") {
      line = line.strip_suffix("'").unwrap();
    }
    line = line.trim();
    if !line.is_empty() {
      let columns = line.split(" ||| ").map(|s| s.to_string()).collect::<Vec<String>>();
      if columns.len() != col_count {
        return Err(MaggError::new(format!("invalid number of columns, expected: {col_count}, actual: {}", columns.len())));
      }
      rows.push(columns);
    }
  }
  Ok(rows)
}
