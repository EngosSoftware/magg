//! # Changelog generator
//!

use crate::errors::{MaggError, Result, error_execute_command, error_obtain_output, error_spawn_command};
use std::path::Path;

/*

This command displays a list of commits between two tags:

git log --format="%H ||| %s" v3.0.1...v3.0.2

Output:

96dbd737cb9ac2bf9d617ce9b8f2a05b2db90bbc ||| trigger gh actions on tag
0b65977879d2ea71fd382ce65a33cb76cd430460 ||| [skip ci] Built release libraries
4da6e4d9ea35fc5d6725dd2c076c6a87fb32ee0b ||| Prepare for release v3.0.2
f6bb401e079c88f4b5ddf690b14bbd2f9c0c2236 ||| [skip ci] Built release libraries

Column separator is " ||| ".

--------


git log --format="%H ||| %s" v2.2.3...v2.2.4 --

git log --format="%H ||| %s" v3.0.1...HEAD --

Output:

45efbe55f3874020a1ba16effadf2f6737a69da1 ||| [skip ci] Built release libraries
67b6dc70f9d7364be3bd27de1bc0ece05263ef24 ||| Set libwasmvm version to 2.2.4
66234c119c730a9f82117606ab033ab3a2c3b465 ||| [skip ci] Built release libraries
3bfac0dcaaad612b81564191783456389d349ad4 ||| Merge pull request #660 from CosmWasm/mergify/bp/release/2.2/pr-635
683b5709a64042440c676bed3dd05e310281e247 ||| [skip ci] Built release libraries
411ce3d48f8461220f8798c4fddd129a3d1cf5cc ||| Adapt code to linter rule
c353f0794144d93f060324512da5c43c6e4ca1e8 ||| Add doc comments to ExpectedJSONSize
dd27921e581a74648fffc4244d3753a780d8d7bd ||| Fix handling of \b, \f, \n, \r, \t
bd90e0f7b5c2c3f043ed9fdaf1ff77fb829bbfdb ||| Add ExpectedJSONSize
17bc1a65680ee5278deb7f73ec95589181cb7410 ||| Merge pull request #661 from CosmWasm/mergify/bp/release/2.2/pr-637
cec743ff73aa709eaaee53e1d14ef9237d10d854 ||| Bump github actions linting job
af21fc7aeb89a4523bd1b42c8d155521e6ea4d77 ||| Bump min Go version to 1.22
d5920b179028b0743880ab1858620bd81bc20f99 ||| [skip ci] Built release libraries
d1cb93bf0cac5133cbe325aee262e919a4409fb9 ||| Merge pull request #650 from CosmWasm/backport-2.2-Improve-panic-messages-when-VM-panicks
e697442ad6319b12373a9398628b21a6b3d404ff ||| Auto-deref, ... okay clippy
a92f7589c9cf9326bf7b6ee895d5aa66412e833d ||| Use Display representation of err: &str
b49cb90307a73146865098014e5a9557b7d8c1d6 ||| Improve panic messages

gh pr list --search "milestone:2.2.4" --state all --repo cosmwasm/wasmvm --json number,title,url

Output:

[
  {
    "number": 661,
    "title": "Bump min Go version to 1.22 (backport #637)",
    "url": "https://github.com/CosmWasm/wasmvm/pull/661"
  },
  {
    "number": 660,
    "title": "Add ExpectedJSONSize (backport #635)",
    "url": "https://github.com/CosmWasm/wasmvm/pull/660"
  },
  {
    "number": 650,
    "title": "Backport 2.2: improve panic messages when vm panicks",
    "url": "https://github.com/CosmWasm/wasmvm/pull/650"
  }
]

gh pr view 661 --repo cosmwasm/wasmvm --json commits

{
  "commits": [
    {
      "authoredDate": "2025-04-10T15:59:37Z",
      "authors": [
        {
          "email": "simon@warta.it",
          "id": "MDQ6VXNlcjI2MDMwMTE=",
          "login": "webmaster128",
          "name": "Simon Warta"
        }
      ],
      "committedDate": "2025-04-25T14:39:38Z",
      "messageBody": "for https://github.com/CosmWasm/wasmvm/pull/635#issuecomment-2794217549\n\n(cherry picked from commit 12962dfadc2ee54739fd6b154fec5e7c264579b1)",
      "messageHeadline": "Bump min Go version to 1.22",
      "oid": "af21fc7aeb89a4523bd1b42c8d155521e6ea4d77"
    },
    {
      "authoredDate": "2025-04-25T14:43:06Z",
      "authors": [
        {
          "email": "chris@confio.gmbh",
          "id": "MDQ6VXNlcjQ0NjY5Mzc=",
          "login": "chipshort",
          "name": "Christoph Otter"
        }
      ],
      "committedDate": "2025-04-25T14:43:06Z",
      "messageBody": "",
      "messageHeadline": "Bump github actions linting job",
      "oid": "cec743ff73aa709eaaee53e1d14ef9237d10d854"
    }
  ]
}

gh pr list --search "milestone:2.2.4" --state all --repo cosmwasm/wasmvm --json number,title,url --template '{{range .}}{{printf "%v ||| %s ||| %s\n" .number .title .url}}{{end}}'

*/

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
  // inputs:
  // Git start revision (tag, HEAD)
  // Git end revision (tag, HEAD)
  // GH milestone name
  // GH repository name

  let commits = get_commits("/Users/ddepta/Work/CosmWasm/wasmvm", "v2.2.3", "v2.2.4")?;

  for commit in commits {
    println!("{} | {}", commit.hash, commit.subject);
  }

  let pull_requests = get_pull_requests("2.2.4", "cosmwasm/wasmvm")?;

  for pull_request in pull_requests {
    println!("{} | {} | {}", pull_request.number, pull_request.title, pull_request.url);
  }

  Ok("CHANGELOG".to_string())
}

fn parse_pull_requests(input: String) -> Result<Vec<PullRequest>> {
  let mut pull_requests = vec![];
  let rows = parse_columns(input, 3)?;
  for columns in rows {
    pull_requests.push(PullRequest {
      number: columns[0].to_string(),
      title: columns[1].to_string(),
      url: columns[2].to_string(),
      commits: vec![],
    });
  }
  Ok(pull_requests)
}

fn get_pull_requests(milestone: impl AsRef<str>, repository: impl AsRef<str>) -> Result<Vec<PullRequest>> {
  let search = format!(r#"--search=milestone:{}"#, milestone.as_ref());
  let repo = format!("--repo={}", repository.as_ref());
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
  parse_pull_requests(stdout)
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
  println!("Executing command:\n{} {}", program, args.join(" "));
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
