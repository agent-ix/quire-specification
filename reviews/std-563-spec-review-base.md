---
id: SR-2026103101
title: "base review of STD-563 (QSpec PR #549)"
type: SpecReview
analysis: base
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

Base checklist, ID formats and the six coverage rules. No collision: FR-322-AC-283..285 and TC-946 K16..K21 are free on origin/main da7ecb37 and in the only open PR (#549 itself). `quire validate` shows the same 2 FR-322 errors and the same warnings on origin/main and on the branch (full text diffed, identical). No hash, digest or pin was added.

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
| FND-001 | medium | Vectors K16..K21 gave the `protocol_reference` carrier `R` an `owner`, which the carrier rule and the V2 schema forbid, and the vector node sets held no `dependencies` or body edges on `Flow`, `Holder` or `joinPred`, so no cycle existed in the data and the Rust helper only compared labels. | proposals/checked-package-v2/fixtures/protocol-node-vectors.json K16..K21, tests/checked_package_v2.rs |
| FND-002 | low | FR-322-AC-283..285 named TC-233 as a test case, but TC-233 carries none of K16..K21 and does not verify them; TC-946 does. | spec/objects/interfaces/FR-322-checked-package-artifact.md |
| FND-003 | low | FR-464-AC-15 gained a second and third behaviour (refusal and acyclic admission) that FR-322-AC-284/285 already own. | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md |
| FND-004 | low | The test's `refused_cycle_member` ordered by path only, ignoring `package`, against the node-id order the spec defines. | tests/checked_package_v2.rs |
| FND-005 | low | The new test lines were not rustfmt-clean (edition 2024, four hunks). | tests/checked_package_v2.rs |

## Verdict

All findings are fixed or accepted on the branch; see Dispositions.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | fixed: `owner` removed from every carrier; `dependencies` edges Flow>Holder>joinPred>R>Flow added; the test now computes cyclic components, asserts one component of four, and applies the rule only inside components |
| FND-002 | fixed | fixed: rows now cite TC-946 only |
| FND-003 | fixed | fixed: AC-15 keeps the membership statement and points at FR-322-AC-284/285 |
| FND-004 | fixed | fixed: `node_id_order_key` orders by package, path elements, ordinal |
| FND-005 | fixed | fixed: touched lines only; pre-existing drift elsewhere left alone |
