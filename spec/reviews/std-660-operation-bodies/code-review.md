---
id: SR-202611101500007
title: "Code review of the STD-660 operation_bodies reference reader"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification; PR #651 at 6ff1281f: tests/checked_package_v2.rs (read_operation_bodies, operation_body_carrier_vectors_match_the_closed_member_schema_and_reader), proposals/checked-package-v2/schema.json OperationBodyRow/OperationBody"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
---

## Summary

Ticket: STD-660. A code review with its Rust lane (rust-review) over the reference reader and the
test. The reader follows the section's check order exactly. It reuses the `operation_effects`
helpers (`effect_node`, `effect_node_order`, `refuse`, `apply_json_patch`). The test asserts the
exact code, cause and locus of every refusal.

## Verdict

CONDITIONAL with one low finding. Ran `cargo test --locked --test checked_package_v2`: 168 passed.
Assertions can fail: each removed check and each schema change was caught by a mutation run, which
the gap-analysis SpecReview reports. Panics come from `unwrap` on fixture data only, inside a test,
so there is no untrusted-input path. The test cannot detect a body that belongs to another
operation, because the section it implements does not require that join. That gap is the
failure-domain SpecReview's FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new code fails both of the README's gates. `cargo clippy --locked --test checked_package_v2` reports two new `clippy::type_complexity` errors, at 22533 (`declared` map) and 22558 (`keys`): 20 errors on the head against 18 on main. `rustfmt --edition 2024 --check` reports 21 diff hunks in lines 22543-22750. Both gates were already red on main for this file, which is why the severity is low. Fix: add type aliases (for example `type NodeKey = (String, Vec<String>, u64)`) and run rustfmt over the new lines. | tests/checked_package_v2.rs:22533; tests/checked_package_v2.rs:22558 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 498a3593 adds the `BodyNodeKey`, `BodyRowKey` and `BodyDeclared` aliases and runs rustfmt over the new code. `cargo clippy --locked --test checked_package_v2` is back to the 18 errors on main, all before line 22252. rustfmt reports no hunk between 22524 and 22849. Every hunk of the fix commit lies inside the author's own functions (22524-22849). |
