---
id: SR-1297
title: "Gap analysis of PR #189: QSL-635 real structural keys and tampered-range negative"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@07cc7cf2f663b8b8c739ba5251ea7b59acf60fd6; PR #189 diff against origin/main 2b2dd28; acceptance taken from Linear QSL-635 and its 2026-10-05 scope comment; tests TC-233 (tests/checked_package_v2.rs)"
review_set: subset
---
# Gap analysis of PR #189 (QSL-635)

## Summary

Ticket: QSL-635. No plan bundle exists for this ticket, so the acceptance
criteria are the ticket's asks, checked against the code at 07cc7cf.

| Ask (QSL-635) | Result |
| --- | --- |
| 1. Every positive fixture's structural keys are real FR-092 derivations; projections and package ids recomputed | Met. No repeated-byte node key remains. An independent recomputation matched every structural node. `tc_233_positive_fixtures_carry_derived_structural_keys` and the FR-322-AC-14 loop check keys and package ids |
| 2. node-identity-vectors.json holds real vectors | Met. Only the deliberate `absent-entry` `ffff...` remains |
| 3. A tampered `integer_range` negative refuses `stale-node-key` | Met. Four `derived_shape_key_mutations` rows, each asserted `StaleNodeKey` at the tampered node |
| 4. QSL's emitter produces real keys for `Int[0,1000]` | Claimed in the PR body and not re-run here. The operation-identities range key recomputes from its FR-092 preimage |
| Scope add: recursive-record positive (Option field, `Sequence<T>[0, n]` field) with real in-group keys from QSL's emitter | Met as data. The five group members equal QSL FR-092 vectors G2, G3, G7, G8 and G9, and `tc_233_recursive_record_fixture_carries_derived_group_keys` recomputes the keys and group labels. Partly unmet as coverage: see FND-001 |

Bindings checked: the three new tests trace `TC-233`/`FR-322-AC-10`; see
SR-1296 FND-002 for the code mismatch with that AC's text.

## Verdict

**Request changes.** One medium coverage gap and one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The recursive fixture is left out of every all-positive-fixtures check. It is not read through `read_package_semantics` (admission), and it is absent from `tc_233_positive_fixtures_carry_derived_structural_keys` and the FR-322-AC-14 package-identity loop. Its own test checks keys and identity, but no test shows the reader admits it. The scope comment's purpose was "so IR's conformance run covers in-group re-derivation", yet nothing refuses an in-group tamper. It should join the admission loop, and it should get a tamper row on an in-group `collection_bounds` (SR-1296 FND-001). | tests/checked_package_v2.rs:9760-9790, tests/checked_package_v2.rs:3971-3982 |
| FND-002 | low | The tamper rows all patch the one `integer_range` of operation-identities. The reader stage also re-derives `collection_bounds`, `option`, `reference` and the collection composites, but no row tampers any of them, so those branches of `is_derived_shape` have no failing test. | proposals/checked-package-v2/fixtures/adverse.json:17-21 |
