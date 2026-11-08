---
id: SR-9131
title: "Base review of the domain package map type key and bound"
type: SpecReview
analysis: base
scope: "quire-specification spec/package-map-key-bound diff origin/main...cd6c4059: FR-154 Map types, FR-144, FR-322, FR-415, TC-234 I35/I36, TC-367 CK-13/CK-39, map-type-vectors.json package_map_types, README, tests/package_map_types.rs"
review_set: subset
review_date: "2026-11-08"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-154, type: references }
  - { target: ix://agent-ix/quire-specification/FR-144, type: references }
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
  - { target: ix://agent-ix/quire-specification/FR-415, type: references }
---

## Summary

REVISE. The base checklist passes on IDs, the Given/When/Then-free AC table
style this repo uses, error codes and cross-references. The new FR-154 rows,
FR-154-AC-48/49, TC-234 I35/I36 and PM-01..PM-27 agree case for case. The
defects are in criterion coverage. FR-415-AC-27 is new and no test carries it.
The extended FR-144-AC-26 is untagged. FR-322-AC-396's node-level claims are
tagged only by a mutation test that never builds the nodes. The new test also
asserts fixture bookkeeping (a case count, ID sequence and criterion labels),
which the checklist forbids.

## Review set

`git diff origin/main...HEAD` (15 files). `quire matrix --scope . --format tsv`
gave the coverage status. Checked with the repo's convention that
Verification cells carry `Test (TC-NNN)`, which is not a finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-415-AC-27 is a new criterion, and `quire matrix` reports it `untagged`: no test carries it. It covers key-selected places, the place staying put when entries are inserted or removed, the `ref` refusal, the snapshot `Reference<Clock>` refusal, ineligible keys and clock dimension 2. TC-367 CK-39 states the vector, but no test runs it. FR-144-AC-26, extended in this diff with `Map<Cfg, Int>` and `Map<Option<Text>, Int>`, is also `untagged`. Fix: add a trace-tagged test for FR-415-AC-27 (at least the static parts: the key-type selector, the refusals at the key type, and dimension 2 from `bound.upper` times one clock). Tag a test that asserts the FR-144-AC-26 key refusals (`package_map_types.rs` PM-10/PM-11/PM-12 already assert them and could also carry FR-144-AC-26). | spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md:534; spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:367; spec/test-cases/TC-367-clock-declarations.md:68 |
| FND-002 | medium | FR-322-AC-396 makes node-level claims: a `collection_bounds` `[0, 2]` node over the `composite_type`/`map` node of the `Text` node and the data-part `Timer` node, and the bare `map` node for `{lower: 0}`. Its only tag is `tc_367_selected_package_assertions_each_fail_on_their_mutation`, which checks the `ClockMap` row's kind and bounds and the key join. No vector or test builds or checks the nodes for `M`, and its Verification cell names TC-234, whose I35/I36 rows cite only FR-154-AC-48/49. Fix: add a vector (in `package_map_types` or `model-member-type-vectors.json`) with the expected node paths for `M` with `{lower: 0, upper: 2}` and with `{lower: 0}`, and tag the test that checks them with FR-322-AC-396. Otherwise, drop the node claims from the AC until such a vector exists. | spec/objects/interfaces/FR-322-checked-package-artifact.md:4684; tests/checked_package_v2.rs:19835 |
| FND-003 | low | `tests/package_map_types.rs` asserts fixture bookkeeping, not behaviour. It checks `cases.len() == 27`, IDs in `PM-NN` sequence, and that each case's `criterion` label matches its outcome. The checklist's "no test case counts … fixtures" rule covers this, and none of these asserts can catch a wrong map rule. Fix: delete the count, ID-sequence and criterion-label asserts at lines 224-226 and 233-238, and keep the per-case `read_map` comparison. | tests/package_map_types.rs:224; tests/package_map_types.rs:233 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | not-a-defect | FR-415-AC-27 and the FR-144-AC-26 map clause are language criteria that the QSL checker carries (TC-367 compiler acceptance), the same as FR-415-AC-10 to AC-20, which QSpec's qualification tests do not tag. The static part this repository can run, the key refusals, is FR-154-AC-49, which `tc_234_package_map_types_declare_a_scalar_key_and_a_bound` tags. Tagging AC-27 with a test that checks only its refusals would overstate coverage. |
| FND-002 | fixed | FR-322-AC-396 now claims only what the vectors check: the row rendering `Map<Text, ix://acme/maps/Timer>` with minimum and maximum from `bound` (PM-01, PM-02) and the ClockMap join. The package map test now carries FR-322-AC-396. The node mapping stays in the normative step 4 row. |
| FND-003 | fixed | The count, ID-sequence and criterion-label asserts are deleted. The test compares only each case's reader outcome. |
