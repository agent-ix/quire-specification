---
id: FR-362
title: "Apply weak and strong fairness constraints"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-161
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-361
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
  - target: ix://agent-ix/quire-specification/FR-368
    type: references
---

# FR-362: Apply weak and strong fairness constraints

## Description

When an infinite-trace temporal clause carries fairness constraints, the
analyzer SHALL admit only the behaviours that satisfy every constraint, with
each constraint's kind and granularity read as stated below, before it
evaluates the clause's formula.

## Inputs

A checked infinite-trace temporal clause with its fairness constraints, and
either a model subject ([FR-361](FR-361-read-model-behaviours-as-temporal-traces.md))
or a supplied trace.

## Outputs

The clause's resolved fairness set, and the subset of behaviours it admits; or
a located refusal for an inadmissible constraint; or `unsupported` with cause
`unsupported_projection`/`missing-fairness-premise` for a fairness constraint
over a supplied trace.

## Behavior

### Syntax and resolution

A temporal clause body carries zero or more fairness constraints before its
captures:

```ebnf
temporal-clause = 'temporal', ident, 'using', ident,
                  'over', '(', parameter, ')', 'clock', string,
                  activation, '{', { fairness }, { capture },
                  temporal-expr, '}' ;
fairness        = 'fair', [ 'weak' | 'strong' ], [ 'whole' | 'each' ],
                  qualified-name, ';' ;
```

`fair`, `weak`, `strong` and `whole` are reserved words of the temporal facet;
`each` is already reserved. The `qualified-name` resolves to one effective
operation of the subject's model, else the frontend refuses it with
`missing_declaration`/`missing-name` at the name.

A constraint resolves to the triple (kind, operation, granularity):

- The kind is `weak` or `strong`. An unwritten kind is `weak`. Strong fairness
  is written with `strong`.
- The granularity is `whole` or `each`. An unwritten granularity is `whole`,
  for both kinds. `each` is written with `each`.

So `fair Op` resolves to (`weak`, `Op`, `whole`), and `fair strong Op` to
(`strong`, `Op`, `whole`). The clause's **fairness set** is the set of its
resolved triples, ordered by operation qualified name, then kind, then
granularity, each compared bytewise as UTF-8. A clause may carry both kinds on
one operation; both stay in the set. The empty set is a valid premise and
admits every behaviour.

A fairness constraint is admitted on an infinite-trace clause only. On a clause
under a bounded profile it refuses `unsupported_construct`/`declaration-form`
at the constraint, naming the selected profile.

### Enabled and taken

- A transition identity is **enabled** at a state when the operation's
  effective precondition holds for that receiver and argument vector and at
  least one post-state within the operation's frame satisfies its
  postcondition.
- A `whole` constraint on `Op` is one constraint over every transition
  identity of `Op`; it is enabled at a state when any of them is.
- An `each` constraint on `Op` is one constraint per transition identity of
  `Op`, over every receiver in the universe and every argument vector of the
  finite parameter domains; each is enabled when its identity is.
- A constraint is **taken** at a step when the step's transition identity
  belongs to it. The terminal stutter step takes no constraint.

Enabledness is a property of the model state at a position. A terminal state
enables no constraint.

### Weak and strong fairness

A behaviour satisfies a constraint when:

| Kind | On an infinite behaviour | On a lasso (prefix plus loop) |
| --- | --- | --- |
| `weak` | there is no position from which the constraint is enabled at every position and never taken | some step of the loop takes the constraint, or some state of the loop does not enable it |
| `strong` | if the constraint is enabled at infinitely many positions, it is taken at infinitely many steps | if some state of the loop enables the constraint, some step of the loop takes it |

A lasso whose loop is the terminal stutter step satisfies every constraint of
either kind.

Fairness filters behaviours before evaluation
([FR-161](FR-161-analyze-infinite-trace-properties.md)). A constraint changes
which behaviours are admitted; it never changes the truth of a formula on an
admitted behaviour.

### Ordering of premises

- **Strong implies weak.** Every behaviour that satisfies (`strong`, `Op`, `g`)
  satisfies (`weak`, `Op`, `g`). A claim proved under the weak constraint holds
  under the strong one, and a claim refuted under the strong constraint is
  refuted under the weak one.
