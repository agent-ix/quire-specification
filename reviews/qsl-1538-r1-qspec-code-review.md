---
id: SR-20261103145409
title: "QSL-1538 r1 code review (Rust lane) of quire-specification PR #610"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification; PR #610 head f7934212; tests/protocol_node_mutation_manifest.rs; proposals/checked-package-v2/fixtures/protocol-node-mutation-manifest.json; proposals/checked-package-v2/fixtures/protocol-node-vectors.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-465
    type: references
---

# QSL-1538 r1 code review (Rust lane) of PR #610

## Summary

Ticket: QSL-1538. The only Rust change deletes
`assert_eq!(expected.len(), 406, "the prose corpus is 406 vectors")` from
`every_vector_is_a_row_a_later_row_or_a_pending_question_exactly_once`. The
manifest note also drops the "26" from "The 26 Q-LAYOUT vectors".

**Value test.** The count was a pin: adding any vector broke it, and it
guarded nothing the partition check does not already guard. The partition
check reads the vector ids from the vectors fixture and asserts that every id
appears exactly once across rows, later and pending. A dropped or duplicated
entry still fails. The pin's removal is justified. Nothing else in the repo
depends on 406 or 26: a `git grep` excluding SVGs and unrelated FR-406 ids
found no other use.

**Evidence.** No cargo build was run. `/` had 4.8G free, below the 15G
threshold set for this review. Instead:

- The partition was recomputed in Python with the test's `ids_of` rule:
  409 ids, unique, and equal to the vector set.
- The manifest validates against its schema (jsonschema, 0 errors).
- Both JSON fixtures parse.

## Verdict

**PASS.** The deletion is correct. The fixture entries follow the existing
shapes, and the remaining test still enforces the partition.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
