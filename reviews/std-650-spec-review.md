---
id: SR-20261104011604
title: 'Spec review of PR #602: wire ProtocolControlApplication into the ProtocolNode body rule (STD-650)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification; commit c63cde3fd (head 0570a560): FR-464 reader-order paragraph, proposals/checked-package-v2/schema.json, protocol-node-layout-rows.json, tests/protocol_node_layout_rows.rs; ticket STD-650'
review_set: subset
date: '2026-11-04'
---

## Summary

Ticket: STD-650. PR: quire-specification#602. The schema rule matches the added FR-464 paragraph and FR-466 layout text. Mutation check: removing the ProtocolNode if/then rule fails both new tests (26 rows report no first error). cargo test --locked --offline is green. quire validate exits 0 on FR-464 and exit 3 repo-wide, with output identical to main apart from line numbers (FR-200, FR-220, FR-299). Two documentation and coverage findings.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-464 reader-order paragraph | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:578 | examined |
| FR-464-AC-1 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:700 | examined |
| FR-466-AC-3 | spec/objects/interfaces/FR-466-checked-package-protocol-control-bodies.md:343 | context_only |
| protocol-node-mutation-manifest.json pending rows | proposals/checked-package-v2/fixtures/protocol-node-mutation-manifest.json:8750 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The test doc comment lists FR-466-AC-1, AC-3 and AC-5 as covered, but the #[trace] tags carry only TC-946/947/948, FR-464-AC-1 and FR-464-AC-5. The FR-466 criteria are not mapped in the matrix, and the new FR-464 prose adds a schema-refusal obligation with no AC stating it. | tests/protocol_node_layout_rows.rs:92 |
| FND-002 | low | protocol-node-mutation-manifest.json still states the schema references ProtocolControlApplication nowhere and wiring fails the base (pending rows V12.. RP4). After this PR that statement is false. | proposals/checked-package-v2/fixtures/protocol-node-mutation-manifest.json:3 |
| FND-003 | low | Positive coverage is the base plus one L12 pair; admitted spellings of event, await and the other enumerations get no positive row, so over-tightening of the schema would not be caught. | tests/protocol_node_layout_rows.rs:100 |

## Verdict

Mergeable. No high findings. FND-001 should be fixed in the PR (trace tags or an AC); FND-002 and FND-003 may be deferred.
