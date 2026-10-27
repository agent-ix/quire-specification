---
id: SR-498001
title: 'Spec review of quire-specification PRs #456 and #457 (STD-498): owner_model is the object-type node of a model owned by a selected declaration'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@fb92162a (PR #456, spec/std-498-owner-model) and @d8bae322 (PR #457, spec/fix-model-root-owner-rule); FR-375 class-catalog paragraph and FR-375-AC-23, FR-464 owner_model bullet and FR-464-AC-16, fixtures positive-all-families, positive-timed-model-nodes, positive-tick-timed-model-nodes, positive-clause-operations, dependency-selection-vectors.json, timed-model-node-vectors.json TM-23, tests/checked_package_v2.rs, proposals/checked-package-v2/schema.json NodeBase'
review_set: subset
date: '2026-10-27'
relationships:
- target: ix://agent-ix/quire-specification/FR-375
  type: reviews
- target: ix://agent-ix/quire-specification/FR-464
  type: reviews
---

## Summary

Ticket: STD-498. PRs: quire-specification#456 (head fb92162a) and #457 (head d8bae322). Planner ruling (decided): FR-464 governs; `owner_model` is the object-type node of a model owned by a selected declaration.

Examined (all `examined` unless noted):
- FR-375 class-catalog paragraph (spec/functional/analysis/FR-375-declare-model-refinement.md:494-506)
- FR-375-AC-23
- FR-464 `owner_model` bullet (FR-464:68-74) and FR-464-AC-16
- FR-464 clause paragraph "The clause's model owner is the separate node-level `owner_model` edge" (FR-464:194-199)
- FR-464 Level paragraph (FR-464:311-321), context_only for the finish-binder check
- FR-464-AC-18 (FR-464:527)
- TC-946 OM-01 (spec/test-cases/TC-946-checked-package-protocol-node-family.md:144) and protocol-node-vectors.json OM-01
- the four repaired fixtures, dependency-selection-vectors.json, TM-23, tests/checked_package_v2.rs diff
- schema.json NodeBase.allOf (#457)

Independent recomputation: a Python JCS script mirroring `application_node_preimage`, `structural_node_preimage` and `package_id_preimage` recomputes every node key and the package id of all four fixtures at fb92162a. All keys and all four package ids match; dependency lists are digest-ascending and every dependency and source-map id names a graph node. The three nodes per fixture that use other preimages (dimension, unit nominal nodes; the correspondence node of clause-operations) are the same on main. dependency-selection-vectors.json carries the all-families (ffcf3fe1...) and clause-operations (6c4a9ded...) ids. FR-375 and FR-464 criterion ids are identical to main. `quire validate` on FR-375 and FR-464 exits 0 (one warning, unchanged from main); FR-322 exits 3 on both main and the PR with the same missing `id` error (PR does not touch it).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375 class-catalog paragraph | spec/functional/analysis/FR-375-declare-model-refinement.md:494 | examined |
| FR-375-AC-23 | spec/functional/analysis/FR-375-declare-model-refinement.md:686 | examined |
| FR-464 owner_model bullet | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:68 | examined |
| FR-464-AC-16 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:525 | examined |
| FR-464 clause model owner paragraph | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:194 | examined |
| FR-464-AC-18 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:527 | examined |
| FR-464 Level paragraph | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:311 | context_only |
| TC-946 OM-01 | spec/test-cases/TC-946-checked-package-protocol-node-family.md:144 | examined |
| protocol-node-vectors.json OM-01 | proposals/checked-package-v2/fixtures/protocol-node-vectors.json:2733 | examined |
| positive fixtures and dependency vectors | proposals/checked-package-v2/ | examined |
| schema.json NodeBase.allOf | proposals/checked-package-v2/schema.json:116 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-464 still says the `owner_model` target is "the exact local model declaration node represented by the clause's source `over` type" (also TC-946 OM-01 and protocol-node-vectors.json OM-01), but the amended bullet and the repaired fixtures make the owner an `acme/orders` Order node while the `over` binder in each fixture is typed by a different object type (the source-local ConfigVersion). Two readings of one edge coexist. | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:194 |
| FND-002 | low | The new FR-464 sentence "No selected declaration owns a model declaration node of an unselected package, so such a node is never a valid target" is circular and does not describe the removed case: the old target Config::ConfigVersion is a source-local node, not a node of an unselected package. "Selected declaration" is not defined where FR-375/FR-464 use it. | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:70 |
| FND-003 | low | FR-464-AC-18 still says `owner_model` is "the exact declaring model node"; FR-322:508 and TC-310 CC-14 ("owned by the selected concrete model") keep the old phrasing, so the ruling's wording is applied to FR-375/FR-464 bullet only. | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:527 |

## Verdict

#456: the hand-computed keys and package ids are correct; nothing was fudged. The finish-binder repair is right: FR-464 Level gives `finish` an entry count of the binders readable after the root control, which is `over` alone (1) in these fixtures, so a distinct `closing` binder at level 1 conforms and the old reuse of level-0 `over` did not. Criterion ids are stable. Mergeable once FND-001 is fixed (or ruled a follow-up by the planner); FND-002 and FND-003 are wording nits. #456 requires #457 to compile the schema, so #457 merges first.

#457: clean. The removed `{"$ref":"#/$defs/ModelRootOwnerRule"}` was never defined in #454 (no `$defs` entry in any revision), so no validator could compile the schema. Both directions of the FR-322 owner rule are enforced in the same allOf: the inline "owner is ModelRootOwner implies model/model_root" and OwnerPresenceRule's model_root branch (model_root requires ModelRootOwner). The identity/digest join to a `model_selections` row is not expressible in JSON Schema and is a reader check (FR-322). No intended constraint is lost; the schema has no dangling `$defs` reference after the change.

Cross-repo follow-ups, not defects of these PRs: IR FR-064-AC-4 ("a clause declared `over` model A emits A's exact local declaration edge even when selected model B exists") and QSL FR-383-AC-26 ("bound by the checker for its `over` subject") still describe the edge as derived from `over`; IR PR #600 changes FR-064-AC-5 only.
