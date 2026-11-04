---
id: SR-17127
title: 'Spec review of PR #615: delivery-interval malformed_wire locus (STD-650)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@245a34b; git diff origin/main...245a34b (PR #615, branch fix/std-650-delivery-locus): FR-465 delivery bullet and FR-465-AC-10, FR-464 enumerated text values sentence, TC-947 CH14a; ticket STD-650'
review_set: subset
date: '2026-11-04'
relationships:
- target: ix://agent-ix/quire-specification/FR-465
  type: reviews
- target: ix://agent-ix/quire-specification/FR-464
  type: reviews
- target: ix://agent-ix/quire-specification/TC-947
  type: reviews
---

## Summary

Ticket: STD-650. PR: quire-specification#615, head 245a34b. The three edits agree with each other and with FR-464-AC-24 (merged #602) and the layout-rows fixture: the delivery/enumerated-spelling locus is the node pointer /semantic_graph/nodes/{i} at the schema stage. Layout rows CH14, CH15, CH16, QU6, L12-L17, TM3, TM4 all record node pointers. A repo-wide grep finds one missed instance (FND-001) and one stale note (FND-002). No test reads the old wording: tests/protocol_node_mutation_manifest.rs only checks that a vector's Expected text names a refusal. The PR's schema-stage claim for the ten CH14a interval spellings has no fixture row and the shipped schema.json cannot decide them (FND-003). quire validate on the three changed files exits 0 (one pre-existing FR-464 line 717 warning); the repo-wide baseline exits 3 with FR-200, FR-220 and FR-299 diagnostics. cargo test --locked --offline passes. TC range: TC-947 scope FR-465-AC-1 to AC-10 unchanged.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-465 delivery bullet | spec/objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md:162 | examined |
| FR-465-AC-10 | spec/objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md:325 | examined |
| FR-464 enumerated text value sentence | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:186 | examined |
| TC-947 CH14a | spec/test-cases/TC-947-checked-package-protocol-declaration-bodies.md:89 | examined |
| FR-464-AC-24 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:723 | context_only |
| protocol-node-layout-rows.json delivery rows | proposals/checked-package-v2/fixtures/protocol-node-layout-rows.json | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | protocol-node-vectors.json CH14a Expected still says "refuses malformed_wire at the delivery term", contradicting the amended TC-947 CH14a | proposals/checked-package-v2/fixtures/protocol-node-vectors.json:2466 |
| FND-002 | low | protocol-node-mutation-manifest.json CH14a note still describes the spellings as "at the delivery term" | proposals/checked-package-v2/fixtures/protocol-node-mutation-manifest.json:8851 |
| FND-003 | medium | FR-465-AC-10 claims schema-stage refusal with a node-pointer locus for ten interval spellings, but no layout row records one and schema.json's delivery value is a four-word enum with no interval form | proposals/checked-package-v2/schema.json:10712 |
