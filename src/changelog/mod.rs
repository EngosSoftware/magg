//! # Changelog generator
//!

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

*/

pub fn a() {
  // inputs:
  // Git start revision (tag, HEAD)
  // Git end revision (tag, HEAD)
  // GH milestone name
  // GH repository name
  let a = 0;
}
