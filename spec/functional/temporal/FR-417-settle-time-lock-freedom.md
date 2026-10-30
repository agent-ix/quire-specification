---
id: FR-417
title: "Settle time-lock freedom for every timed subject"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/FR-415
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-161
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-361
    type: references
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
  - target: ix://agent-ix/quire-specification/FR-426
    type: references
  - target: ix://agent-ix/quire-specification/FR-425
    type: references
  - target: ix://agent-ix/quire-specification/FR-418
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# FR-417: Settle time-lock freedom for every timed subject

## Description

When a request contains an item over a timed subject, the request writer SHALL
add one derived time-lock-freedom item for that subject.

## Inputs

The request's items; each distinct timed subject
([FR-415](FR-415-declare-clocks-and-timed-models.md)) among them.

## Outputs

One time-lock-freedom item per distinct timed subject, settled `proved`,
`refuted` with a `time-lock` counterexample, or a non-conclusive result.

## Behavior

### Time-locks

A **time-lock** is a reachable timed state from which no time-divergent
behaviour exists. It is **local** when no positive delay is admissible and no
discrete step is enabled at it, so time stops there; otherwise every
continuation from it is Zeno. Claims over a time-locked state hold vacuously,
because time-divergent behaviours are the admitted set (FR-161), so the
derived item reports every time-lock.

### The derived item

For each distinct timed subject among a request's items the request writer
adds exactly one derived item of fixed kind `time-lock-freedom`, beside the
derived deadlock-freedom item. It has no opt-out. Its obligation identity is
the subject and the item kind. It has no node of its own. It is keyed by the
subject's `model`/`model_root` anchor, the node that the subject's
deadlock-freedom item is keyed by, together with its `item_kind`
`time-lock-freedom`
([FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) "Derived request items"). It settles with the verdict kinds of FR-161.

### Protocol subjects

Time-lock-freedom is owed for timed model subjects only. A protocol subject
([FR-425](../protocol/FR-425-define-protocol-subjects-and-state-keys.md)) is
never a timed subject, even when its model selection reads a timed model: the
protocol system reads no clock ([FR-426](../protocol/FR-426-apply-protocol-steps.md)),
has no delay step, and its steps neither advance nor stop the model's clocks.
Protocol timing semantics is outside v1.1. A request therefore gains no
time-lock-freedom item for a protocol subject, and a timed model subject over
the same model keeps its own item.

### Time-lock counterexample

A refutation carries a counterexample whose `kind` is `time-lock`: a finite
prefix to the time-locked state, ending with the member `final_delay`, the
exact rational delay that reaches it.

- For a local time-lock, replay re-executes the prefix with FR-181 timed
  replay and checks, at the last timed state, that no positive delay is
  admissible and no transition identity is enabled.
- For a non-local time-lock, the evidence is a **trap**: the stem is the
  prefix, and the trap's target is a time-divergent continuation, that is a
  quiescent state or a cycle admitting positive total delay. The trap's
  forward closure holds no target. Replay re-executes the stem, explores the
  forward closure of the last timed state afresh, unreduced, under the
  request's limits, and settles `refuted` only when that exploration
  completes with no target.

Replay agreement settles `refuted` with basis `closed-scope`; a target found
settles `inconclusive`, `replay-parity`; a stopped exploration settles
`incomplete`, `limit-reached` naming the budget. A local time-lock is the
trap whose closure is its one state.

### Deadlocks and time-locks

A timed state is **terminal** when no discrete step is enabled at it or after
any admissible delay ([FR-181](../runtime/FR-181-simulate-finite-executions.md)
"Timed subjects"). A terminal state at a quiescent discrete state (no time
invariant applies and it is not urgent) admits the idle tail of FR-161 and is
a deadlock when it is not intended; the deadlock-freedom item reports it. A
terminal state where the admissible delay is bounded, a **bounded-delay
terminal state**, has no time-divergent behaviour, so it is a time-lock: local
when no positive delay is admissible, non-local otherwise. The time-lock-freedom
item reports it and the deadlock-freedom item does not.

### Proof

A proof carries an [FR-418](../../objects/interfaces/FR-418-zone-certificate.md)
certificate stating, for each node, that a time-divergent target is
reachable, and settles only after the checker accepts it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-417-AC-1 | A request with items over two distinct timed subjects and one untimed subject gains exactly two derived time-lock-freedom items, each with obligation identity (subject, `time-lock-freedom`). | Test (TC-370) |
| FR-417-AC-2 | The strict-guard `Rpc` variant settles the time-lock-freedom item `refuted` with `time-lock` evidence: stem `send` at 0 and final delay 3, and replay finds no admissible positive delay and no enabled identity. | Test (TC-370) |
| FR-417-AC-3 | A non-local time-lock settles `refuted` only after a fresh, unreduced exploration of the trap's forward closure completes with no target; a found target settles `inconclusive`, `replay-parity`. | Test (TC-370) |
| FR-417-AC-4 | In the strict-guard variant the deadlock-freedom item settles `proved` and the time-lock is reported only by the time-lock-freedom item; with non-strict guards both settle `proved`. | Test (TC-370) |
| FR-417-AC-5 | A time-lock-freedom proof settles only after its zone certificate is accepted. | Test (TC-370, TC-371) |
| FR-417-AC-6 | A timed model subject over the model that a protocol subject reads keeps its time-lock-freedom item. | Test (TC-370) |
| FR-417-AC-7 | A time-lock-freedom item adds no node to the checked package. Its claim node is the subject's `model`/`model_root` node, `{package: "acme/orders", path: ["model", "model_root"], ordinal: 0}` in `derived-items-request`, at occurrence `generated` 0. That is the same claim as the subject's deadlock-freedom item, and the two differ only in `item_kind`. | Test (TC-431 BP-51) |

## Dependencies

- [FR-415](FR-415-declare-clocks-and-timed-models.md) supplies time invariants
  and urgency.
- [FR-161](FR-161-analyze-infinite-trace-properties.md) supplies time
  divergence, the idle tail and verdict kinds.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) supplies timed
  replay and exploration from a given state.
- [FR-418](../../objects/interfaces/FR-418-zone-certificate.md) carries the
  proof certificate.

## References

- QSL ADR-026 TD-1 to TD-4, TV-1, EZ-7; QSL ADR-022 trap evidence (its QSpec
  half: STD-135).
- Owner ruling recorded on QSL-373: a non-local time-lock reuses the trap
  evidence, and `time-lock` stays its own counterexample kind.
- Ticket: STD-139 (paired with QSL-373).
