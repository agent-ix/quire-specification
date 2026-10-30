---
id: FR-366
title: "Declare intended terminal states and check deadlock freedom"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-361
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-417
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
  - target: ix://agent-ix/quire-specification/FR-368
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# FR-366: Declare intended terminal states and check deadlock freedom

## Description

When a request holds temporal items over a model subject whose model does not
declare `terminal any`, the request writer SHALL add one derived
deadlock-freedom item that reports every reachable terminal state the model
does not mark intended as a deadlock, except that over a timed model subject a
bounded-delay terminal state is a time-lock and is reported by the
time-lock-freedom item ([FR-417](FR-417-settle-time-lock-freedom.md)).

## Inputs

The checked package with its state model and its `terminal` declaration, if
any; the model subject ([FR-361](FR-361-read-model-behaviours-as-temporal-traces.md));
and the request's temporal items.

## Outputs

One deadlock-freedom item per distinct model subject, settled with an
[FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md) verdict; or no
item for a subject whose model declares `terminal any`; or a located refusal
for an inadmissible `terminal` declaration.

## Behavior

### Intended terminal states

A package declares the intended terminal states of a model with at most one
`terminal` declaration per model alias:

```ebnf
terminal-decl = 'terminal', ident, 'using', ident, 'on', ident,
                ( 'when', block | 'any' ), ';' ;
```

`terminal` and `any` are reserved words of the temporal facet. The `on` name is
a declared model alias. `using` selects a declared value profile alias, as for
every native declaration. A second `terminal` declaration for the same model
alias refuses `ambiguous_declaration`/`ambiguous-name`, naming both.

- `terminal T using v on M when { P }`: `P` is a Boolean state predicate,
  read through the same clause evaluator as state clauses over a state's
  observation. A terminal state that satisfies `P` is intended. `P` classifies
  terminal states only: a state that satisfies `P` and has an enabled
  transition is not terminal.
- `self` in `P`: when the model's populations cover exactly one object type
  `T` and exactly one population covers `T`, `self` is typed
  `Reference<M::T>` and `P` is evaluated once for each object of that
  population at the terminal state. `P` is satisfied when it is true for
  every object, so a terminal state with no object of `T` satisfies it; not
  satisfied when it is false for any object, even when it is undefined for
  another; and otherwise, when it is undefined for some object, undefined.
  When two or more populations cover `T`, `self` refuses
  `ambiguous_declaration`/`ambiguous-name` at `self`; when the populations
  cover no object type or several, `self` refuses
  `missing_declaration`/`missing-name` at `self`. A `P` that does not read
  `self` is evaluated once at the terminal state.
- `terminal T using v on M any`: every terminal state is intended. This is the
  per-model opt-out.
- A model with no `terminal` declaration intends no terminal state.

The declaration is part of the checked package, so it is part of the subject
and of every obligation identity that reads the subject.

### Deadlock

A **deadlock** is a reachable state of the subject that is terminal (no
transition identity is enabled at it) and not intended. The definition reads
the state graph alone; fairness plays no part in it.

### The derived deadlock-freedom item

For each distinct model subject among a request's temporal items, the request
writer SHALL add one deadlock-freedom item, unless the subject's model declares
`terminal any`. The item:

- is the reachable-state invariant `always holds(not deadlocked)`, where
  `deadlocked` is the deadlock predicate above, derived and not authored;
- has an empty fairness set;
- has the obligation identity (subject, item kind `deadlock-freedom`)
  ([FR-368](FR-368-negotiate-temporal-model-check-items.md));
- is recorded, negotiated and settled as a reachable-state invariant, with the
  verdicts of FR-363;
- has no node of its own: it is keyed by its anchor, the `model`/`model_root`
  node `{package: <selected identity>, path: ["model", "model_root"],
  ordinal: 0}` of the model selection the subject reads, together with its
  `item_kind` `deadlock-freedom`, as
  [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) "Derived request items" states.

A reachable deadlock settles the item `refuted`, basis
`decisive-counterexample`, with an [FR-364](FR-364-replay-temporal-counterexamples.md)
counterexample of `kind: deadlock`: a finite prefix from an initial state to
the deadlocked state. An expansion that meets an undecided contract
conjunction settles `inconclusive`, cause `undecided-successor`.

A `terminal when` predicate that evaluates undefined at a reachable state with
no enabled transition settles the item `refuted`, cause
`undefined-evaluation`, by the one model-side undefined rule of
[FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md), with the FR-364
undefined-evaluation counterexample ending at that state. The state is read
neither as intended nor as a deadlock.

