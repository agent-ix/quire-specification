---
id: FR-439
title: "Bound, report and replay weak-memory runs"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-433
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-434
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-436
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-437
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
---

# FR-439: Bound, report and replay weak-memory runs

## Description

When an item over a protocol subject with a weak `parallel` is checked, the
analyzer SHALL bound the memory component by the caller's memory budgets,
state each budget used and whether it was reached in every result, settle the
verdicts stated below, and carry the memory component in every
counterexample for replay.

## Inputs

A protocol subject with a weak `parallel`; the request's `max_store_buffer`
and `max_messages` members, if set; and, for replay, a counterexample.

## Outputs

A model-check verdict with the memory budgets used and whether they were
reached; a counterexample carrying the memory component; or a replay outcome.

## Behavior

### Memory budgets

Two request members of the model-check method bound the memory component:
`max_store_buffer`, the entries per thread's buffer under `tso`, and
`max_messages`, the live messages per location after garbage collection under
`ra`. The caller sets each with no ceiling. A request that leaves one unset
receives the published default, **4**. They are run parameters, not part of
the model, the subject or the obligation identity.

A store that would exceed a budget is not expanded at that state; every other
enabled step is. The state is **bound-limited**. Fairness enabledness reads
the store as enabled, so a budget never makes an unfair lasso fair. A state
whose only enabled steps are bound-limited stores is a boundary state:
discovered, keyed, read by the monitor, never expanded, never
stutter-extended, never a deadlock.

### Verdicts

| Run | Verdict |
| --- | --- |
| a counterexample found | `refuted`: bounded behaviours are behaviours of the unbounded model |
| completes, no counterexample, no bound-limited state | `proved`, basis `exhaustive`, with each budget stated as not reached |
| completes, no counterexample, at least one bound-limited state | `inconclusive`, cause `memory-bound-reached`, naming the budget, its value, the count of bound-limited states and the request member that raises it |

A claim that evaluates undefined at a reachable state, bound-limited or not,
settles `refuted` with cause `undefined-evaluation` by the FR-363 rule, with
the memory component in its counterexample.

`constraint-reached` takes precedence over `memory-bound-reached`, which takes
precedence over `instance-bound-reached` and `bound-reached`. Every terminal
accounting record of an item over a subject with a weak `parallel` states, in
its method, the value of `max_store_buffer` (under `tso`) or `max_messages`
(under `ra`) used and whether the run reached it.

An item whose subject has an access the pre-check finds not to be one access
settles `unsupported`, cause `weak-access-shape`, carrying `site`,
`transition` and `shape`; so does an item in which a reader
([FR-435](FR-435-classify-memory-accesses-and-orderings.md)) reads a shared
location, with shape `multi-location` naming the reader as the site and the
shared locations it reads; so does an item with an instantiated attempt
whose class does not take its ordering (FR-435 Admitted orderings), with
shape `ordering-mismatch`. The `shape` is one of
`{"type":"multi-location","locations":[<location>, ...]}`,
`{"type":"join-policy"}` and
`{"type":"ordering-mismatch","access":<access>,"ordering":<ordering>}`, where
`access` is one of `load`, `store`, `rmw` and `local`, and `ordering` is one
of `relaxed`, `acquire`, `release`, `acq_rel` and `seq_cst`, or `null` when
the attempt carries none.
An item whose resolved model no candidate advertises settles `unsupported`,
`unsupported_projection`/`unsupported-requested-capability`. A candidate
advertises the memory models it decides, by the names `sc`, `tso` and `ra`,
in the `memory_models` member of its manifest descriptor
([FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md); a
descriptor with no `memory_models` advertises `sc` only); negotiation routes
an item over a subject with a weak `parallel` only to a candidate advertising
its resolved model and never weakens the model to fit a candidate.

**Load-buffering warning.** When an item's resolved model is `ra` and the
subject has a load-buffering shape (a `relaxed` or non-atomic load followed in
program order by a store to another shared location, with no `acquire` or
`seq_cst` load and no `acquire` fence between them), the provider SHALL add
one `memory.load-buffering-shape{load, store}` warning to the item's terminal
record for each distinct (load, store) shape, each member a transition
identity (FR-331). The warning is outside the result, the counterexample, its
replay identity and the obligation identity, and it changes no verdict. Under
`sc` and `tso` the provider adds none.

### Positions

Every memory step is a position, observed with its `memory` anchor, and is
uncounted for intervals. Buffers, messages, views, the SC event graph and the
race summary are read by replay and rendering; atoms read memory values.

### Counterexample wire

A counterexample over a subject with a weak `parallel` is the protocol
counterexample ([FR-433](../protocol/FR-433-replay-protocol-counterexamples.md))
with these additions:

- each step's transition identity may be an access with its `memory_choice`,
  a fence or a memory step such as `flush`;
