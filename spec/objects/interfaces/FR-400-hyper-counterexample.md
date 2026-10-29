---
id: FR-400
title: "Hyper counterexample wire"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-399
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-401
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-400: Hyper counterexample wire

## Description

When a hyper or step-relation model claim is refuted, the producer SHALL
encode its counterexample as one `HyperCounterexample` value below, serialized
with RFC 8785 JCS, and the reader SHALL refuse a value that breaks a rule
below before any replay runs.

## Properties

- **HyperStep** — the model trace step of an
  [FR-364](../../functional/temporal/FR-364-replay-temporal-counterexamples.md)
  `TemporalCounterexample`:
  `{"type":"step","transition":<transition>,"post_state":"<64 lowercase hex>"}`,
  with `transition` the FR-181 typed canonical form of the transition
  identity, or `{"type":"terminal-stutter"}` for the terminal stutter step,
  and `post_state` the post-state's state key. A
  terminal-stutter step's post-state is its pre-state.
  In a `projected` counterexample every step also carries
  `"mark":"skipped"` or `"mark":"visible"`.
- **HyperTrace** — `{"type":"hyper-trace","variable":"<ident>","alias":"<ident>","initial":"<decimal>","steps":[<HyperStep>, ...]}`:
  the trace variable (or execution variable), its subject's model alias, the
  index of its first state among that subject's initial states in canonical
  order, and its steps.
- **Instance** — `[{"parameter":"<ident>","key":<encoded>}, ...]`, the clause
  instance's object parameters in declaration order with their universe keys
  in FR-181 typed canonical form; empty for a clause with no object parameter.
- **HyperCounterexample** —
  `{"type":"hyper-counterexample","kind":<kind>,"instance":<Instance>,"traces":[<HyperTrace>, ...],"trace_position":"<decimal>"}`
  plus the members its kind requires:

| `kind` | Form | `traces` | Extra members |
| --- | --- | --- | --- |
| `lockstep` | HP-2 | one per universal variable, in quantifier order, all with the same number of steps | `loop_entry`: the joint position at which every component's loop re-enters |
| `witness-exhausted` | HP-3 | one per universal variable, in quantifier order, all with the same number of steps | `loop_entry` as above; `position`: the first joint position at which the witness set is empty |
| `projected` | HP-6 | one per universal variable, in quantifier order, every trace with the same number `L` of visible steps, total lengths free | none; `trace_position` is projected position `L` |
| `step-tuple` | HP-1 | one per execution variable, in binding order: a finite path from an initial state to the execution's pre-state, then the execution's transition as its last step | none |
| `undefined` | any | one per trace or execution variable of the clause, universal and existential, in quantifier or binding order: a finite path from an initial state to the state the undefined evaluation reads | `undefined`: the `UndefinedEvaluation` below; `trace_position` is the joint position at which the body, a match, or a guard, the `terminal when` predicate or a state constraint of a component's subject evaluates undefined |

- **UndefinedEvaluation** — the
  [FR-364](../../functional/temporal/FR-364-replay-temporal-counterexamples.md)
  undefined-evaluation envelope,
  `{"type":"undefined-evaluation","where":{...},"cause":<reason>}`. A hyper
  evaluation point is one joint position over several components, so its
  `where` carries `position`, the joint position; `state_keys`, FR-364's
  plural key, each component's state key there
  in quantifier or binding order; `locus`, the source locus of the expression
  that has no value; and `variables`, the trace or execution variables with
  their aliases, `[{"variable":"<ident>","alias":"<ident>"}, ...]`, in the
  same order, so for the body, a match or any other locus read over every
  component, `variables` and `state_keys` hold as many entries as the
  counterexample has traces, however many universal variables the clause has.
  For a guard (a contract conjunction of a transition), the `terminal when`
  predicate or a state constraint of a component's subject, which reads one
  component's state, `variables` and `state_keys` each name that component
  alone, and for a guard `where` also carries FR-364's `transition`, the
  FR-181 transition identity whose guard it is. It is the `where` and `cause` of the item's
  [FR-363](../../functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md)
  `undefined-evaluation`, so the counterexample replays on its own.

`trace_position` names the failing joint position. Every decimal is `0` or
`[1-9][0-9]*`.

**Paths of an `undefined` counterexample.** The paths need not have equal step
counts. When a `μ_U` has no value on a joint step out of a state of an HP-3
clause, the universal paths include that step and end at the state it enters,
the existential paths end at the pre-state the step leaves, one step short, and
`trace_position` is the position the step enters. In an HP-6 clause the paths
are unmarked and hold every step the path took, skipped steps included, so
their total lengths may differ; `trace_position` and the `where` position are
the projected position, the number of visible steps of a path, not the number
of its steps. A `state_keys` entry is the key of the last state of its
component's path, so for an existential path that ends one step short its
entry is the key of its pre-state, the state at `trace_position` minus 1.

