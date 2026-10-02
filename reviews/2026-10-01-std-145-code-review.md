---
id: SR-880
title: "Code review — PR #174 Q9 ceremony deletion, STD-145 content-only ModelOwner, digest-chain deletion"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@1f0e91b; PR #174 diff against origin/main: Cargo.toml, Cargo.lock, src/lib.rs, tests/checked_package_model.rs, tests/checked_package_v2.rs, tests/model_member_types.rs, tests/shared_reference.rs; cross-checked quire-verification-contracts be73336 src/lib.rs (parse_bounded_json)"
review_set: subset
---
# Code review — PR #174 Q9 ceremony deletion and STD-145

## Summary

Ticket: STD-145 (pointer on QSL-395). This is a code review with the
rust-review lane folded in. It covers the Rust and Cargo part of the PR #174
diff at head 1f0e91b.

Gates were run on 1f0e91b with `CARGO_TARGET_DIR=<worktree>/target`:

- `cargo test --locked` exits 0: lib 0, checked_package_model 4,
  checked_package_v2 55, model_member_types 10, shared_reference 5,
  doctests 0.
- `cargo clippy --locked --all-targets -- -D warnings` exits 0.

The `rev` to `branch = "main"` move for the three git dev-dependencies
resolves and builds under `--locked`.

The STD-145 golden vectors were recomputed independently with JCS SHA-256:

- `model-enum-order-status` gives 251d2fdf…, and `model-dimension-order-mass`
  gives 640e314f….
- All nine `model_declaration_nodes` M1..M9 match.
- Every 64-hex reference in `model-member-type-vectors.json` is one of the
  new M-digests.
- No old ModelOwner digest survives anywhere in the repository.

The removed tests checked stored digests. They are tc_195 (rule manifest
byte for byte), tc_233 (value rule manifest), tc_281 (the catalog SHA
constant) and `definition_lock_entry_matches_bytes`. The kept tests still
check behaviour: selection slots, selection-vector codes, effective-declaration
digests, node-key recovery and the mutation refusals.

## Verdict

**Approve with changes.** There is one medium finding: the depth limit was not
removed. It moved into the parser dependency as a fixed value, and an
over-deep input is reported as `Invalid`. There are also two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `intake_with` still has a fixed, hidden depth cap. `parse_bounded_json` (quire-verification-contracts be73336, reached through quire-verification) refuses past `MAX_JSON_DEPTH = 128` and `MAX_ARRAY_ITEMS = 100_000`. `map_err(\|_\| IntakeFailure::Invalid)` turns both into `Invalid`, so the report has no limit name, no value and no way to raise it. This contradicts IT-003 Inputs ("input over a limit yields incomplete naming the limit and its value"). The test only nests 8 deep, so it never reaches this. | tests/shared_reference.rs:59 |
| FND-002 | low | `IntakeFailure::Incomplete{limit, value}` names the limit and its value. "Raise that `IntakeLimits` field" appears only in a doc comment, so the failure itself never says how to raise it. The `bytes()` helper panics with "fixture exceeds qualification budget" on the fixed `MAX_BYTES` and gives no value or field. | tests/shared_reference.rs:38-46,85-94 |
| FND-003 | low | Cargo.lock now resolves two copies of quire-verification-contracts. One is `branch=main` ead78f3 (the direct dependency, which supplies the operation catalog). The other is `rev=be73336`, pulled by quire-verification main for JCS and intake. If the two copies disagree, the tests read the catalog from one and parse with the other, and nothing would show it. | Cargo.lock; Cargo.toml:17-18 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The new iterative parser admits nesting up to the node limit, but `serde_json::Value` drops recursively. Measured on 87d32f4 by running the branch's own intake code on a 2 MiB thread (the default test-thread stack): a 40,000-deep `[[…0…]]` input is 80 KB and 40,001 nodes, inside the defaults, and it aborts the process with `stack overflow`. On its own, dropping a 32,767-deep `Value` also overflows. The 1,000-deep test pops one level at a time, so it never drops a deep value. The fix is to tear the result down iteratively, or bound the work some other way that names its limit, without bringing back a fixed depth cap. | tests/shared_reference.rs:85-221,520-535 |

## Dispositions

Round 1, reviewed at 87d32f40fd7ef0b86990e749067247cc821331d9 (confirmed with `git ls-remote`). Gates on that head: `cargo test --locked` exits 0 (lib 0, model 4, v2 56, member 10, shared 6, doctests 0), and `cargo clippy --locked --all-targets -- -D warnings` exits 0. Cargo.lock has one copy of quire-verification-contracts and no quire-verification.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 87d32f4 |
| FND-002 | fixed | 87d32f4 |
| FND-003 | fixed | 87d32f4 |
