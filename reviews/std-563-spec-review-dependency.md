---
id: SR-2026103106
title: "dependency review of STD-563 (QSpec PR #549)"
type: SpecReview
analysis: dependency
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

Ordering: the cycle check precedes the owner join and the carrier join, and follows node-id, dependency-list and dangling-reference checks; the order is now in the total-order list.

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
| FND-001 | low | FR-322 now points at FR-464-AC-15 only through FR-464's own citation of FR-322-AC-285; the AC-15 duplication that made the dependency circular was removed. | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md |

## Verdict

All findings are fixed or accepted on the branch; see Dispositions.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed with the AC-15 change | fixed with the AC-15 change |
