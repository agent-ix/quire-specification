---
id: FR-398
title: "Align hyperproperty traces on a projection"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-395
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-191
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-396
    type: references
  - target: ix://agent-ix/quire-specification/FR-400
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-367
    type: references
---

# FR-398: Align hyperproperty traces on a projection

## Description

When a universal `hyper` clause over `behaviours` with a safety body carries
`align skip { O, … }`, the analyzer SHALL align its traces on their
projections onto the steps of the unlisted operations, and SHALL hold the
clause exactly when no matched joint projected prefix is a bad prefix of the
body.

## Inputs

The checked HP-6 clause ([FR-396](FR-396-classify-hyperproperty-forms.md)):
its `align skip` list, quantifiers, match and body, and one subject per
alias.

## Outputs

An HP-6 result settled by [FR-399](FR-399-settle-hyperproperty-results.md),
with a `projected` counterexample
([FR-400](../../objects/interfaces/FR-400-hyper-counterexample.md)) when
refuted.

## Admission

- Each `align skip` entry names an operation of a model alias the clause binds.
  When an entry names an operation of no bound alias, names an operation
  already listed, or the list is empty, the checker SHALL refuse it as
  `unsupported_construct`/`expression-form`, located at the entry (or at the
  list, when empty).
- When a variable of an `align skip` clause carries a fairness constraint,
  the checker SHALL refuse it as `unsupported_construct`/`expression-form`,
  located at its first constraint. Every finite path of a subject extends to a fair
  behaviour, so a fairness set would change no HP-6 verdict.
- A clause with `align skip` and an existential variable, or with a body
  outside the safety fragment, is HP-4 (FR-396).

## Projection

A step is **skipped** when its operation is in the list and **visible**
otherwise. The projection of a finite path is the sequence of its positions
that are position 0 or are entered by a visible step; projected position `j`
is the `j`-th of them, with position 0 the initial state. At a projected
position an indexed read reads the state the visible step entered, and the
match reads that visible step's label. Skipped steps reach neither the match
nor the body. Interval operators count projected positions.

## Meaning

An HP-6 clause holds exactly when, for every tuple of finite paths from
initial states, one per variable of its subject, whose projections have one
length `L + 1` and on which `μ_U` holds at every joint visible step `1` to
`L`, the joint projected prefix is not a bad prefix of the body: the safety
fragment's three-valued evaluation does not return false for it. Projected
positions correspond by index. A path that ends at a terminal state, or whose
continuations are all skipped steps, takes part in the prefixes up to its last
visible step.

A projected counterexample is a tuple of finite paths with equal visible
counts `L`, the traces of unequal total length, whose joint projected prefix
evaluates false; the canonical one is the first in FR-181 breadth-first order
over moves of either kind, where a move is either one component taking one
skipped step or every component taking one visible step with `μ_U` true on the
joint step. The maximum depth counts moves of either kind.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-398-AC-1 | On the busy/mix vault, lockstep noninterference matching only `step` arguments is refuted, and the same clause with `align skip { V::Vault::mix }` is proved. | Test (TC-344) |
| FR-398-AC-2 | A projected counterexample on a leaky variant carries equal visible counts and unequal total lengths, and its joint projected prefix evaluates false. | Test (TC-344) |
| FR-398-AC-3 | An entry naming an operation of no bound alias, a repeated entry, an empty list and a fairness set on a variable of an `align skip` clause each refuse as `unsupported_construct`/`expression-form` at the located construct. | Test (TC-337) |
| FR-398-AC-4 | Skipped steps change neither the match nor any indexed read: two paths with the same visible steps and different skipped steps give the same projected prefix. | Test (TC-344) |

## Dependencies

- [FR-395](FR-395-analyze-hyperproperties-over-behaviours.md) owns binding,
  the match split and indexed reads.
- [FR-191](FR-191-analyze-relational-hyperproperties.md) requires an explicit
  total alignment; the projection is that alignment for HP-6.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns
  the `align-skip` production.
- [FR-363](../temporal/FR-363-settle-every-behaviour-temporal-verdicts.md) and [FR-367](../temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md)
  own the safety fragment and its three-valued prefix evaluation.

## References

- STD-136 (this requirement); QSL-370 and its ADR-023 (design).
