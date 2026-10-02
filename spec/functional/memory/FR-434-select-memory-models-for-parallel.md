---
id: FR-434
title: "Select the memory model of each parallel from source and request"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-052
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-228
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-425
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-426
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-436
    type: references
  - target: ix://agent-ix/quire-specification/FR-437
    type: references
---

# FR-434: Select the memory model of each parallel from source and request

## Description

When a protocol subject holds a `parallel` control, the analyzer SHALL resolve
its memory model from the source default and the request's selection, bind
the resolved model into the obligation identity, and apply it only through the
memory-model contributions stated below.

## Inputs

A checked protocol clause whose `parallel` controls may carry a `memory`
clause; the request's `memory` member, if any; and the protocol subject
([FR-425](../protocol/FR-425-define-protocol-subjects-and-state-keys.md)).

## Outputs

The resolved memory model of every `parallel`; or a located refusal for a
conflicting default or an invalid selection.

## Behavior

### Source default

A `parallel` control declares its default memory model after its name, by the
[shared grammar](../../../proposals/quire-v1/shared-grammar.md)'s `memory-clause` production: `parallel Name memory M { … }
join all [ … ];`, with `M` one of `sc`, `tso` and `ra`. A `parallel` with no
`memory` clause has default `sc`, the interleaving meaning of
[FR-052](../protocol/FR-052-represent-bounded-control.md). `tso` and `ra` are
the **weak** models.

A nested `parallel` declares the same default as its outermost `parallel` or
none; another default refuses at check time
`invalid_model_binding`/`conflicting-binding`, naming both controls and their defaults. A
`parallel` that declares a weak default joins `all` with no `outstanding`
clause; another join policy refuses
`unsupported_construct`/`declaration-form` at the join, naming the policy and the weak default.

### Request selection

A request may carry a member `memory`, a list of
`{"parallel":"<scoped path>","model":"sc"|"tso"|"ra"}` entries, each naming
an **outermost** `parallel`, one nested in no other `parallel`, by its scoped
path. A selection replaces the model of that `parallel` and of every
`parallel` nested in it. An entry that names a nested `parallel`, a control
that is not a `parallel`, or a `parallel` already named by another entry,
refuses the request `invalid_runtime_input`/`invalid-value`, naming the
entry.

The **resolved model** of an outermost `parallel` is the request's selection
when present, else its source default. A request that selects a weak model for
a `parallel` whose join is not `all` without `outstanding` settles the item
`unsupported`, cause `weak-access-shape` with shape `join-policy`
([FR-439](FR-439-bound-replay-and-report-weak-memory-runs.md)).

### Scope and threads

The resolved model governs its `parallel`'s branches and every control nested
in them. Each branch of a weak `parallel` is one thread, whose program order
is its causal order; a nested `parallel`'s branches are threads forked from
the enclosing branch. A control outside every weak `parallel` belongs to the
enclosing thread, and its accesses apply in place, as under `sc`. Sequential
`parallel` controls in one protocol may resolve to different models: each
`join all` returns memory to one value per location.

### Identity

Two identities read the memory model differently:

- **Obligation identity.** The resolved model of every outermost `parallel`
  enters the obligation identity beside the subject, and the obligation
  identity reads the resolved model in place of the source `memory` clause.
  Two requests that resolve every `parallel` to the same models carry the same
  obligation identity whether the models came from source or from the
  request.
- **Package identity.** The source `memory` clause is source content, so it
  enters the checked package's identity and the qualified protocol-definition
  identity ([FR-052](../protocol/FR-052-represent-bounded-control.md)). A
  source `tso` default and a source `sc` default are two packages.
 The same protocol checked under `sc` and under `tso` is two
obligations, and a verdict on one never answers the other.

### Memory-model contributions

A memory model affects the protocol system through exactly these eight
contributions, and through no other:

| Contribution | Meaning | Under `sc` |
| --- | --- | --- |
| component | the memory state; the state key's `memory` member | empty |
| observation | the model state a thread's attempt evaluates its contracts over | the model state |
| write | where an attempt's post-state delta goes | applied in place |
| memory steps | internal steps, each with a typed canonical identity, an enabledness condition and an effect, and no operation | none |
| gate | an extra enabledness condition on a thread's `fork`, `join`, `send`, `receive` and `attempt` steps | true |
| split and merge | what a `fork` gives each branch thread and what a `join` gives the parent | nothing |
| atom values | the value of each location a temporal atom reads | the model state |
| fairness | constraints added to every infinite-trace clause, with origin `memory` | none |

`tso` and `ra` give these contributions in
[FR-436](FR-436-explore-total-store-order.md) and
[FR-437](FR-437-explore-release-acquire-memory.md). A memory step is a
position, uncounted for intervals, owned by no thread, and moves no rest
point.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-434-AC-1 | The store-buffering protocol with source default `sc` is checked under `sc` with no `memory` member, and under `tso` and `ra` with a `memory` entry naming `SB::Both`; the three requests carry three different obligation identities. | Test (TC-388, TC-390) |
| FR-434-AC-2 | A source default `tso` checked with no `memory` member and a source default `sc` checked with a `tso` selection carry the same obligation identity and the same verdict, while their checked packages carry different package identities. | Test (TC-388) |
| FR-434-AC-3 | A selection naming a nested `parallel`, a non-`parallel` control or one `parallel` twice refuses the request `invalid_runtime_input`/`invalid-value` naming the entry; a selection on an outermost `parallel` applies to every `parallel` nested in it. | Test (TC-388) |
| FR-434-AC-4 | A nested `parallel` declaring a default different from its outermost `parallel` refuses `invalid_model_binding`/`conflicting-binding` naming both; a weak default with an `any` join refuses `unsupported_construct`/`declaration-form` at the join; a `tso` selection on an `sc` `parallel` with an `any` join settles `unsupported`, cause `weak-access-shape`, shape `join-policy`. | Test (TC-388) |
| FR-434-AC-5 | Under `sc` the protocol system's states, steps and verdicts equal those of the same protocol with no `memory` clause, and the `memory` member of every state key is empty. | Test (TC-388) |

## Dependencies

- [FR-052](../protocol/FR-052-represent-bounded-control.md) owns the
  interleaving meaning that `sc` keeps.
- [FR-426](../protocol/FR-426-apply-protocol-steps.md) owns the steps the
  contributions act on.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) owns
  the request envelope that carries the `memory` member.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns the
  `parallel` and `memory-clause` productions.

## References

- Design: quire-spec-language ADR-025 MM-1 to MM-5, SC-1 and RU-2;
  ADR-027 SE-1, SE-2 and SE-4.
- Tickets: Linear STD-138; paired QSL-372.
