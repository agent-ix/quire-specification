---
id: SR-16285
title: "spec-review of clock places, clock collections and unit-typed positions (quire-specification PR 410)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@b6daac79; spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md; spec/functional/type-model/FR-154-admit-domain-package-model.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/functional/runtime/FR-181-simulate-finite-executions.md; spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md; spec/test-cases/TC-367-clock-declarations.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-415
    type: references
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# Spec review of clock places, clock collections and unit-typed positions (quire-specification PR 410)

## Summary

Ticket: STD-400. One spec-review pass over the PR's own delta (`git diff the head of #407...b6daac79`), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across open PR heads, and schema/vector consistency.

Examined, clean:

- Clock collections are now admitted in every kind, and the earlier `clock-in-collection` refusal is gone. Hand-derived: CK-16's filter over n = 1, 2, 3 with clocks 1, 2, 3 leaves clocks 2 and 3 at slots 0 and 1; the flatten gives a fresh slot 0 at 0 and moves the old clocks to slots 1 to 3; the set literal keeps the first occurrence's clock 0. FR-415-AC-13: 3 x 2 = 6 per object. Set<Clock> has at most one element because its data part is empty.
- A variant payload of Clock or a unit refuses `unsupported_construct` through FR-154's existing payload row, which refuses every resolved payload. That is not a narrowing by this PR.
- Gates at b6daac79: quire validate 86; cargo 136 + 24 passed. QSL conformance: see the report.
- QSL conformance at QSL ef43bab4 with QSPEC_DIR at b6daac79: checked_v2 47 passed, 0 failed.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | The setting is named but has no carrier. `proposals/backend-provider-v1/schema.json` `TemporalLimits` is closed (additionalProperties false) and has no `max_clocks` member, and the FR-368 limit list does not include it. So a caller cannot set it, and no limit-reached record can carry it. Fix: add `max_clocks` (PositiveCount) to TemporalLimits and to FR-368's limit table with its default, plus a vector. | spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md | wrong-requirement |
| FND-002 | medium | This rule has no case for returning a place. CK-11's `pick(a, b, left) { if left then a else b }` and AC-14's `maybe(o): Option<Clock> { o.x }` return a clock, which the bullet read literally refuses. Add "or returns it as the declaration's place result" to the rule. | spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md | wrong-requirement |

## Verdict

Not mergeable: FND-001 (high) block the PR; FND-002 (medium) should be fixed in the same round.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-003 | medium | QSpec now admits a magnitude bound on a unit-typed alias but never defines it. It does not say how the bound is typed or where it is checked (construction, a definedness obligation, or a runtime refusal code), and it defers the meaning to QSL FR-230. FR-142 "Quantity type reference" still says a bounded magnitude is not a complete-V1 type form, and #405's FR-146 now says a quantity has no magnitude bound, so a quotient carries no range obligation. Fix: state in FR-142 (or here) the bounded-quantity domain this constraint produces, its membership check and its refusal, and reconcile it with FR-142 and FR-146. | spec/functional/type-model/FR-154-admit-domain-package-model.md | wrong-requirement |

## Dispositions

Round 1, at da80beca. New since round 0: Reference<Clock> (ref/deref, place comparison, state key holds the place index), package clock `max` constraints as time invariants, and unit min/max constraints admitted at intake. AC-15/CK-23's four-place disjunction is correct for two calls with two clocks each. The remaining variant payloadType refusal is merged QSpec FR-154, so it is an owner question, not a finding.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | "max_clocks": {"description": "FR-415: the most live clocks a state of the explored graph may hold (setting model_check.max_clocks, default 1024).", "$ref": "#/$defs/PositiveCount"} in TemporalLimits; FR-368 and FR-331 list it; vectors limit-reached-max-clocks and temporal-limits-max-clocks-zero |
| FND-002 | fixed | reads a clock only as the clock operand of an atomic clock constraint, passes it to another clock-reading declaration, or returns it as the declaration's place result |
