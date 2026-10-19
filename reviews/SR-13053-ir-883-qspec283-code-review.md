---
id: SR-13053
title: "IR-883 QSpec PR 283 code review: tests/checked_package_v2.rs trace removal"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@7baa1104b8a37f8606a6c05a9e0c43fba8ddada8 vs origin/main a2230e1; tests/checked_package_v2.rs"
review_set: subset
---

## Summary

Ticket: IR-883. The only code change is the deletion of `#[trace("TC-233", "FR-322-AC-35")]` from `tc_233_dependency_selections_carry_each_dependency_package_identity` (tests/checked_package_v2.rs:5329). The rust-review lane was run over that hunk: no idiom, panic or lint issues; clippy `-D warnings` and `cargo test --locked` pass. The test body is unchanged. It still asserts AC-35's unchanged clauses: schema admission of two ascending direct entries, the `package_id` recompute and change, the entry mutations, the `ModelRef` `version` mutation, and the order vectors with outcomes and loci.

## Examined criteria

- FR-322-AC-35 (examined) against `tc_233_dependency_selections_carry_each_dependency_package_identity`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Removing the trace leaves FR-322-AC-35 untagged in `quire matrix` (it was tagged at tests/checked_package_v2.rs:5331 on main). The test still verifies every clause AC-35 carried before this PR. The binding is correct for those clauses; it only became partial because two planned reader clauses were appended to the same row. Keep the tag and move the new clauses to their own AC (SR-13051 FND-002), rather than dropping verified coverage | tests/checked_package_v2.rs:5329 |

## Verdict

**FAIL as-is: medium trace regression.** The code is otherwise correct.

## Dispositions

Disposition pass 1, against head 78abc809a95ca92c2e426e0d78569566d1f81aec (fix commit 73aec7e).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `#[trace("TC-233", "FR-322-AC-35")]` is restored on `tc_233_dependency_selections_carry_each_dependency_package_identity`, and `quire matrix` shows FR-322-AC-35 tagged at tests/checked_package_v2.rs:5417 |
