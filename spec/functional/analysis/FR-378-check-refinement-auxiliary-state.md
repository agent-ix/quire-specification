---
id: FR-378
title: "Check refinement with history fields and hidden abstract fields"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/FR-379
    type: references
---

# FR-378: Check refinement with history fields and hidden abstract fields

## Description

When a refinement declaration carries history rows or leaves an abstract
field unmapped, the analyzer SHALL track history values and the set of
consistent abstract states along each concrete behaviour without adding,
removing or blocking any concrete behaviour.

## Inputs

An admitted refinement declaration ([FR-375](FR-375-declare-model-refinement.md))
with one or more history rows, one or more hidden abstract fields, or both,
and the two model subjects.

## Outputs

The history values and candidate sets [FR-377](FR-377-decide-model-refinement.md)
reads at each concrete position, or a verdict this requirement settles.

## Behavior

**History fields.** A history row adds one field to every object of its
concrete type, for this refinement only. Every object starts with the row's
initial literal, and an object a step creates starts with it too. When a step
of an operation with an update is taken, the update is evaluated and written
to the receiver's history field only; a step of an operation with no update
keeps every history value. History values are a function of the concrete
prefix and gate no step, so the concrete subject extended with its history
fields has exactly the concrete subject's behaviours, each with one history
annotation. History fields never enter the concrete package, its clauses or
its successor relation.

**History failures.** An update that evaluates undefined at a reachable state
settles the refinement item `refuted` with cause `undefined-evaluation` and an
`Undefined` refinement counterexample
([FR-379](FR-379-report-and-replay-refinement-counterexamples.md)). An update
that is refused or incomplete, or whose value leaves the history field's
declared type, at a reachable state settles the item `inconclusive` with cause
`mapping-undetermined` (FR-379). Either way the step stays a step of the
concrete subject.

**Hidden fields.** An abstract field with no row is hidden. Along each
concrete behaviour the analyzer tracks the **candidate set**: the abstract
states whose visible fields equal the mapped ones and that are consistent
with the behaviour so far.

- Initially, the abstract initial states whose visible part equals
  `map(c0, h0)`.
- After a `stutter` step, the same set, provided the visible part is
  unchanged.
- After an abstract-operation or `any` step, every abstract successor that
  FR-377's step rule admits from a member, whose visible part equals
  `map(s', h')`.

A step after which the set is empty fails with `NoAbstractMatch`. The set
keeps every abstract choice alive until a later concrete step rules it out,
so a safety refinement that would need a prophecy variable is decided
exactly. The set is bounded by the hidden fields' finite domains and by the
caller's budgets; reaching a budget settles the item `incomplete`, FR-241 `resource-incomplete`, cause `limit-reached{limit, value, setting}`.

**Divergence with hidden fields.** When the abstract side is a protocol
specification whose profile makes divergence observable, the analyzer
decides [FR-177](../protocol/FR-177-check-protocol-refinement.md)'s
divergence-freedom check on the candidate-set product. Along a loop of
internal steps the candidate set is unchanged, because each internal step is
a `stutter` step. Such a loop refutes as a divergence when every member of
the candidate set at the loop enables a visible abstract step, and is not a
divergence when some member enables none, because that member is an
abstract state the loop may stay at. The check reads only the loop and its
candidate set, so it is decided with hidden fields present and does not
depend on `F_A`; only the liveness half under `F_A` is unsupported.

**Liveness with hidden fields.** A declaration with a hidden field and a
non-empty `F_A` has its safety half decided as above, and its liveness half
settles `unsupported` with cause
`unsupported_projection`/`unsupported-requested-capability`, naming liveness
through hidden abstract fields as the unsupported capability. With `F_A`
empty there is no liveness half. A refinement whose hidden field is
determined by the concrete past is checked with a history row that maps it.

**Product key.** The analyzer's product state key adds the history values,
encoded as FR-181 typed values in history-row order, and the candidate set's
abstract state keys, sorted ascending by their JCS bytes.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-378-AC-1 | A history field starts at its initial literal on every object, including an object a step creates; a step with an update writes only the receiver's history field; a step with none keeps every history value. | Test (TC-314) |
| FR-378-AC-2 | Adding history rows to a declaration leaves the set of concrete behaviours, the concrete package and every other verdict about the concrete model unchanged. | Test (TC-314) |
| FR-378-AC-3 | A history update that evaluates undefined at a reachable state settles the refinement item `refuted`, cause `undefined-evaluation`, with an `Undefined` counterexample that replays; one whose value leaves the declared type settles `inconclusive`, cause `mapping-undetermined`; in both the step is not removed. | Test (TC-315) |
| FR-378-AC-4 | For a hidden field, the candidate set starts at the abstract initial states matching the visible mapped state, is unchanged by a `stutter` step with an unchanged visible part, is replaced by the admitted successors matching the new visible part after an abstract-operation or `any` step, and a step that empties it fails `NoAbstractMatch`. | Test (TC-314) |
| FR-378-AC-5 | The coin refinement, whose abstract `toss` fixes a hidden `side` that the concrete `reveal` later shows, holds as a safety refinement, while every functional mapping of `side` fails. | Test (TC-314) |
| FR-378-AC-6 | A declaration with a hidden field and a non-empty `F_A` decides its safety half and settles its liveness half `unsupported` with `unsupported_projection`/`unsupported-requested-capability` naming liveness through hidden abstract fields; the same declaration with `F_A` empty settles only its safety half. | Test (TC-314) |
| FR-378-AC-7 | The history-backed counter refinement, mapping abstract `total` from a history field counting commits, holds; removing that field row makes `total` hidden, and with an `ensure` row the liveness half settles `unsupported`. | Test (TC-314) |
| FR-378-AC-8 | TC-318's divergence vector with a hidden abstract field `mode: Boolean`, free at the abstract initial states, where the specification's `ack` requires `not mode`: with `F_A` empty the `spin` loop at `ready` is not a divergence, because the candidate set holds the `mode = true` state, which enables no visible step; with `ack` enabled for either `mode` the loop refutes as a divergence with a lasso that replays; both verdicts are the same under `fair weak whole ack`, and neither settles `unsupported`. | Test (TC-314) |

## Dependencies

- [FR-375](FR-375-declare-model-refinement.md) admits history rows and
  defines hidden fields.
- [FR-377](FR-377-decide-model-refinement.md) reads history values and
  candidate sets in `map` and the step rules.
- [FR-379](FR-379-report-and-replay-refinement-counterexamples.md) owns the
  `mapping-undetermined` cause, the `Undefined` refinement failure and the
  replay of candidate sets.

## References

- Design: quire-spec-language ADR-020 (AX-1 to AX-5, QS-3, RU-4).
- Tickets: Linear STD-133; paired QSL-367. Liveness through hidden fields is
  explored later in Linear QSL-380.
