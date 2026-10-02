---
id: FR-366
title: "Declare intended terminal states and check deadlock freedom"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-361
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-363
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
  - target: ix://agent-ix/quire-specification/FR-368
    type: references
---

# FR-366: Declare intended terminal states and check deadlock freedom

## Description

When a request holds temporal items over a model subject whose model does not
declare `terminal any`, the request writer SHALL add one derived
deadlock-freedom item that reports every reachable terminal state the model
does not mark intended as a deadlock.

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
  verdicts of FR-363.

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
