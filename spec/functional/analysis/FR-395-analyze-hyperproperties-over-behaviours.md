---
id: FR-395
title: "Analyze hyperproperties over every behaviour of a model"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-191
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-161
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-396
    type: references
  - target: ix://agent-ix/quire-specification/FR-361
    type: references
  - target: ix://agent-ix/quire-specification/FR-362
    type: references
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
  - target: ix://agent-ix/quire-specification/FR-367
    type: references
  - target: ix://agent-ix/quire-specification/FR-398
    type: references
  - target: ix://agent-ix/quire-specification/FR-399
    type: references
---

# FR-395: Analyze hyperproperties over every behaviour of a model

## Description

When a `hyper` clause over `behaviours` is checked, the analyzer SHALL bind
each trace variable to the fair behaviours of the model subject its alias
names, SHALL compare the variables' behaviours only at explicitly aligned
positions and only on matched steps, and SHALL evaluate the body over each
tuple of behaviours by the rules below.

## Inputs

The checked clause: its object parameters, quantifier prefix with each
variable's alias and fairness set, `match` block, optional `align skip` list
and body; one model subject per distinct alias (initial states, universes and
proof bounds).

## Outputs

One result per clause instance, classified by
[FR-396](FR-396-classify-hyperproperty-forms.md) and settled by
[FR-399](FR-399-settle-hyperproperty-results.md).

## Binding

- **Domain.** The `behaviours` domain makes every trace variable range over
  behaviours of model subjects. The FR-191 domain `t in Q bounded N` keeps its
  finite meaning.
- **Aliases.** Under `behaviours` every quantifier names a model alias with
  `of`. Two quantifiers that name the same alias range independently over the
  same subject, which is the self-composition. Quantifiers that name
  different aliases range over different subjects. The request supplies one
  subject per distinct alias.
- **Object parameters.** `behaviours (v: V::T)` binds an object of a
  population of alias `V`'s model. The clause has one instance per key of
  that population's universe. Within an instance, `v` denotes the object with
  that key in every trace of that alias: objects are correlated across traces
  by universe key, never by local name. A clause over two aliases names one
  parameter per alias it reads, and its instances range over the product of
  their universes. A clause one of whose parameters ranges over an empty
  universe, or over a type that no object of the universes conforms to, has
  zero instances: the item is a `NoInstance` item of
  [FR-361](../temporal/FR-361-read-model-behaviours-as-temporal-traces.md)
  Instancing and settles by [FR-399](FR-399-settle-hyperproperty-results.md),
  never `proved`, since no instance was ever evaluated. A parameter whose
  population has no universe is `requires-bound` first.
- **Indexed reads.** Every model member read in the body carries exactly one
  trace variable, written `e @ a`; one atom may read several variables.
- **Step labels.** Inside `match`, `a.step.op` is the operation of the step
  into the current position, or `stutter` for a terminal stutter step;
  `a.step.receiver` is that step's receiver key, of type
  `Option<Reference<T>>`, read the same way as an argument: it is none at a
  terminal-stutter step and at a step of an operation with no receiver;
  `a.step.O.p` is the argument
  `p` of operation `O`, of type `Option<T>` for `p: T`, present exactly when
  the step is an `O` step. A step label is the step's FR-181 transition
  identity, so "same inputs" is the operation and arguments the environment
  chose; the post-state the model chose is not an input. The match reads no
  state.
- **Fairness per variable.** Each quantifier carries its own fairness set, the
  `fairness` constraints written after its alias, over its own model's
  operations, with the kinds and granularity of
  [FR-362](../temporal/FR-362-apply-fairness-constraints.md): `weak` and
  `whole` when unwritten. A quantifier with no constraint has the empty set.
- **Profile.** A clause over `behaviours` selects
  `quire.temporal.infinite-trace/v1`, and its body is an infinite-trace
  formula over indexed atoms, interval operators nested under unbounded ones
  admitted as [FR-367](../temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md)
  admits them.

## Semantics

- **Domain of a variable.** A variable `a of M` ranges over the behaviours of
  `M`'s subject that satisfy `a`'s fairness set. Fairness is read on the
  variable's own behaviour: enabledness and taking come from its own states
  and steps.
- **Lockstep alignment.** Unless the clause carries `align skip`, a tuple of
  behaviours is read position by position: joint position `j` is position `j`
  of every component, and joint step `j` is the tuple of the components'
  steps into position `j`. Every behaviour is infinite, because a terminal
  state is stutter-extended, so every joint position exists for every
  component. Lockstep is the explicit total alignment FR-191 requires: it pairs
  positions by their index in each model's step sequence. `align skip` aligns
  on projections instead
  ([FR-398](FR-398-align-hyperproperty-traces-on-a-projection.md)).
- **Match split.** For a prefix with universal variables `u` before
  existential variables `e`, the match is a conjunction, split into `μ_U`, the
  conjuncts that read universal variables only, and `μ_E`, the conjuncts that
  read an existential variable. The clause holds exactly when, for every tuple
  of behaviours of `u` on which `μ_U` holds at every joint step, there is a
  tuple of behaviours of `e` on which `μ_E` holds at every joint step and the
  body holds on the joint behaviour. With no existential variable this is:
  every tuple matched by `μ_U` satisfies the body. The match constrains joint
  steps `j >= 1`; position 0 has no step, and a premise on initial states is
  written in the body. Quantifier order is preserved.