Over a timed model subject a state is terminal as
[FR-181](../runtime/FR-181-simulate-finite-executions.md) "Timed subjects"
states: no discrete step is enabled at it or after any admissible delay. A
terminal timed state in which time can pass without bound is a terminal state
of this requirement and is classified intended or a deadlock as above. A
bounded-delay terminal state (the admissible delay is bounded by a time
invariant or urgency) is a time-lock in the sense of
[FR-417](FR-417-settle-time-lock-freedom.md) "Time-locks", reported by the
time-lock-freedom item and not here.

### Claims read every terminal state the same way

A deadlock changes no authored claim's verdict. Every terminal state, intended
or not, reads by FR-361: closed under the event-position false-extension
profile and stutter-extended under infinite-trace. Under infinite-trace, a
behaviour that deadlocks before `p` refutes `eventually holds(p)` with the
terminal stutter lasso, and the deadlock-freedom item reports the same state as
a deadlock; in a model that declares `terminal any`, only the claim reports it.
A `terminal` declaration changes only the deadlock-freedom item.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-366-AC-1 | A model with no `terminal` declaration and a reachable successor-free state receives a deadlock-freedom item that settles `refuted` with a `kind: deadlock` finite prefix ending at that state, and the prefix replays. | Test (TC-297) |
| FR-366-AC-2 | The same model with `terminal when { P }`, where `P` holds at that state, settles the item `proved`; with `P` false at a second reachable terminal state, the item settles `refuted` at that second state. | Test (TC-297) |
| FR-366-AC-3 | The same model with `terminal any` adds no deadlock-freedom item to the request, and its authored claims' verdicts are unchanged. | Test (TC-297) |
| FR-366-AC-4 | A request with two temporal items over one subject receives exactly one deadlock-freedom item, and a request over two distinct subjects receives one per subject; the item's obligation identity is the subject with item kind `deadlock-freedom`. | Test (TC-297, TC-302) |
| FR-366-AC-5 | Under infinite-trace, an authored `eventually holds(p)` claim over a model that deadlocks before `p` is refuted with the terminal stutter lasso, with and without a `terminal` declaration covering the deadlocked state. | Test (TC-297) |
| FR-366-AC-6 | A second `terminal` declaration for one model alias refuses `ambiguous_declaration`/`ambiguous-name` naming both declarations. | Test (TC-297) |
| FR-366-AC-7 | A state that satisfies `P` and enables a transition is not terminal and is never reported as a deadlock or as intended. | Test (TC-297) |
| FR-366-AC-8 | A model whose `terminal when` predicate evaluates undefined at a reachable state with no enabled transition settles the deadlock-freedom item `refuted`, cause `undefined-evaluation`, with an undefined-evaluation counterexample ending at that state, never `proved` and never a `deadlock` counterexample. | Test (TC-305) |
| FR-366-AC-9 | Over the Job subject with universe `{j, k}`, `terminal Finished using v on M when { self.status = done }` classifies a terminal state where both objects are `done` as intended, one where `j` is `done` and `k` is `stuck` as deadlocked, and, with universe `{}`, the initial terminal state as intended; a model whose populations cover two object types refuses `self` with `missing_declaration`/`missing-name` at `self`; a model with two populations that both cover `Job` refuses `self` with `ambiguous_declaration`/`ambiguous-name` at `self`, naming both populations. | Test (TC-297) |
| FR-366-AC-10 | A bounded-delay terminal state is not reported by the deadlock-freedom item. | Test (TC-370) |
| FR-366-AC-11 | A model subject's deadlock-freedom item adds no node to the checked package. Its claim node is the `model`/`model_root` node of the subject's model selection, `{package: "acme/orders", path: ["model", "model_root"], ordinal: 0}` in `derived-items-request`, at occurrence `generated` 0, and the item is told apart from the time-lock-freedom item on that node by `item_kind` alone. | Test (TC-431 BP-51) |

## Dependencies

- [FR-361](FR-361-read-model-behaviours-as-temporal-traces.md) owns terminal
  states and their reading under each profile.
- [FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md) owns the
  verdicts the item receives.
- [FR-364](FR-364-replay-temporal-counterexamples.md) owns the counterexample
  `kind` member and the deadlock replay.

## References

- quire-spec-language ADR-018 §10 (DL-1 to DL-7), RU-3 and QS-12; Linear
  STD-131 (paired with QSL-366).
