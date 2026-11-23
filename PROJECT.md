# frencli

[All projects](../../../WORKSPACE.md)

Project record for this tree, reviewed 19 November 2026. Git facts are from this machine at review time, taken before this file was added.

| | |
|---|---|
| Path | `~/development/tools/fren/frencli` |
| Kind | Rename toolkit |
| Stack | Rust |
| Repo | byezy/frencli |
| State | clean, in sync, 2026-04-16 |

## What it is

A fast, powerful command-line batch file renaming tool inspired by the "Multi-Rename Tool" in Total Commander.

Source of the description: `README.md`.

## Repository and code state

This folder is its own git repository.

- Branch: `main`
- Upstream: `origin/main`
- Remote: https://github.com/byezy/frencli.git
- Ahead / behind: 0 ahead, 0 behind
- Commits: 12
- Latest commit: 2026-04-16 `e1f853e` chore(release): prepare frencli v0.1.5 for crates.io
- Working tree: clean

## Review

Second pass on 16 November 2026. How this was judged, and how to refresh it, is in [_workspace/review/README.md](../../../_workspace/review/README.md).

Depth for this pass: a file census, the test filenames, the README, and the git snapshot. Tests were not executed. There is no coverage percentage.

**Tests.** First-party source census: 21 source files, 19 test-like files. The count skips `node_modules`, build output, `.svelte-kit`, `third_party`, and rustdoc dumps. Shell, Markdown, and YAML are not in the source count. Runners seen: Cargo.toml. Sample test files: `test_helper.py`, `test_main.rs`, `test_utils.rs`, `apply_tests.rs`.

**Code quality.** Test files are present (19 beside 21 source files). That ratio is not a coverage figure: UI and platform files inflate the source count, and a few focused tests can still cover the domain. Test files are about as numerous as source files (19 and 21). This is the CLI front-end over freneng.

**Risks.** Git snapshot shows nothing more urgent than the test notes above.
