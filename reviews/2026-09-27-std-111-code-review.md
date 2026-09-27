---
id: SR-724
title: "Code review (rust lane) — STD-111 state clause, operation anchor, frame and reaches_field tests"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@7160d58dcf7a7182ca64c6f61c5aa10df336869d; PR #163 diff against origin/main: tests/checked_package_v2.rs, tests/model_member_types.rs, Cargo.toml, Cargo.lock, proposals/checked-package-v2 fixtures, vectors and schemas"
review_set: subset
---
# Code review (rust lane) — STD-111 state clause, operation anchor, frame and reaches_field tests

## Summary

Ticket: STD-111. Rust lane over the two qualification test files the PR
changes. `cargo test --locked --target-dir target-codex-backends` passes at
7160d58 (verified externally by the dispatcher, exit 0). Every new
criterion (FR-322-AC-38, FR-340-AC-1..13, FR-341-AC-1..9, FR-342-AC-1..5)
carries a `#[trace]` tag on a real test. The trace mapping agrees with
`spec/complete-v1/tests.md`.

The frame oracle's switch from `FrameMeaning` to `FrameEntryKind` is
clean. Defect ordering still comes from `derive(Ord)` over
`(group, digest, name, cause)`, and adding the `SemanticType` variant
first keeps FR-340's tie-break order. `package_state_refusal` finds a
misplaced clause application by pointer identity against the node's own
`body`. That is correct because `application_terms` returns borrows into
the same tree. The `reaches_field` oracle follows FR-322's four steps in
order. The catalog bump in `Cargo.toml`/`Cargo.lock` matches the digest
constant the tests assert.

The two findings below are local to the TC-280 oracles. Gaps in what the
full reader checks are recorded in the gap-analysis artifact (SR-725).

## Verdict

**Approve with nits.** Two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The doc comment on `resolve_member` names parameters that do not exist ("one member of `kind`", "mapped as `refuse` maps a count of matches"). The function takes `list` and `none`. Reword it to describe `list` (the `fields`/`operations` member list) and `none` (the refusal for zero matches). | tests/model_member_types.rs:776-779 |
| FND-002 | low | `anchor_operation` resolves the operation first and checks the context's `semantic_form` afterwards. FR-342's reader order checks Context (step 1, `malformed-declaration`) before Operation (step 4). So a `systems_interface` context with an undeclared operation name gives this oracle `missing_declaration`/`missing-name`, but the spec requires `invalid_model_binding`/`malformed-declaration`. AN-07 only passes because `rate` resolves. Check the form before calling `resolve_member`. | tests/model_member_types.rs:947-962 |
