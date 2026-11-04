---
id: SR-617002
title: 'Code review of PR #617: protocol_node_mutation_manifest tests (QSL-1366)'
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-specification@a85c7fb; tests/protocol_node_mutation_manifest.rs (diff origin/main...a85c7fb); ticket QSL-1366'
review_set: subset
date: '2026-11-04'
relationships: []
---

## Summary

Ticket: QSL-1366. cargo test --locked --offline passes (exit 0). Both new tests fail when mutated in a scratch copy (M1 locus set back to nodes/82 fails the first; V5 locus changed to nodes/80 fails the second), so they are not vacuous. Two low findings on traceability.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| schema_first_placement_rows_and_the_single_listing_row_pin_their_loci | tests/protocol_node_mutation_manifest.rs:726 | examined |
| nested_application_and_unheld_root_rows_pin_their_stage_and_locus | tests/protocol_node_mutation_manifest.rs:775 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The first test pins M1 (the Single listing row, FR-464-AC-6) but its trace tag lists FR-464-AC-2 and FR-466-AC-1 only; quoin matrix shows FR-464-AC-6 bound to the second test alone | tests/protocol_node_mutation_manifest.rs:721 |
| FND-002 | low | The tests check the manifest against the schema and against itself; nothing compares the changed Expected cells of protocol-node-vectors.json or the TC-946/TC-948 tables to the manifest rows, so the prose drift this PR repairs (and FND-001 of SR-617001) is unguarded | tests/protocol_node_mutation_manifest.rs:726 |

## Verdict

Mergeable on the code side; both findings are low and may be accepted.
