---
id: FR-397
title: "Check step relations over model operations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-191
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-353
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-399
    type: references
  - target: ix://agent-ix/quire-specification/FR-401
    type: references
---

# FR-397: Check step relations over model operations

## Description

When a `relation` clause binds an execution variable to an operation of a
model alias, the analyzer SHALL check it as a model claim over the reachable
transitions of that operation and, when every bound operation has an
implementation binding, SHALL also request a separate code claim over every
pre-state that satisfies the declared invariants.

## Inputs

The checked `relation` clause with its execution bindings in order, each
naming an operation; one model subject per distinct alias; the operations'
effective preconditions, frames and postconditions; each model's declared
invariants; and each operation's abstraction-relation binding where present.

## Outputs

A model-claim result and, where requested, a separate code-claim result.

## Binding

An execution binding `x: V::T::op` ranges over the transitions of operation
`op` of alias `V`'s model. The body is a Boolean under the value profile that
reads, per execution, its arguments (`x.i`), its receiver's pre-state
(`pre(x.self.f)`), its receiver's post-state (`x.self.f`) and its result
(`x.result`). Every comparison is between named executions (FR-191). An
execution binding to any other qualified name keeps FR-191's finite reading.

## Model claim

The model claim holds exactly when the body is true for every tuple
`(t_1, …, t_n)` in which `t_i` is a transition of variable `i`'s operation
that leaves a reachable state of its subject: an edge of the reachable graph
with its pre-state, arguments, result and post-state. Every reachable
transition lies on some behaviour, so this is the hyperproperty "for every
tuple of behaviours and every tuple of positions at which each takes its
named operation, the body holds", with positions chosen independently per
trace.

- Tuples are enumerated in the product of the subjects' FR-181 canonical edge
  orders, in binding order, and the first tuple on which the body is false is
  the canonical counterexample.
- The tuples a run evaluates count against `max_relation_tuples`
  ([FR-399](FR-399-settle-hyperproperty-results.md)).
- The counterexample is a `step-tuple` hyper counterexample
  ([FR-400](../../objects/interfaces/FR-400-hyper-counterexample.md)): per
  execution, a finite path from an initial state to the execution's
  pre-state followed by the execution's transition.

## Code claim

- **Request.** When every execution variable's operation is bound to an
  implementation by an [FR-353](../ir-proof/FR-353-declare-abstraction-relation.md)
  abstraction relation, the clause also yields one item of FR-290 kind
  `operation-contract` for the code claim, beside the model claim. The checked
  package carries what the code claim reads: the relation node with its
  execution variables in order, each naming its operation; the body lowered to
  terms over each execution's pre-state, arguments, result and post-state; each
  execution model's declared invariants; each operation's effective
  precondition, frame and postcondition; and each operation's abstraction
  binding.
- **Obligation.** For all pre-states `s_1 … s_n` of the declared types, each
  satisfying every declared invariant of its model, and all argument vectors
  in the operations' parameter domains with each operation's effective
  precondition true at its `s_i`: running each operation's bound
  implementation once from its own `s_i`, the calls sharing no state, gives
  results and post-states on which the body holds. Every outcome of a
  nondeterministic implementation is covered. Pre-states range over the
  declared types, not over reachable states.
- **Separate obligations.** The code claim takes the invariants as its premise
  and says nothing about whether they hold. The model claim and the code claim
  have different obligation identities and different evidence, and neither
  discharges the other. A code-claim run whose premise no state satisfies
  makes zero checks and settles `inconclusive` with the vacuity cause of
  [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md),
  never `proved`.
- **Counterexample.** A code-claim counterexample gives, per execution, a
  pre-state, an argument vector, a result and a post-state, and replays by
  [FR-401](FR-401-replay-hyper-counterexamples.md) with no reachability
  prefix.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-397-AC-1 | A determinism relation over two executions of one operation, comparing post-states under equal pre-states and arguments, is refuted on a model with a nondeterministic post-state with a step-tuple counterexample that replays, and proved on a deterministic model. | Test (TC-343) |
| FR-397-AC-2 | The model claim evaluates only transitions that leave reachable states; a violating tuple whose pre-state is unreachable does not refute it. | Test (TC-343) |
| FR-397-AC-3 | The canonical counterexample is the first false tuple in the product of canonical edge orders, in binding order. | Test (TC-343) |
| FR-397-AC-4 | A relation whose operations all have implementation bindings yields a model-claim item of kind `temporal-satisfaction` and a separate code-claim item of kind `operation-contract`, with different obligation identities; a relation with any unbound operation yields only the model claim. | Test (TC-343) |
| FR-397-AC-5 | A code-claim counterexample's pre-states are admitted as snapshots, checked against the declared invariants, and the body evaluates false over the tuple; a code-claim run with an unsatisfiable premise settles `inconclusive` with the vacuity cause. | Test (TC-343) |
| FR-397-AC-6 | An execution binding to a qualified name that is not a model operation keeps FR-191's finite reading. | Test (TC-337) |

## Dependencies

- [FR-191](FR-191-analyze-relational-hyperproperties.md) owns comparison across
  named executions.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the successor
  relation and canonical edge order.
- [FR-353](../ir-proof/FR-353-declare-abstraction-relation.md) owns the
  implementation binding.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) assigns the
  two claims' kinds.

## References

- STD-136 (this requirement); QSL-370 and its ADR-023 (design).
