---
id: SR-2026103103
title: "failure-domain review of STD-563 (QSpec PR #549)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification PR #549 (branch std-563-grouped-carrier, from origin/main da7ecb37); spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md, spec/test-cases/TC-946-checked-package-protocol-node-family.md, proposals/checked-package-v2/fixtures/protocol-node-vectors.json, proposals/checked-package-v2/schema.json, tests/checked_package_v2.rs"
review_set: all
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-464
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-946
    type: reviews
---

## Summary

Failure modes of the new check: absent-edge cycles, duplicate carriers, first-member unlabelled, deep cycles, label reuse. The extension-point and purity items of the checklist do not apply (no user callbacks).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 carrier and cycle check | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-464 cyclic component | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md | examined |
| TC-946 K16..K23 | spec/test-cases/TC-946-checked-package-protocol-node-family.md | examined |
| vectors | proposals/checked-package-v2/fixtures/protocol-node-vectors.json | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Identity: a grouped carrier's `node_id` is producer-assigned and no longer carries its target, so two carriers of one target no longer collide; the PR kept 'one carrier per target' with nothing that enforces it. | spec/objects/interfaces/FR-322-checked-package-artifact.md |
| FND-002 | medium | Topological robustness: a cycle check with no stated algorithm invites recursion over a graph the same artifact lets reach 100,000 nodes (AC-281). | spec/objects/interfaces/FR-322-checked-package-artifact.md |
| FND-003 | low | Edges to an absent node: the check ignores them so the later join that owns the defect reports it; confirmed stated, no finding beyond confirming the order. | spec/objects/interfaces/FR-322-checked-package-artifact.md |

## Verdict

All findings are fixed or accepted on the branch; see Dispositions.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | fixed: a second carrier of an earlier carrier's target refuses `invalid_semantic_graph` at the later carrier's `/node_id`, before the cycle check (FR-322-AC-286, K23) |
| FND-002 | fixed | fixed: iterative, linear in nodes and edges, no native call-stack recursion; verification is by the reader's own tests, outside this spec repo |
| FND-003 | no change | no change |