- **Tuple evaluation.** The body's truth on a joint behaviour is the FR-161
  infinite-trace value, with each indexed read reading its component's state
  at the joint position. On a tuple of lassos the joint behaviour is
  ultimately periodic: its stem is the longest component stem and its period
  the least common multiple of the components' loop lengths, each component
  indexed prefix-then-loop, so past operators see the same history at a loop
  position and at its unrolled copies. Interval operators count joint
  positions.
- **Terminal stutter.** A component at a terminal state takes its terminal
  stutter step ([FR-361](../temporal/FR-361-read-model-behaviours-as-temporal-traces.md))
  at every later joint step, with step label `stutter`. The stutter step is
  outside every fairness constraint. A match that requires equal operations
  matches a stuttering component only with another stuttering one.
- **Vacuity.** A clause with a non-empty `μ_U` and no tuple of fair universal
  behaviours on which `μ_U` holds at every joint step is vacuous and is never
  `proved` (FR-399). With `μ_U` empty the universal domain is never empty,
  because a finite subject with an initial state has a fair behaviour.
- **Deadlocks.** A hyper item reports no deadlock. A request that holds a
  hyper item holds the one derived deadlock-freedom item
  ([FR-366](../temporal/FR-366-check-deadlock-freedom.md)) of each distinct
  subject the item names.

## Admission

- When a quantifier under `behaviours` names no alias, or a quantifier under
  a finite trace domain names one, the checker SHALL refuse the quantifier as
  `unsupported_construct`/`expression-form`, located at the quantifier.
- When an indexed read names a trace variable bound to a model other than the
  one declaring the member it reads, the checker SHALL refuse it as
  `ill_typed`, located at the read.
- When a model member read in a hyper body carries no trace variable, or an
  object parameter names a population of an alias the clause does not bind,
  the checker SHALL refuse it as `ill_typed`, located at the read or
  parameter.
- When a match conjunct reads state, the checker SHALL refuse it as
  `unsupported_construct`/`expression-form`, located at the read.
- When a clause over `behaviours` is checked under a temporal profile other
  than `quire.temporal.infinite-trace/v1`, the negotiation SHALL settle it
  `unsupported`, `unsupported_projection`/`unsupported-requested-capability`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-395-AC-1 | Under `behaviours`, a quantifier without `of`, and under a finite trace domain a quantifier with `of`, refuse at the quantifier; an indexed read through a variable of another model and an unindexed member read refuse `ill_typed`; a match that reads state refuses. | Test (TC-337) |
| FR-395-AC-2 | Two quantifiers over one alias range independently over one subject, quantifiers over two aliases over two subjects, and an object parameter yields one instance per universe key with the object correlated by key in every trace. | Test (TC-337, TC-338) |
| FR-395-AC-3 | Lockstep noninterference with matched inputs is refuted on the leaky vault and proved on the secure vault; without `match` it is refuted on both. | Test (TC-338) |
| FR-395-AC-4 | `μ_U` restricts the universal tuples and `μ_E` the existential tuples, so a `∀∃` clause is refuted on the leaky vault and proved on the secure vault. | Test (TC-339) |
| FR-395-AC-5 | A component at a terminal state stutters with step label `stutter`, and a match of equal operations pairs it only with another stuttering component; `a.step.receiver` is none at its terminal-stutter step, so `a.step.receiver = b.step.receiver` holds between two stuttering components. | Test (TC-340) |
| FR-395-AC-6 | A clause whose non-empty `μ_U` no fair universal tuple satisfies is vacuous and is not `proved`. | Test (TC-340) |
| FR-395-AC-7 | Each variable's fairness set filters only its own behaviours: a liveness body that holds only under one variable's fairness set is proved with that set and refuted without it. | Test (TC-341) |
| FR-395-AC-8 | A body with an interval operator nested under `always` is evaluated over joint positions of a tuple of lassos, with the stem the longest component stem and the period the least common multiple of the loop lengths. | Test (TC-341) |
| FR-395-AC-9 | A clause over `behaviours` under a profile other than infinite-trace settles `unsupported` with `unsupported_projection`/`unsupported-requested-capability`. | Test (TC-337) |
| FR-395-AC-10 | A request with a hyper item holds one deadlock-freedom item per distinct subject the item names, and the hyper item itself reports no deadlock. | Test (TC-340) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns the
  `hyper-clause`, `behaviours-domain`, `quantifier`, `fairness`,
  `match-block`, `step-label` and `indexed-read` productions.
- [FR-191](FR-191-analyze-relational-hyperproperties.md) owns quantifier order,
  explicit alignment and correlation by explicit identity.
- [FR-161](../temporal/FR-161-analyze-infinite-trace-properties.md) owns the
  infinite-trace value of the body.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns transition
  identities, which step labels read.
- [FR-396](FR-396-classify-hyperproperty-forms.md) classifies the clause.
- [FR-361](../temporal/FR-361-read-model-behaviours-as-temporal-traces.md),
  [FR-362](../temporal/FR-362-apply-fairness-constraints.md),
  [FR-366](../temporal/FR-366-check-deadlock-freedom.md) and
  [FR-367](../temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md)
  own the model subject's behaviours, fairness, deadlock-freedom item and
  mixed interval formulas.

## References

- STD-136 (this requirement); QSL-370 and its ADR-023 (design).
