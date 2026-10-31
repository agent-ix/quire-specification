---
id: SR-2026103108
title: "risk-complexity review of STD-563 (QSpec PR #549)"
type: SpecReview
analysis: risk-complexity
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

Risk is concentrated in the wire change to the carrier and in consumers that have not yet moved.

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
| FND-001 | medium | Technical risk: the schema change relaxes a closed refusal that Contract IR implements (`malformed_wire` for an acyclic carrier label); a reader that is not updated will refuse documents this spec now admits. | spec/objects/interfaces/FR-322-checked-package-artifact.md |
| FND-002 | low | Volatility: the label-reuse and duplicate-carrier decisions were not in the ticket; recorded as rules with vectors so the owner can reverse them cheaply. | spec/objects/interfaces/FR-322-checked-package-artifact.md |

## Verdict

All findings are fixed or accepted on the branch; see Dispositions.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | mitigated | mitigated: listed in the post-merge change list |
| FND-002 | accepted | accepted |
