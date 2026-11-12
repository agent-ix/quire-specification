---
id: FR-380
title: "Transfer temporal claims through a refinement"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/FR-379
    type: references
  - target: ix://agent-ix/quire-specification/FR-161
    type: references
  - target: ix://agent-ix/quire-specification/FR-092
    type: references
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-348
    type: references
---

# FR-380: Transfer temporal claims through a refinement

## Description

When a claim over a concrete model subject is requested by transfer from a
proved abstract claim and a proved refinement, the verification planner SHALL
settle it from those two premises, without exploring the concrete subject,
exactly when the clause is stuttering-invariant, the subjects match and the
refinement's abstract fairness implies the claim's fairness.

## Inputs

A transfer item naming: an infinite-trace clause `P` with fairness set
`F_P`; the concrete subject; the obligation identity of a refinement item
([FR-377](FR-377-decide-model-refinement.md)) over that concrete subject;
and the obligation identity of a claim item for `P` over that refinement's
abstract subject.

## Outputs

An [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
result for the transfer item with its basis, cause and premises.

## Behavior

**Kind and reading.** A transfer item is classified as an item of kind
`temporal-satisfaction` ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md))
by the verification planner only: it is a planner-local item, evaluated by the
planner and never sent to a provider, so it has no FR-331 `item_kind` and is
not a request item ([FR-368](../temporal/FR-368-negotiate-temporal-model-check-items.md)
"Item kinds").
Its claim is `P` read through the refinement's `map`, under the refinement's
concrete fairness set `F_C`: `P`'s atoms read the mapped abstract state, and
`P`'s `over` parameter ranges over the images of the concrete subject's
objects. Its obligation identity binds `P`, `F_P`, the concrete subject, the
refinement node and the two premise identities. No backend is negotiated for
it, and no exploration of the concrete subject runs.

**Stuttering-invariant fragment.** A clause transfers only when it is built
from `always`, `eventually`, `until`, `release`, `historically`, `once`,
`since` and `triggered` over state-predicate atoms, with no interval operator
and no previous operator. On that fragment inserting a step that repeats a
state changes no truth value, because until, since and their duals are
non-strict ([FR-161](../temporal/FR-161-analyze-infinite-trace-properties.md)).
In a clause with an interval operator nested under an unbounded one, the
interval subformula, every operator whose scope contains it, and so the whole
clause, do not transfer. Previous, and strong previous
([FR-092](../temporal/FR-092-evaluate-bounded-past.md)), read the position
before, which a concrete step mapped to `stutter` changes. A clause outside
the fragment is checked on the concrete subject directly as its own item.

**Subject match.** The refinement premise's abstract subject equals the
abstract claim premise's subject: the same checked package, the same initial
states, and universes equal to the concrete subject's universes of the
populations they are mapped from. The refinement premise's concrete subject
equals the transfer item's subject.

**Fairness implication.** Every constraint of `F_P`, a triple of kind,
operation and granularity, is implied by the refinement's `F_A`. A
constraint of `F_A` implies a constraint of `F_P` on the same operation only
when the kinds are compatible: a `strong` constraint in `F_A` implies a
`weak` or a `strong` one in `F_P`, and a `weak` constraint in `F_A` implies
only a `weak` one in `F_P`, because weak fairness does not imply strong
fairness ([FR-362](../temporal/FR-362-apply-fairness-constraints.md)). The
granularity is equal, or the `F_P` constraint is the `whole` form of an
`each` constraint in `F_A`, and the `whole` form keeps the same kind rule. A
constraint of `F_P` that no entry of `F_A` implies settles
`transfer-not-preserving`.

**Safety and liveness premises.** A safety clause transfers from an abstract
premise over the abstract subject. A liveness clause transfers only from an
abstract premise over the abstract subject's **stutter closure**: the abstract
subject with a stutter step added at every state, belonging to no fairness
constraint, checked under `F_P`. A premise over maximal abstract behaviours
does not cover the stutter tails the refinement admits.

**Settlement.**

| Premises and checks | FR-360 label | FR-243 basis | Cause |
| --- | --- | --- | --- |
| Both premises `proved`; fragment, subject match, fairness implication and premise form all hold | `proved` | `decisive-witness` | none; the witness is the pair of premise records |
| A clause outside the fragment | `inconclusive` | `unsettled` | `transfer-not-preserving`, naming the interval or previous operator |
| Fairness implication fails | `inconclusive` | `unsettled` | `transfer-not-preserving`, naming the `F_P` constraint `F_A` does not imply |
| A liveness clause with an abstract premise over maximal behaviours | `inconclusive` | `unsettled` | `transfer-not-preserving`, naming the missing stutter closure |
| A premise not `proved` | `inconclusive` | `unsettled` | `premise-unproved`, naming the premise and its result |
| Subject match fails, or a premise identity names no settled item | — | — | refused `invalid_runtime_input`/`invalid-value`, naming the mismatching member |