## Envelope

`HyperCounterexample` travels in the witness envelope of the replay boundary
with its own model-trace-tuple replay-source arm, distinct from the
backend-transcript and input arms of
[FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md)
and from the model-graph arm of
[FR-393](FR-393-state-graph-evidence.md). It is replayed by
[FR-401](../../functional/analysis/FR-401-replay-hyper-counterexamples.md).
It carries no canonical state and no permutation: a counterexample found
under a reduction is concretised before it is encoded.

## Validation

The reader SHALL refuse each of the following as
`invalid_runtime_input`/`invalid-value`, before replay, naming the offending
member:

- an unknown `kind`, a missing member, a member its kind does not carry, a
  duplicate key or a non-canonical encoding;
- traces that do not match the clause's variables in number, order, variable
  name or alias;
- `lockstep` or `witness-exhausted` traces of unequal step counts, or a
  `loop_entry` or `position` beyond them;
- a `projected` trace with a step that has no `mark`, or traces with unequal
  visible counts; a `mark` on a step of any other kind;
- a malformed stutter: a terminal-stutter step followed by a step that is not
  a terminal-stutter step, or a terminal-stutter step in a `step-tuple`
  trace. Whether a terminal-stutter step's pre-state is terminal is a replay
  check ([FR-401](../../functional/analysis/FR-401-replay-hyper-counterexamples.md));
- an `undefined` kind with no `undefined` member, an `undefined` member whose
  `variables` and `state_keys` neither match the traces in number and order
  nor name exactly one of them, a `transition` with `variables` naming more
  than one component, a
  `position` other than `trace_position`, or a `cause` that is not a
  catalogued undefined reason;
- an `initial` index outside its subject's initial states.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-400-AC-1 | Each of the five kinds serializes and reads back byte-identically under RFC 8785 JCS with exactly its members. | Test (TC-345, TC-348) |
| FR-400-AC-2 | A `lockstep` and a `witness-exhausted` counterexample carry equal step counts and one `loop_entry`; a `witness-exhausted` one also carries `position`; a `projected` one carries per-step marks and equal visible counts; a `step-tuple` one carries one prefix-plus-transition trace per execution and no `loop_entry`. | Test (TC-338, TC-339, TC-343, TC-344) |
| FR-400-AC-4 | An `undefined` counterexample carries its `undefined-evaluation` value (variables, joint position, state keys, locus and catalogued reason, with one component and, for a guard, its transition identity when a subject's expression has no value) and reads back with it. `variables` and `state_keys` hold one entry per trace for a body or match locus, however many universal variables the clause has, and one entry for a guard, `terminal when` predicate or state constraint locus. | Test (TC-345, TC-348) |
| FR-400-AC-3 | `lockstep` or `witness-exhausted` traces of unequal step count, mismatched variables or aliases, a misplaced or out-of-range `loop_entry` or `position`, a missing or misplaced `mark`, a terminal-stutter step followed by a non-stutter step or placed in a `step-tuple` trace, an `undefined` kind whose carried value is missing or mismatched, and an out-of-range `initial` each refuse as `invalid_runtime_input`/`invalid-value` before replay. | Test (TC-345) |
| FR-400-AC-5 | The paths of an `undefined` counterexample may differ in step count. For an undefined `μ_U` on an HP-3 joint step the universal paths include the step and the existential paths end at its pre-state, one step short, with `trace_position` the position the step enters; for an HP-6 clause the unmarked paths hold their skipped steps and `trace_position` is the projected position; each `state_keys` entry is the key of the last state of its path, a short existential path's being its pre-state's. | Test (TC-348) |

## Dependencies

- [FR-181](../../functional/runtime/FR-181-simulate-finite-executions.md) owns
  transition identities, state keys and their typed canonical form.
- [FR-399](../../functional/analysis/FR-399-settle-hyperproperty-results.md)
  decides when a counterexample settles an item.
- [FR-401](../../functional/analysis/FR-401-replay-hyper-counterexamples.md)
  replays each kind.
- [FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md)
  owns the replay boundary.
- [FR-364](../../functional/temporal/FR-364-replay-temporal-counterexamples.md)
  owns the model trace step and the terminal-stutter identity.

## References

- STD-136 (this requirement); QSL-370 and its ADR-023 (design).