- each step carries `memory`, its post-state's memory component in the
  state-key encoding: every buffer under `tso`; every message sequence, view,
  `S` and the SC event graph under `ra`, in the typed forms of
  [FR-437](FR-437-explore-release-acquire-memory.md); and the race summary when
  the subject has a non-atomic access;
- the counterexample carries `memory_models`, a list of
  `{"parallel":"<scoped path>","model":"sc"|"tso"|"ra"}` for every outermost
  `parallel`, and `memory_budgets`, the budget values the run used;
- its fairness set lists the flush or visibility constraints with origin
  `memory`.

The memory component is data for the reader; the post-state key already
binds it.

### Replay

Replay re-executes each step through the protocol system under the
counterexample's resolved models: an access by the FR-181 successor rule over
the thread observation, a memory step by its model's rule. It refuses
`invalid_runtime_input`/`invalid-value` when a step is not enabled or a named
message or slot does not exist. A lasso is checked against the authored,
scheduler and memory constraints; an unfair lasso refuses. Then the formula is
evaluated as for any protocol counterexample, or race replay runs for `kind:
race` ([FR-438](FR-438-report-data-races.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-439-AC-1 | A request with no memory budget members runs with 4 and states `max_store_buffer` 4, not reached (under `tso`), or `max_messages` 4, not reached (under `ra`), in the store-buffering results. | Test (TC-390, TC-393) |
| FR-439-AC-2 | A protocol whose loop keeps storing to one location under `tso` with `max_store_buffer` 2 settles `inconclusive`, cause `memory-bound-reached`, naming the budget, the value 2, the count of bound-limited states and `max_store_buffer`; with a counterexample within the budget it settles `refuted`. | Test (TC-393) |
| FR-439-AC-3 | Two requests that differ only in a memory budget carry the same obligation identity; a bound-limited state is never terminal or a deadlock. | Test (TC-393) |
| FR-439-AC-4 | When a state constraint and the memory budget both cut states, the cause is `constraint-reached`; when the memory budget and the instance budget both cut states, the cause is `memory-bound-reached`. | Test (TC-393) |
| FR-439-AC-5 | The `tso` store-buffering counterexample carries both buffers at every position, `memory_models` naming `SB::Both` as `tso` and `memory_budgets`, and replays; naming an absent message depth refuses `invalid_runtime_input`/`invalid-value`. | Test (TC-394) |
| FR-439-AC-6 | An item over a `tso` subject routed to a candidate that advertises only `sc` settles `unsupported`, `unsupported-requested-capability`; an item with a two-location access under a selected weak model settles `unsupported`, cause `weak-access-shape`, with its site, transition and shape. | Test (TC-389, TC-394) |
| FR-439-AC-7 | A `flush` position carries the `memory` anchor and does not advance interval distance. | Test (TC-394) |
| FR-439-AC-8 | A claim whose atom reads `lookup<T>(p, r) absent undefined` at an absent key at a reachable state, in a run that also reaches bound-limited states, settles `refuted`, cause `undefined-evaluation`, not `inconclusive` with `memory-bound-reached`, and its counterexample carries the memory component and replays. | Test (TC-393) |
| FR-439-AC-9 | An item over the load-buffering litmus (TC-391 LT-06) resolved to `ra` has the terminal record `warnings` hold two `memory.load-buffering-shape{load, store}` warnings, one per branch's distinct (load, store) shape, with the result, counterexample and obligation identity equal to the settlement without it; the same item resolved to `sc` or `tso` carries none; a candidate whose descriptor has no `memory_models` receives `SB` (TC-390) resolved to `sc` and not `SB` resolved to `tso` or `ra`. | Test (TC-393) |
| FR-439-AC-10 | The `ra` counterexample of TC-390 SB-05's protocol (every store and load `seq_cst`) to the claim `always holds(not (c1.r = 1 and c2.r = 1))` replays. | Test (TC-394) |
| FR-439-AC-11 | A `weak-access-shape` cause with shape `{"type":"ordering-mismatch","access":"rmw","ordering":null}`, one with `{"type":"ordering-mismatch","access":"load","ordering":"release"}` and one with `{"type":"ordering-mismatch","access":"local","ordering":"release"}` are admitted; a shape whose `access` is outside `load`, `store`, `rmw` and `local`, whose `ordering` member is missing, or that carries an extra member is refused. | Test (TC-389, TC-431) |

## Dependencies

- [FR-433](../protocol/FR-433-replay-protocol-counterexamples.md) owns the
  protocol counterexample and its replay.
- [FR-436](FR-436-explore-total-store-order.md) and
  [FR-437](FR-437-explore-release-acquire-memory.md) own the memory
  components bounded here.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) owns
  the manifest `memory_models` member that carries the memory models and the
  warning's wire form.

## References

- Design: quire-spec-language ADR-025 MB-1 to MB-5, MS-1 to MS-4, MV-1, MK-1,
  MK-2, MF-4, MF-5, MX-7 and RU-5.
- Tickets: Linear STD-138; paired QSL-372.
