---
id: FR-428
title: "Check replicated roles and per-trigger protocol activation"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-050
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-055
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-171
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-425
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-426
    type: depends_on
---

# FR-428: Check replicated roles and per-trigger protocol activation

## Description

When a protocol subject's clause declares replicated roles or activation `on
each`, the analyzer SHALL spawn, act through and retire role instances, start
one protocol instance per trigger occurrence within the caller's
`max_live_instances` budget, and state the budget used in every result, as
stated below.

## Inputs

A protocol subject ([FR-425](FR-425-define-protocol-subjects-and-state-keys.md));
each replicated role's population universe; and the request's
`max_live_instances` member, if set.

## Outputs

The `activate`, `spawn` and `retire` steps of each state; the `roles` member
of each state key; and, for an `on each` subject, the budget used and whether
it was reached, in every verdict.

## Behavior

### Activation on each

A clause `activation on each (x: T) when {g}` starts one protocol instance per
`activate` step. The trigger is a semantic event the environment supplies; the
analyzer does not derive it from the model's own steps. Each instance has its
own root thread, captures, role bindings, replicated role instances,
registrations, binders and queues; every instance shares the one model state.
An instance's ordinal is the smallest natural number no live instance holds.

An instance is removed in the step that finishes it (its `finish` step with
every one of its threads off `running`), with its binders, registrations,
role instances and queues. A trigger record is not kept.

### Replicated roles

A role `role r each T from pop max n lifetime L` has one **role instance** per
object of population `pop` it spawns. `pop` names a population declared by
the domain package that declares `T`, written as its declared name; a `pop`
that names no population of that package refuses
`missing_declaration`/`missing-name` at the name, and a `pop` that names a
population whose element type is not `T` refuses `ill_typed`/`type-mismatch`
at the name, each retaining the role and the population name. The population's
universe is the subject's, set by the caller. `max n` is the protocol's authored limit on live
instances of `r` at once, part of the checked package.

- An event node `by r` is taken by any live instance of `r`, one step per
  instance, and its binder records the acting instance.
- A `send` node is owned by the `from` role of its channel and a `receive`
  node by the `to` role of its channel. When that role is replicated, the
  node is taken by any live instance of the role, one step per instance, with
  the acting instance in the `actor` member, as an event node `by r` is; a
  thread at such a node while the role has no live instance and no enabled
  `spawn` waits with the `no-role-instance` cause (FR-429).
- A role instance's identity in the state key is the object's reference. The
  FR-171 spawn ordinal is trace data, recomputed by replay, so equal instance
  sets key equal in whatever order they were spawned.
- The `roles` entry of a replicated role is a map from each object reference
  to `"live"` or `"retired"`. A retired instance stays in the map, so it is
  never spawned again in that protocol instance.
- The instance binder of `r` is the role name `r`: inside `until(p)`, `r`
  names one instance, typed `Reference<T>`. The binder is readable only in
  that role's own `until(p)`; a role name used as a value anywhere else (a
  choice guard, a node constraint, another role's `until`) SHALL refuse
  `missing_declaration`/`missing-name` at the name, because it names no value
  there.
- Lifetime `workflow`: the instance retires when its protocol instance
  finishes. Lifetime `scope`: it retires in the step after which no live
  thread can reach any node `by r` inside the *scope region*, the closest
  control node that is an ancestor of every node `by r` (a control is never
  one of those nodes, so the region contains them strictly); when no control
  other than the root `run` holds every such node, the region is the root. Lifetime `until(p)`: the `retire` step. Retirement under
  `workflow` and `scope` is folded into the step that ends the lifetime.

### Steps

| Kind | Owner | Enabled when | Effect |
| --- | --- | --- | --- |
| `activate` | the environment of an `on each` subject | some record of `T` satisfies the activation guard, and fewer than `max_live_instances` instances are live | starts one instance with its captures evaluated over the record and its root thread settled from the start of `run` |
| `spawn` | the protocol instance | `o` is an object of `r`'s population not yet in `r`'s map; fewer than `max n` instances of `r` are live; the instance is not finished | `o` becomes a live instance of `r` |
| `retire` | the protocol instance | `r`'s lifetime is `until(p)` and `p` holds for instance `o` over the binders in scope and the model observation | `o` becomes retired: it takes no further step |

### The instance budget

`max_live_instances` is a request member of the model-check method, set by
the caller with no ceiling. A request that leaves it unset receives the
published default, **3**.

- An `activate` step from a state with `max_live_instances` live instances is
  not expanded. The state is **instance-limited**. Fairness enabledness reads
  that `activate` as enabled, and an instance-limited state is never terminal
  and never a deadlock.
- A run that completes with no counterexample and at least one
  instance-limited state settles `inconclusive`, cause
  `instance-bound-reached`, naming the bound value and the count of
  instance-limited states.
- A counterexample found is real and settles `refuted`.
- A run that completes with no counterexample and no instance-limited state
  settles `proved`, basis `exhaustive`.
- Among under-approximation causes, `constraint-reached` takes precedence over
  `memory-bound-reached` ([FR-439](../memory/FR-439-bound-replay-and-report-weak-memory-runs.md)),
  which takes precedence over `instance-bound-reached`, which takes precedence
  over `bound-reached`.
- Every verdict over an `on each` subject, `proved` included, states in its
  terminal accounting record the `max_live_instances` value used and whether
  any state was instance-limited. An `inconclusive` result names the request
  member `max_live_instances` that raises the bound.

The budget is a run parameter. It is not part of the subject or of its
obligation identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-428-AC-1 | An `on each` protocol whose triggers are always available, checked with `max_live_instances` 1 and with 2, explores at most that many live instances, settles `inconclusive` with cause `instance-bound-reached` naming the bound and the count of instance-limited states, and names `max_live_instances` as the member that raises it. | Test (TC-381) |
| FR-428-AC-2 | A request with no `max_live_instances` member runs with 3 and states 3 in its terminal record; a counterexample found under the bound settles `refuted`, and a run that reaches no instance-limited state settles `proved` and states the bound as not reached. | Test (TC-381) |
| FR-428-AC-3 | Two requests that differ only in `max_live_instances` carry the same obligation identity. | Test (TC-381) |
| FR-428-AC-4 | A replicated role with each lifetime `workflow`, `scope` and `until(p)` retires at the stated step, and a retired object is never spawned again in that protocol instance; never more than `max n` instances are live. | Test (TC-382) |
| FR-428-AC-5 | Two behaviours that spawn the same objects in different orders reach byte-identical `roles` entries, while their traces carry distinct FR-171 spawn ordinals. | Test (TC-382) |
| FR-428-AC-6 | An event node `by r` with two live instances gives two steps, each naming its acting instance in the `actor` member. | Test (TC-382) |
| FR-428-AC-7 | An `on each` activation guard, or an `until(p)` retire predicate, that evaluates undefined at a reachable state settles every item over the subject `refuted` with cause `undefined-evaluation` naming that predicate's locus, never a disabled `activate` or `retire` step. | Test (TC-381, TC-382) |
| FR-428-AC-8 | In `role w each M::Worker from workers max 2 lifetime until(w.done)`, `w` names one instance of type `Reference<M::Worker>` inside the `until` expression, and the same role written with a brace-delimited `until({w.done})` refuses `invalid_syntax`/`unexpected-token` at the `{`, while an `until` expression naming a binder other than `w` (`z`) refuses `missing_declaration`/`missing-name` at `z`. | Test (TC-382, TC-947) |
| FR-428-AC-9 | With `Work` by `r` as the only node inside a `repeat`, a `scope` instance is not retired while the `repeat` can iterate again and retires in the step that leaves it; with `Work` nodes inside two sibling `sequence`s of one `parallel`, the region is the `parallel`, and the instance retires only after no live thread can reach either `Work`; with every `Work` directly in the root `run`, the region is the root. | Test (TC-382) |
| FR-428-AC-10 | A role name used as a value outside its own `until(p)` refuses `missing_declaration`/`missing-name` at the name: `w` read in a `choice` guard of the protocol, in a node constraint, and in another replicated role's `until`, each refuse and none is read as the acting instance. | Test (TC-382) |
| FR-428-AC-11 | `role w each M::Worker from workers max 1 lifetime workflow` checks when `workers` is a population of `M::Worker` in the domain package; the same role with `from absent` refuses `missing_declaration`/`missing-name` at `absent`, and with `from jobs` where `jobs` is a population of `M::Job` refuses `ill_typed`/`type-mismatch` at `jobs`, each retaining the role and the population name. | Test (TC-382) |
| FR-428-AC-12 | A channel `from r to c` where `r` is a replicated role with two live instances: a `send` on it gives two steps, each naming its acting instance in the `actor` member; a `receive` on a channel `from c to r` does the same; with `r`'s population universe empty, a thread at the `send` deadlocks with the `no-role-instance` cause naming `r`. | Test (TC-382) |

## Dependencies

- [FR-050](FR-050-bind-protocol-instances.md),
  [FR-055](FR-055-activate-protocol-obligations.md) and
  [FR-171](FR-171-evaluate-dynamic-control.md) own instance binding,
  activation and dynamic-role meaning.
- [FR-425](FR-425-define-protocol-subjects-and-state-keys.md) and
  [FR-426](FR-426-apply-protocol-steps.md) own the state and steps.

## References

- Design: quire-spec-language ADR-027 AE-1 to AE-4, RR-1 to RR-4, ST-13 to
  ST-15 and QS-11.
- Tickets: Linear STD-140; paired QSL-396.
