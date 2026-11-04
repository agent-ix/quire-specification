---
id: SR-617001
title: 'Spec integrity analysis of PR #617: IR-1322 M1/V3/V4/V5/V7/L19/C12 locus and stage rulings (QSL-1366)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@a85c7fb; git diff origin/main...a85c7fb (PR #617): FR-464 Single listing, Canonical order, FR-464-AC-2; TC-946 V3 V4 V5 V7 C12 M1 and Expected Results 2; TC-948 L19 and Expected Results 1; protocol-node-vectors.json; protocol-node-mutation-manifest.json; ticket QSL-1366'
review_set: subset
date: '2026-11-04'
relationships:
- target: ix://agent-ix/quire-specification/FR-464
  type: reviews
- target: ix://agent-ix/quire-specification/TC-946
  type: reviews
- target: ix://agent-ix/quire-specification/TC-948
  type: reviews
---

## Summary

Ticket: QSL-1366. PR: quire-specification#617, head a85c7fb. Manifest, TC-946, TC-948 and the vectors file agree on every changed row (M1 at /semantic_graph/nodes/59; V3 and V4 schema malformed_wire at nodes/59; V7 and L19 at nodes/64; V5 at nodes/78; C12 invalid_semantic_graph at nodes/78). The old wordings 'at the clause' for M1 and invalid_model_binding for C12 are gone from the changed artifacts. Two paragraphs of FR-464 that the PR did not touch still state the pre-ruling behaviour for the same shapes, and FR-464-AC-19 gives a different locus than the amended AC-2.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-464 Single listing | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:308 | examined |
| FR-464 Canonical order | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:190 | examined |
| FR-464 Placement and profile fit | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:534 | examined |
| FR-464-AC-2 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:703 | examined |
| FR-464-AC-19 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:720 | examined |
| FR-464-AC-24 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:725 | context_only |
| TC-946 V3 V4 V5 V7 C12 M1 | spec/test-cases/TC-946-checked-package-protocol-node-family.md | examined |
| TC-948 L19 | spec/test-cases/TC-948-checked-package-protocol-control-bodies.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-464 'Placement and profile fit' still says a nested application refuses malformed_wire 'at the nested application', a root application of another form refuses ill_typed/operator-ineligible, and an operator the profile does not define refuses operation-member-mismatch, with no schema-first qualification; this contradicts amended AC-2, V3, V4, V5, V7, L19 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:536 |
| FND-002 | medium | FR-464-AC-19 (and FR-341 line 302) put the malformed_wire locus of a schema-failing protocol.control application at operation/member/operator, while amended AC-2 and AC-24 record the node pointer for the same class of document | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:720 |

## Verdict

Not mergeable until FND-001 and FND-002 are resolved (amend the Placement bullets and reconcile AC-19's locus with the node-pointer convention). Everything the PR did change is internally consistent.
