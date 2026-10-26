---
id: SR-17125
title: "Spec review of quire-specification PR #436 (QSL-856): regenerate protocol package fixtures"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@b1ccf7df; PR #436 diff against main 69d75ea1; proposals/checked-package-v2 fixtures (positive-all-families, positive-clause-operations, positive-timed-model-nodes, positive-tick-timed-model-nodes, protocol-node-vectors), schema.json, FR-322 worked example, FR-464, TC-946, TC-947, TC-948; read by quire-contract-ir@9a84e65 and quire-spec-language pin quire-contract-model 6f6b5621"
review_set: subset
---
# Spec review of quire-specification PR #436

## Summary

Ticket: QSL-856. The PR replaces the stub `protocol_clause` body with a thirteen-binding `quire.op.protocol.control` body in four positive fixtures, adds `protocol-node-vectors.json` (355 rows), updates the schemas, the FR-322 worked example and `tests/checked_package_v2.rs`.

Examined:
- FR-464 text row of the operand-shape table, line 127 (examined)
- FR-464-AC-1, FR-464-AC-5, FR-464-AC-15 (examined)
- TC-946, TC-947, TC-948 vector tables against `protocol-node-vectors.json` `cases` (examined)
- `protocol_clause` node 9 in four positive fixtures (examined)
- FR-322 worked example edit (examined)
- quire-contract-ir `tests/conformance_qspec` and quire-spec-language `checked_v2` tests (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The `activation.kind` text literal of the protocol clause node carries `type` = the `scalar_type`/`boolean` node (node 0, digest 2c8a0dce...) instead of the `scalar_type`/`text` node (node 23, 0062ac9e...). FR-464 line 127 says a text literal's `type` is the text node. The real IR reader (quire-contract-ir 9a84e65) and the QSL pin (6f6b5621) both refuse positive-all-families.json with `ill_typed`/`operator-ineligible` at `/semantic_graph/nodes/9/body/arguments/1/value/members/0/value/type`. The same defect is in positive-timed-model-nodes.json and positive-tick-timed-model-nodes.json (node 9). So the fixtures are not admitted and the 12 QSL checked_v2 conformance tests still fail (14 checked_v2 failures in all). Retype the literal to the text node and recompute node and package keys. | proposals/checked-package-v2/fixtures/positive-all-families.json:node 9 |
| FND-002 | medium | `protocol-node-vectors.json` has no row for TC-946 K10 (acyclic protocol node adds `recursion_group` with `recursion` null: `malformed_wire`), added to main by #441 and merged into this branch; the test pins `[108, 85, 162]`. All other 355 rows match TC-946/947/948 text exactly. FR-464-AC-15 (Option A cyclic components) has no vector either. | proposals/checked-package-v2/fixtures/protocol-node-vectors.json |
| FND-003 | medium | The regenerated fixtures shift node indexes; quire-contract-ir `timed_model_tm_23_to_tm_29...` (TM-23) expects `/semantic_graph/nodes/49/...` and now sees `/nodes/51/...`. This failure is new relative to main fixtures (17 conformance_qspec failures on main, 18 on the PR, with the stale shape). The consumer test needs the new index with this PR, or the fixture should keep the index. | proposals/checked-package-v2/fixtures/positive-timed-model-nodes.json |
| FND-004 | low | No generator for the 355 rows or the recomputed keys is checked in, so regeneration cannot be re-run and diffed. The rows were checked here against the TC tables and match; key correctness depends on the reader admitting them (see FND-001). | proposals/checked-package-v2/fixtures/protocol-node-vectors.json |

## Verdict

Not mergeable. FND-001 is a real fixture defect, found by the real readers: the thirteen-argument shape and operator are the admitted ones, but one literal has the wrong type node, so nothing is admitted. Verified: 355 vector rows equal the TC-946/947/948 rows (ids, sections, edits, expected) except the missing K10; no leftover stub shape remains (`protocol_control` occurrences are the operator class named by FR-464); `cargo test --locked` in QSpec passes except `tc_280_the_domain_package_projection...`, which fails identically on main 69d75ea1; `quire validate` on FR-322 reports the same two warnings and failure as main. quire-contract-ir `conformance_qspec` does not compile at 9a84e65 (non-exhaustive match on `WrongExport`, a separate IR defect); it was run with a one-line wildcard arm in a scratch copy. The QSL result is a fixture problem, not a pin problem: the pin 6f6b5621 gives the identical refusal.