- **`each` is the stronger granularity.** Every behaviour that satisfies
  (`k`, `Op`, `each`) satisfies (`k`, `Op`, `whole`), for either kind `k`.
- **Machine closure.** Fairness of operations of the model is machine-closed:
  every finite prefix of a behaviour of a finite subject extends to a
  behaviour that satisfies any finite set of weak and strong constraints.
  Fairness therefore never makes a proof vacuous, and it never changes the
  verdict of a safety form or of a bounded-profile form
  ([FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md)), whose truth
  depends on finite prefixes only.

### Missing fairness premise

Enabledness comes from a model subject only. A supplied trace, replayed or
monitored with no model subject behind it, carries states and steps and no
enabledness. A clause with a non-empty fairness set evaluated over a supplied
trace SHALL settle `unsupported` ([FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)),
cause `unsupported_projection`/`missing-fairness-premise`, naming the first
constraint of its fairness set. A clause with an empty fairness set over a
supplied trace evaluates as FR-161 states.

### Identity

The resolved fairness set enters the clause's obligation identity and every
counterexample ([FR-364](FR-364-replay-temporal-counterexamples.md)). Two
spellings with the same resolved set have the same identity: `fair Op` and
`fair weak whole Op` are one obligation. Two different resolved sets are two
obligations: a claim under (`strong`, `Op`, `each`) and the same claim under
(`weak`, `Op`, `each`) never share a verdict.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-362-AC-1 | `fair Op`, `fair weak Op`, `fair whole Op` and `fair weak whole Op` resolve to (`weak`, `Op`, `whole`) and produce one obligation identity; `fair strong Op` resolves to (`strong`, `Op`, `whole`) and `fair each Op` to (`weak`, `Op`, `each`), each a different identity. | Test (TC-291) |
| FR-362-AC-2 | A fairness constraint on a clause under a bounded profile refuses `unsupported_construct` at the constraint naming the profile; a constraint naming no effective operation of the model refuses `missing_declaration`/`missing-name`. | Test (TC-291) |
| FR-362-AC-3 | On conformance lassos, weak fairness accepts a loop that takes the constraint or has a state that disables it and rejects a loop that enables it at every state without taking it; strong fairness accepts a loop that takes the constraint or never enables it and rejects a loop that enables it at some state without taking it. | Test (TC-291) |
| FR-362-AC-4 | A lasso whose loop is enabled for a constraint at one state, disabled at another and never takes it is weakly fair and strongly unfair; a lasso whose loop is the terminal stutter step is fair under every constraint. | Test (TC-291) |
| FR-362-AC-5 | A `whole` constraint is discharged by a step of any receiver or argument vector of its operation, while the `each` constraint on the same operation is violated by a loop that starves one enabled identity. | Test (TC-291, TC-293) |
| FR-362-AC-6 | A claim proved under a weak constraint is also proved under the strong constraint with the same operation and granularity, and a claim refuted under a strong constraint is also refuted under the weak one. | Test (TC-299) |
| FR-362-AC-7 | A safety claim and a bounded-profile claim over a finite subject receive the same verdict with and without any set of weak and strong constraints, and no fairness set makes a finite subject's set of admitted behaviours empty. | Test (TC-291) |
| FR-362-AC-8 | A clause with one weak constraint and a clause with one strong constraint, each evaluated over a supplied trace, settle `unsupported` with cause `missing-fairness-premise` naming that constraint; the same clause with an empty fairness set evaluates over the trace. | Test (TC-292) |

## Dependencies

- [FR-161](FR-161-analyze-infinite-trace-properties.md) states that fairness
  filters traces before evaluation.
- [FR-361](FR-361-read-model-behaviours-as-temporal-traces.md) supplies the
  model subject, its transition identities and the terminal stutter step.
- [FR-368](FR-368-negotiate-temporal-model-check-items.md) routes a fairness
  set only to a candidate that decides each of its kinds.

## References

- quire-spec-language ADR-018 FA-1 to FA-6 and QS-4; ADR-019 SY-1 to SY-5,
  SF-1 to SF-7, SV-2, SV-5 and QS-1 to QS-3; Linear STD-131 and STD-132
  (paired with QSL-366 and QSL-365).
