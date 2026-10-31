---
id: SR-2026103104
title: "scope-boundary review of STD-563 (QSpec PR #549)"
type: SpecReview
analysis: scope-boundary
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

Responsibility: FR-322 owns the wire and the reader's check; FR-464 owns the protocol-node consequence; QSL (producer) writes the label and ordinal; Contract IR (reader) implements the check. The PR keeps that split.

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
| FND-001 | medium | The PR states the wire and reader rules but not which consumer owns which change; Contract IR (reader) and QSL (producer) both hold contrary text. | spec/objects/interfaces/FR-322-checked-package-artifact.md |
| FND-002 | low | FR-464's sentence that FR-322's validation 'assigns and validates the group' places assignment in the reader; the producer assigns. Pre-existing and outside this diff. | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md |

## Verdict

All findings are fixed or accepted on the branch; see Dispositions.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed in this review's report | fixed in this review's report: exact post-merge edits listed for quire-contract-ir FR-038/AC-241 and quire-spec-language FR-383 |
| FND-002 | not changed | not changed: pre-existing, out of scope |
