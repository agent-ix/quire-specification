---
id: SR-728
title: "Code review — PR #167 STD-130 definition lock accessor"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@ce802b78d2569db659890e1b8a4c587e850192ee; PR #167 diff against origin/main 4634f5f: Cargo.toml, src/lib.rs, tests/checked_package_model.rs, tests/checked_package_v2.rs, README.md, AGENTS.md; cross-checked docs/qualification-dependencies.md, reviews/data/verification-reference-dependencies.json"
review_set: subset
---
# Code review — PR #167 STD-130 definition lock accessor

## Summary

Ticket: STD-130. Code review with the rust-review lane folded in, scoped to
the PR #167 diff.

The PR adds `[lib] name = "quire_specification"` to the existing package
`quire-specification-qualification`. `src/lib.rs` holds two `include_str!`
constants, `COMPLETE_VALUE_LOCK` and `NATIVE_DIAGNOSTICS`, and nothing else.
The package has no `[dependencies]` and no `build.rs`; every other crate is a
dev-dependency. Both `include_str!` paths resolve to files inside the package
root (`proposals/quire-v1/definitions/`), and the package has no
`include`/`exclude`, so a git checkout carries them.

Measured: a scratch consumer with
`quire-specification-qualification = { git = "file://<worktree>", rev = "ce802b7" }`
locked one package, compiled only the lib in 0.15 s, and printed 9179 and
32753 bytes, equal to the two files on disk. `cargo fmt --check`,
`cargo test --locked` (lib 0, checked_package_model 6, checked_package_v2 55,
model_member_types 11, shared_reference 5, doctests 0; all pass) and
`cargo clippy --locked --all-targets --all-features -- -D warnings` all exit 0
on ce802b7 with Rust 1.98.1.

The lib exposes the authored bytes only. It carries no digest constant. The
lock's own per-artifact digests are FR-322 raw-byte content identities of the
definitions, which is the canonical identity exception, not a recorded pin.

## Verdict

**Approve with changes.** One medium finding and two low findings. None
blocks the accessor itself; all three are small text or layout fixes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The README paragraph this PR edits links `docs/qualification-dependencies.md`, whose "Exact selection" column and the `reviews/data/verification-reference-dependencies.json` inventory it cites restate versions and git revs that live in Cargo.toml and Cargo.lock. That is version tracking, which AGENTS.md says to remove when touched. Drop the column, the inventory file and the README's "exact selected sources" clause; keep the licence-grant rows. | README.md:92-93; docs/qualification-dependencies.md:8-26 |
| FND-002 | low | Two `use quire_specification::...` declarations sit in the middle of the file among `const` items (one at line 30 between `include_str!` consts, one at line 5316 after a test fn) instead of with the other imports at the top. | tests/checked_package_v2.rs:30,5316 |
| FND-003 | low | "record its grants and exact selected sources" now follows a sentence about the dependency-free `quire_specification` lib, so "its" reads as the lib. The grants belong to the dev-only qualification tests. | README.md:90-93 |

## Dispositions

Round 1, reviewed at 21e5a7bda4b2acfffe077af1b2649f24f1d9caf9. Gates re-run
on that head with Rust 1.98.1: `cargo fmt --check`, `cargo test --locked`
(78 tests, all pass) and clippy `-D warnings` all exit 0. The new
`COMPLETE_VALUE_SELECTION_VECTORS` constant includes a file inside the package
and no test still reads that file directly. No new code-review findings.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 21e5a7b |
| FND-002 | fixed | 21e5a7b |
| FND-003 | fixed | 21e5a7b |
