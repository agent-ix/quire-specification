---
id: SR-2026103107
title: "evidence review of STD-563 (QSpec PR #549)"
type: SpecReview
analysis: evidence
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

Evidence for the new criteria: vectors plus Rust conformance over them. `cargo test --locked` passes (142 tests in checked_package_v2, all other suites ok).

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
| FND-001 | medium | Method for AC-283..286 is 'Test (TC-946)' but the vectors are descriptive node sets, and there is no reader in this repository; the Rust test checks the vectors against the stated rule, not a reader. | spec/objects/interfaces/FR-322-checked-package-artifact.md |
| FND-002 | low | Algorithmic complexity of the cycle check is asserted in text and cannot be tested here. | spec/objects/interfaces/FR-322-checked-package-artifact.md |

## Verdict

All findings are fixed or accepted on the branch; see Dispositions.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | accepted | accepted: this repo's convention for vector-defined rules; the reader tests belong in Contract IR |
| FND-002 | accepted | accepted |