A transfer item is never `refuted`: an abstract counterexample is not a
concrete behaviour. A refutation of the concrete claim comes from checking it
on the concrete subject directly. The transfer record carries the two premise
identities, the refinement node identity and the fragment, fairness and
premise-form facts it checked.

**Vertical composition.** A `proved` transfer item over subject `C` is a
claim over `C`. It serves as the abstract premise of a further transfer to a
subject that refines `C` when `P` is a safety clause. A transferred liveness
result covers `C`'s fair behaviours, not `C`'s stutter closure, so a liveness
clause is transferred to a subject that refines `C` from a premise over `C`'s
stutter closure. Each link is its own transfer item with its own premises, settled by this
requirement; [FR-348](FR-348-admit-compositional-verification.md) owns
composition over component interface contracts.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-380-AC-1 | A transfer item with a proved abstract claim, a proved refinement, a clause in the fragment, matching subjects and implied fairness settles `proved`, basis `decisive-witness`, with no exploration of the concrete subject and both premise identities in its record. | Test (TC-317) |
| FR-380-AC-2 | A clause with an interval operator nested under an unbounded one, and a clause using previous, each settle `inconclusive`, cause `transfer-not-preserving`, naming the operator; checked directly on the concrete subject, the previous clause's verdict differs from its abstract verdict on the vector model. | Test (TC-317) |
| FR-380-AC-3 | An `F_P` constraint that no `F_A` constraint implies settles `inconclusive`, cause `transfer-not-preserving`, naming the constraint; a `whole` constraint in `F_P` with the matching `each` constraint of the same or a stronger kind in `F_A` transfers, a `weak` constraint in `F_P` transfers from a `strong` one in `F_A`, and a `strong` constraint in `F_P` does not transfer from a `weak` one in `F_A`, whether `each` or `whole`. | Test (TC-317) |
| FR-380-AC-4 | A liveness clause proved over maximal abstract behaviours but refuted over the abstract stutter closure settles `inconclusive`, cause `transfer-not-preserving`, naming the missing stutter closure; the same clause with a stutter-closure premise transfers. | Test (TC-317) |
| FR-380-AC-5 | A premise that is refuted, inconclusive, unsupported or failed settles the transfer item `inconclusive`, cause `premise-unproved`, naming the premise; no transfer item settles `refuted`. | Test (TC-317) |
| FR-380-AC-6 | A refinement premise whose abstract subject differs from the abstract claim's subject in package, initial states or universes is refused `invalid_runtime_input`/`invalid-value`, naming the mismatching member. | Test (TC-317) |
| FR-380-AC-7 | A proved transferred safety claim over `C` serves as the abstract premise for a transfer to a subject refining `C`; a transferred liveness claim over `C` does not, and a stutter-closure premise over `C` does. | Test (TC-317) |
| FR-380-AC-8 | `always eventually holds(c.value = 2)`, proved over the abstract counter's stutter closure under `fair weak each Spec::Counter::inc`, transfers through the compare-and-set refinement to the concrete counter under `F_C`. | Test (TC-317) |

## Dependencies

- [FR-377](FR-377-decide-model-refinement.md) and
  [FR-379](FR-379-report-and-replay-refinement-counterexamples.md) settle the
  refinement premise.
- [FR-161](../temporal/FR-161-analyze-infinite-trace-properties.md) gives the
  infinite-trace operators their non-strict meaning, and
  [FR-092](../temporal/FR-092-evaluate-bounded-past.md) defines previous and
  strong previous.
- [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md),
  [FR-243](../../objects/protocol/FR-243-settlement-basis.md) and
  [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) supply the
  result labels, bases and kind.
- [FR-348](FR-348-admit-compositional-verification.md) owns composition over
  component interface contracts.
- The model-subject temporal requirement (claims over a model subject and
  their fairness sets) is a normative prerequisite.

## References

- Design: quire-spec-language ADR-020 (CO-3 to CO-5, MC-3, QS-7, RU-3).
- Tickets: Linear STD-133; paired QSL-367. The model-subject temporal
  requirement is Linear STD-131.
