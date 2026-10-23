---
id: SR-16286
title: "spec-review of quire.temporal.complete/v1 header-selectable layer (quire-specification PR 417)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@f72f4ceb; spec/functional/foundation/FR-001-select-native-profile.md; spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md; spec/functional/foundation/FR-453-preserve-parent-profile-refusals-under-layering.md; proposals/quire-v1/definitions/complete-value-lock.json; proposals/quire-v1/definitions/complete-temporal-lock.json; proposals/quire-v1/definitions/temporal-complete.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-001
    type: references
  - target: ix://agent-ix/quire-specification/FR-453
    type: references
  - target: ix://agent-ix/quire-specification/FR-415
    type: references
---

# Spec review of quire.temporal.complete/v1 header-selectable layer (quire-specification PR 417)

## Summary

Ticket: STD-421. One spec-review pass over the PR's own delta (`git diff the head of #410...f72f4ceb`), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across open PR heads, and schema/vector consistency.

Examined, clean:

- Layer row E6 checked: `requires` is `quire.model.complete/v1`, the selected closure in FR-001-AC-8 is the transitive requires set, and the lock follows the model-lock pattern with every role in `always`. The grammar already named `quire.temporal.complete/v1` as the timed-form facet, so a unit with no profile was already refused the timed forms. This is not a narrowing.
- Gates at f72f4ceb: quire validate 86; cargo 136 + 24 passed. QSL conformance: see the report (the layer-row test is expected red until QSL-1013).
- QSL conformance at QSL ef43bab4 with QSPEC_DIR at f72f4ceb: checked_v2 47 passed; qsl-semantics complete_value_lock 7 passed, 1 failed (conformance_layer_rows_match_qspec_complete_value_lock, the expected red until QSL-1013, so this PR merges with or after QSL-1013's implementation).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | The layer set is now six, but AC-5 still says "five layers" while naming complete-temporal units in its tail. Say six, or limit the tail to the five. | spec/functional/foundation/FR-001-select-native-profile.md | wrong-requirement |
| FND-002 | low | The admitted-forms list does not name what #410 adds: clock-reading predicates and functions, `forall` and `exists` clock reads over clock collections, reset by selection, and quantity-parameter bounds. List them or point to FR-415 "Clock places" and "Clock collections". | proposals/quire-v1/definitions/temporal-complete.md | wrong-requirement |

## Verdict

Mergeable: no defects found in this PR's own delta; merge in stack order.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-003 | low | #410's later commit ffc1a2cb adds `Reference<Clock>` with `ref` and `deref`, plus package clock `max` constraints, to FR-415. The temporal-complete admitted-forms list does not name them: there is no mention of `Reference<Clock>` or `deref`. Add them so the layer's admitted forms match FR-415. | proposals/quire-v1/definitions/temporal-complete.md | wrong-requirement |

## Dispositions

Round 1, at 270faa4c.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | \| FR-001-AC-5 \| For each of the six layers, ... |
| FND-002 | fixed | clock places and clock collections as FR-415 "Clock places" and "Clock collections" define them ... clock reads and resets over clock collections: element and slot selection ... forall/exists ... reset by selection |

Round 2.

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | temporal-complete.md admitted forms now list `Reference<Clock>`, `Option<Reference<Clock>>`, `ref`, `deref` and package clock `max` constraints (with `min` refused `time-invariant-shape`), tied to FR-415 "Clock places" and FR-154-AC-18 |
