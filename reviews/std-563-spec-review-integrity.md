---
id: SR-2026103102
title: "integrity review of STD-563 (QSpec PR #549)"
type: SpecReview
analysis: integrity
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

Consistency of the new rule with main. (1) Code, stage and locus: `invalid_semantic_graph` with no cause at the graph-shape step before the owner join is the convention main uses for every other graph-shape defect (dependency list, duplicate node id, owner-root duplicate) and AC-153 requires no cause for that code; main's text said only 'all other graph cycles refuse' with no code, so the PR supplies it, and the locus convention (node, or its member when a different member is the defect) matches. Contract IR FR-038 already uses the same locus ('at its recursion_group when it carries a different one, else at the node') but says 'lowest-positioned', which must become ascending node-id order. (2) A label on an acyclic carrier changes from `malformed_wire` (the V2 schema's `not anyOf required recursion_group` on the carrier, and the carrier sentence that listed no `recursion_group`) to admitted. This is the same rule main already states for every node and for K10 (acyclic protocol node plus lone label is admitted); no test or vector on main expected the old refusal (grepped tests, TC-943, TC-946, vectors). It is a real behaviour change for Contract IR, which implements `malformed_wire`. (3) Disjoint cycles reusing a label: admitted, see FND-003. I ran the new cycle rule over every fixture package and vector node set in proposals/: the five cyclic components (List, List/Tree, two-owner fixtures) all carry one label, none violates; the model tuple/union cycles are cycles of the selected document, not graph edges, and are untouched.

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
| FND-001 | high | The cycle check names `invalid_semantic_graph`/no cause/graph-shape step, but FR-322's total join order (stage 1) listed only the dependency checks and the dangling reference, so the order text and the new check disagreed. | spec/objects/interfaces/FR-322-checked-package-artifact.md |
| FND-002 | medium | The cyclic-component edge set was stated by pointer to the `dependencies` item and a parenthesis about `semantic_type`, which left own-path index segments and `semantic_type` edges ambiguous, and the check order against the dangling-reference check unstated. | spec/objects/interfaces/FR-322-checked-package-artifact.md |
| FND-003 | medium | Two disjoint cyclic components that reuse one label were unspecified. By the rule each is compared with its own first member, so both are admitted, but membership is the label alone, so the 'exactly one declared member' path anchor then counts the declared members of both. | spec/objects/interfaces/FR-322-checked-package-artifact.md |
| FND-004 | medium | 'The reference body and target remain part of its structural identity, so a body edit does not change the id' contradicted itself for a grouped carrier, whose id is not a function of its body. | spec/objects/interfaces/FR-322-checked-package-artifact.md |
| FND-005 | low | FR-322-AC-284 said 'two-occurrence behaviour' of AC-66 while K17 has one occurrence. | spec/objects/interfaces/FR-322-checked-package-artifact.md |
| FND-006 | low | 'MUST carry ... a producer SHALL write it and a reader requires it' mixed modals. | spec/objects/interfaces/FR-322-checked-package-artifact.md |

## Verdict

All findings are fixed or accepted on the branch; see Dispositions.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | fixed: stage (1) now names the cycle check, last |
| FND-002 | fixed | fixed: edges are a non-null foreign `semantic_type`, each `dependencies` entry and each body `reference` target; index segments are not edges (AC-280 refuses them earlier); the check runs after node-id, dependency-list and dangling-reference checks |
| FND-003 | fixed | fixed: stated as admitted with the anchor consequence; K22 and a test assertion added |
| FND-004 | fixed | fixed: body and target are part of the package value, not of a grouped carrier's id |
| FND-005 | fixed | fixed |
| FND-006 | fixed | fixed: reader SHALL require |
