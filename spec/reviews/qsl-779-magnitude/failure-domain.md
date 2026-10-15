---
id: SR-9130
title: "failure-domain review of the supplied numeric magnitude bound B in FR-323 (PR 255)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@2a04a4721; spec/objects/interfaces/FR-323-native-runtime-envelope.md; spec/test-cases/TC-278-replay-envelope-members.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: references
  - target: ix://agent-ix/quire-specification/TC-278
    type: references
---

# Failure-domain review of the supplied numeric magnitude bound (quire-specification PR 255)

## Summary

Ticket: QSL-779. Failure-domain pass over the same diff as SR-9129, at tip
2a04a4721. It asks whether B, as specified, leaves any input-unbounded
numeric helper behind a nominal event charge. The oracle is QSV FR-109
Behavior 5 and AC-7, read on QSV `origin/main`. The pass also checked
where QSV raises `NumericDomain` today (`src/declaration/membership.rs` on
`origin/main`).

Examined, clean: the value-vs-domain path (value, lower, upper; six
Rational operands). The initiating `value-visit` is charged first, then
each operand is sized before the comparison, and the first operand over B
stops on that event. There is no unbounded continuation, truncation or
widening, and nothing further is charged to A, H or the Meter (rules 5-7,
RE-24/25).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Rules 5 and 7 and AC-15 bound only the value-vs-domain comparison inside the initiating `value-visit`. QSV FR-109's "Nested retained type" row says type-link comparisons include input-sized retained numeric data, to be bounded separately with "no outer one-event substitution". QSV raises the same `NumericDomain` stop on a `type-comparison` event for Int/Int and Rational/Rational retained types, for example TC-906's `o: Option<Int[0, 9]>` and `s: Sequence<Int[0, 9]>`. The operands there are the two types' endpoints (4 Integer, 8 Rational), not value/lower/upper. With B stated, nothing checks those operands and no stop applies, so a numeric helper input-bounded by nothing runs behind one nominal `type-comparison` unit, which FR-109 Behavior 5 and AC-7 forbid. B then does not discharge the operand-bound premise for nested Int/Rational types. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:201-215, :224-233, :416 | missing-requirement |

## Verdict

CHANGES REQUESTED: one blocking finding. Extend rule 5's coverage, rule 7's
event placement, AC-15 and a TC-278 vector to the type-comparison numeric
path: the expected and retained interval endpoints, checked on the charged
`type-comparison` event before the comparison, with their own `operand`
names. Alternatively, state where that path is stopped.
