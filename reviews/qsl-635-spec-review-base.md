---
id: SR-1298
title: "Spec review of PR #189 fix round: FR-322 derived-shape key stage"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@4bf7e9c834a6e29695621be5233aec8c19359d67; spec/objects/interfaces/FR-322-checked-package-artifact.md (derived-shape key stage, FR-322-AC-48..50), spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/complete-v1/tests.md"
review_set: subset
---
# Spec review of PR #189 fix round (QSL-635)

## Summary

Ticket: QSL-635. Spec text first changed in fix commit 9630eb4: FR-322
gains a normative derived-shape key stage, ACs 48-50, TC-233 steps and
results, and three Test Matrix rows. The stage's place in the reader order,
its two passes, its refusal locus and its in-group preimage match QSL FR-092
and the test reader (`stale_derived_shape_node`). AC-48, AC-49 and AC-50 are
each testable and name concrete mutations, and the three new tests trace them.
The matrix rows are present. `quire validate` adds no new failure class: the
145 failed documents on main are the pre-existing TestMatrix column
mismatches.

## Verdict

**Request changes.** One high finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The stage re-keys in-group structural nodes "declared or not and of any shape", but outside a group it re-keys only the undeclared derived shapes. An ungrouped declared structural node is never re-derived. QSL emits declared records (FR-092 D1), so all-families `Example::Point` qualifies. Re-pointing Point's `x` from `Integer` to the `Int[0, 9]` node, keeping its node id and recomputing projection and `package_id`, is admitted, and a consumer reads a narrowed field type: the IR-627 class one hop away. The test reader mirrors the spec (`is_derived_shape` requires no `declaration`), and no other stage keys a declared non-nominal node. Fix: re-key every ungrouped non-nominal, non-application structural node, owner from its declaration source occurrence, as the in-group set already does. The positive-fixture key test already shows every such fixture node is current. Add a declared-record tamper row. | spec/objects/interfaces/FR-322-checked-package-artifact.md:245-280, tests/checked_package_v2.rs:2061-2084, tests/checked_package_v2.rs:2216, proposals/checked-package-v2/fixtures/positive-all-families.json:16 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The `declared-record-field-repointed` row re-points `Example::Point`'s `x` at the `Int[0, 9]` node but leaves Point's `dependencies` as `[Integer]`. Every other re-point row also patches `/dependencies/0`. The test reader checks no structural dependency list, so the row refuses `stale-node-key` as intended. A reader that does check dependencies could refuse it on another cause first, so the conformance row is not a clean single-defect tamper. Add a patch that makes the dependencies the digest-ascending set of both targets. | proposals/checked-package-v2/fixtures/adverse.json:28 |
| FND-003 | low | Overlap between the stage and model declaration nodes. FR-322 defines a model declaration node as a `model`/`relation` node that "carries no `declaration` or whose `node_id` equals a model declaration node key of a selected domain package", and step 2 refuses a matched node that carries a `declaration` as `stale-node-key`. The stage excludes only the first branch, so it re-keys a declared model node with its source owner. The outcome is the same, because a `ModelOwner` preimage never equals a source-owner preimage: such a node refuses `invalid_package`/`stale-node-key` at the same node either way. The stage reports it earlier, before declaration, frame and state refusals, so step 2's "carries a `declaration`" branch can no longer fire through the full reader. The stage should win. The fix is to drop the "or whose `node_id` equals ..." branch from the `model_member` definition, or to say the stage subsumes it, so the two rules do not describe the same refusal at two reader positions. | spec/objects/interfaces/FR-322-checked-package-artifact.md:332-336, spec/objects/interfaces/FR-322-checked-package-artifact.md:538-544, tests/checked_package_v2.rs:2092 |

## Dispositions

Round 1, reviewed at 3f8b9224fe1d19415881f3d068db09de156868d5 (fix commit
3f8b922 over 4bf7e9c). The stage's first pass now visits every structural
node: no nominal preimage, no application in its body, and not an
undeclared `model`/`relation` node. Each node is checked against its
source-owned preimage, or its group-local preimage when grouped, and a
derived shape must also be self-typed. FR-322-AC-48 and FR-322-AC-50 now
cover every ungrouped structural node and add the `Example::Point` row. The
four reader tests that edit a node now rekey it through `rekey_stale_nodes`,
so they still reach the frame or state step they test. No build was run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 3f8b922 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Step 2 now says a node carrying a `declaration` or a `recursion_group` "is not a model declaration node" and the derived-shape stage re-keys it. The stage's own definition, the `model_member` definition and `is_rekeyed_structural_node` all leave out every `model`/`relation` node with no `declaration`, grouped or not. So the spec text no longer refuses an undeclared, grouped model node carrying a `ModelOwner` key in either place. FR-322-AC-29 and TC-280's `fixed_members_hold` (tests/model_member_types.rs:128-133) still refuse it at step 2. Fix with one line: keep "or that carries a `recursion_group`" in step 2's refusal list, and limit the "not a model declaration node" sentence to a declared node. | spec/objects/interfaces/FR-322-checked-package-artifact.md:539-546, tests/checked_package_v2.rs:2092-2098 |

Round 2, reviewed at 9b74bd9 (fix commit 9b74bd9 over 3f8b922). No build was run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed | 9b74bd9 |
| FND-003 | fixed | 9b74bd9 |

Round 3, reviewed at 0b7a317 (fix commit 0b7a317 over 9b74bd9, spec and reviews only). No build was run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | 0b7a317 |
