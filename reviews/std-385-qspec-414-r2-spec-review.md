---
id: SR-16257
title: "Round-2 spec-review of the hybrid model node fix-forward (quire-specification PR 414)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@39d4bb9fd90e7d7c80c8228305c32ce451f988a3 (fix commit c33db58b against SR-16255; origin/main...HEAD); spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md; spec/test-cases/TC-966-checked-package-hybrid-model-nodes.md; spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md; proposals/checked-package-v2/fixtures/positive-all-families.json; proposals/checked-package-v2/dependency-selection-vectors.json; tests/checked_package_v2.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-481
    type: references
  - target: ix://agent-ix/quire-specification/TC-966
    type: references
  - target: ix://agent-ix/quire-specification/TC-233
    type: references
---

# Round-2 spec-review of the hybrid model node fix-forward (quire-specification PR 414)

## Summary

Ticket: STD-385. This round re-reviews #414 at head 39d4bb9f: the fix
commit c33db58b against SR-16255, plus a merge of main. Main is an
ancestor of the head. The companion QSL round is SR-16258.

Gates run by the reviewer:

- `quire validate --scope . 'spec/**/*.md'`: 86 failure lines, the
  baseline.
- `cargo test`: `checked_package_v2` 134 passed, `backend_provider_v1` 24
  passed.
- QSL conformance `checked_v2` on QSL #1364 head b65ec862 with `QSPEC_DIR`
  at this head: 41 passed, 6 failed. Every failure is `UnsupportedNodeTag`
  at `/semantic_graph/nodes/39/node_tag`, the missing QSL reader code and
  nothing else.

Re-derived by hand:

- The `Tank` mode's invariant and the transition's guard are
  `8a2ac579…`, the literal `true` typed by the `scalar_type`/`boolean` node
  `Example.Flag`.
- Every `Tank` node key recomputes: transition `3b434ef3…`, mode
  `98181eaf…`, model `f02ca0d9…`, and the term and parameter nodes.
- The fixture `package_id` and the leaf id (`46947d64…`), the `test/a` id
  (`b2f3d176…`) and the top-level id (`bb96c0c4…`) recompute.

Dispositions of SR-16255:

- FND-001 is fixed. The new assertions in
  `tc_966_the_all_families_fixture_carries_a_hybrid_model_with_bare_variable_terms`
  check the term type of the invariant, the guard, each rate and each reset
  value.
- FND-002 is fixed in QSpec. The profile follows the `using` alias,
  intervals take the form FR-370 admits under that profile, check 6 drops
  its profile check, and AC-8/HY-13 admit a `timed/v1` claim in the timed
  form. QSL does not yet emit that form (SR-16258 FND-001).
- FND-003 is fixed. TC-233 now names both hand-authored sets and the key
  rule for each.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | [coverage] Rewriting AC-8, HY-13 and HY-14 dropped every vector for checks 6 and 7, which still stand in the text. Before c33db58b, AC-8 and HY-13/HY-14 pinned clock `model-steps` (`invalid-value` at `/body/arguments/1`), an `each` activation (`invalid-value` at `/body/arguments/2`) and a `once` formula node (`ill_typed`/`operator-ineligible`). These were AC-6's refusals before round 1. No AC, TC row or test in either repo now names `model-steps`, `each` or `once` for a hybrid claim, and no vector covers a non-empty captures or fairness argument. QSL FR-447-AC-6 keeps only the `model-steps` case. Fix: restore the three refusals, and add one for non-empty captures or fairness, as AC rows and HY vectors next to the new `timed/v1` admission. | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:258-268; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:303; spec/test-cases/TC-966-checked-package-hybrid-model-nodes.md:53-54 | correct-requirement-no-evidence |

## Merge order

#414 still cannot merge before QSL's reader code admits the `hybrid` tag.
QSL's conformance reads the sibling QSpec checkout, so merging #414 earlier
turns six `checked_v2` tests red on QSL main. Merge it in the same window
as the QSL reader-code PR: QSpec first, then the QSL code PR at once. The
fixture defect that blocked that order is fixed.

## Verdict

Not mergeable yet. There is one medium finding (FND-001), dropped coverage
of the claim-form and operator checks. The QSpec gates are green. The
merge must wait for the QSL reader code, and QSL #1364 has a high finding
(SR-16258 FND-001).
