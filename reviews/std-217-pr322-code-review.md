---
id: SR-13690
title: "Code review of QSpec PR #322 (STD-217) refinement step occurrence vectors"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@dcc70c07d1a0905407a5242fbbbb5620727aae33 vs main 6ab82c5; tests/checked_package_v2.rs, proposals/checked-package-v2/refinement-step-occurrence-vectors.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---

## Summary

Ticket: STD-217 (PR #322, head dcc70c07d1a0905407a5242fbbbb5620727aae33; context IR-942, QSL PR #1157). Rust lane (rust-review) folded in. The fixture parses (`serde_json`), its eight cases RS-01 to RS-08 match TC-233's list, and `cargo test --test checked_package_v2 --test backend_provider_v1` passes (115 and 14). Mutations run against `tc_233_a_refinement_owner_carries_one_generated_occurrence_per_proof_obligation`: a wrong refusal pointer in RS-03 or RS-08, reversed `rows` in RS-02, the declaration moved to ordinal 1, the duplicate-key check removed and the region check removed each fail the test. Two mutations survive: deleting the `generated` ordinal-0 entry from RS-01, and deleting the `declaration` entry from RS-01.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| refinement_step_refusal | tests/checked_package_v2.rs | examined |
| tc_233_a_refinement_owner_carries_one_generated_occurrence_per_proof_obligation | tests/checked_package_v2.rs | examined |
| RS-01..RS-08 | proposals/checked-package-v2/refinement-step-occurrence-vectors.json | examined |
| SourceMapEntry | proposals/checked-package-v2/schema.json | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The completeness check in `refinement_step_refusal` iterates `(1..=rows.len())`, so it never requires the `generated` occurrence at ordinal 0, while FR-322 requires one `generated` occurrence for each ordinal 0 to `n`. No vector covers a missing ordinal-0 occurrence or a missing `declaration` occurrence: deleting either entry from RS-01 leaves the test green. Start the range at 0, and add vectors that omit the ordinal-0 `generated` entry and the `declaration` entry, with their expected refusals. | tests/checked_package_v2.rs:14552; proposals/checked-package-v2/refinement-step-occurrence-vectors.json |
| FND-002 | low | The vector `source_map` entries have no `node_id`, and their regions have no `source`, although both are required members of the V2 `SourceMapEntry`/`SourceRegion` schema. The `/source_map/{i}` pointers also count only the owner's own entries, so they are not the pointers a reader reports in a real package whose `source_map` holds other nodes' entries. State in the file (for example in its `note_regions`) that it is an owner-local projection and how its indices map to package pointers. | proposals/checked-package-v2/refinement-step-occurrence-vectors.json |

## Verdict

Not mergeable on this lens until FND-001 is fixed. Changing the range to `0..=rows.len()` keeps every current vector passing, so it can be fixed together with the two new vectors.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fe8930c: the required set is the declaration plus generated 0..=n for an eligible owner; RS-09 (no declaration), RS-10 (no generated 0), RS-11 (no generated 1) added. Deleting the generated-0 or declaration entry from RS-01 now fails the test. |
| FND-002 | fixed | fe8930c: every vector entry carries `node_id` and every region a `source`; RS-01 holds another node's entry, which the oracle skips, so pointers index the whole `source_map`. |
