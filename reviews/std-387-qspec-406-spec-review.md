---
id: SR-16332
title: "spec-review of a state constraint as a state_clause node of clause constraint (quire-specification PR 406)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@024ea5e1; spec/objects/interfaces/FR-341; FR-322; spec/functional/analysis/FR-382; TC-233; schemas; tests/checked_package_v2.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/FR-382
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# Spec-review of a state constraint as a state_clause node of clause constraint (quire-specification PR 406)

## Summary

Ticket: STD-387. One spec-review pass over `git diff origin/main...024ea5e1`, checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across all open PR heads, cross-spec names, and mergeability against main.

Examined, clean:

- FR-341-AC-13 and the Rust test agree: CN-01 (admitted, different node id), CN-02 (operation-anchor refuses `malformed-declaration` at the anchor target), CN-03 (second parameter refuses `operator-ineligible` at the clause node). Both V2 schemas enumerate `constraint`, and QSL #1352 cites names that exist here (`state`/`state_clause`, clause `constraint`, "Constraint clause syntax" in FR-382, the FR-331 `constraint` locus already on main). No deferral or narrowing language was found.

Merge status: origin/main 0eef131f is not an ancestor, but `git merge-tree` against main is clean. No conflict with QSpec #403 to #421 apart from #398 in spec/spec.md (not touched here).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Sibling rules still enumerate three clause kinds and now disagree with FR-341's "a constraint is checked as an invariant". FR-151 allows a query-operation call "only inside invariant, precondition and postcondition blocks", so a call in a constraint body is `ill_typed`/`operator-ineligible` by that text. FR-146 limits `self` to "an invariant, precondition, postcondition or operation-body block", and `proposals/quire-v1/definitions/package-reference.md` makes `family.state` apply only when an invariant, precondition or postcondition is declared, so a constraint-only package is left without it. Fix: add `constraint` to those lists, or state the exclusion in FR-341. | spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md:190 | wrong-requirement |
| FND-002 | medium | The new request refusal ("A key that names no such node refuses the request `invalid_runtime_input`/`invalid-value`, naming the key") has no acceptance criterion or vector. FR-382-AC-1 and AC-3 cover other refusals, and FR-341-AC-13 covers only the node, not the request member. A key naming an invariant node, or no node, is exactly the case QSL FR-158-AC-6 relies on, so QSpec should own a testable AC for it. | spec/functional/analysis/FR-382-select-state-space-reductions.md:74 | missing-requirement |

## Verdict

**Mergeable after two medium findings are fixed or dispositioned**, and after main is merged (main moved; `git merge-tree` is clean). Gates on the head: `quire validate --scope . 'spec/**/*.md'` 86 failure lines (baseline); `cargo test --test checked_package_v2 --test backend_provider_v1` 133 + 23 passed. On the merge of main and the head: validate 86; cargo 140 + 24 passed; QSL conformance (`cargo test -p qsl-package --lib checked_v2`, QSL main, `QSPEC_DIR` the merge) 49 passed. Against the bare head, QSL main fails 2 hybrid-fixture tests (47 passed), which is skew from the head being behind QSpec main, not a #406 defect: the same QSL run against QSpec main passes 49. Ids FR-341-AC-13 and CN-01 to CN-03 are unique across 23 open heads. #398 conflicts with the head only in spec/spec.md, which #406 does not touch.
