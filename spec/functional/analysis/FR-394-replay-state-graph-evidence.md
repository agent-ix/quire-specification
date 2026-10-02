---
id: FR-394
title: "Replay state-graph evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-393
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-391
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
  - target: ix://agent-ix/quire-specification/FR-396
    type: references
---

# FR-394: Replay state-graph evidence

## Description

When state-graph evidence reaches the replay boundary, the replay
boundary SHALL settle the item only from a fresh replay of that evidence
through the native transition relation by the rule of its arm.

## Inputs

The `GraphEvidence` value ([FR-393](../../objects/interfaces/FR-393-state-graph-evidence.md)),
the package bytes, the subject's initial states, universes and proof bounds,
the claim and its instance, and the request's limits.

## Outputs

`reproduced-with-evaluated-witness` with the item's verdict, or
`inconclusive` with `replay-parity`, or FR-331 result `incomplete` with cause `limit-reached{limit, value, setting}` (FR-360
`failed`, execution `resource-incomplete`).

## Path re-execution

Replay recompiles the package, re-admits the subject from the byte provision
and trusts no engine claim. A `ModelPath` re-executes from the initial state its `initial` index names,
through the FR-181 transition relation, one step at a time. A step is refused
when its transition is not enabled at the current state or when no successor
of that transition has the recorded post-state digest. The source of a witness
changes no replay step: a sampled, explored or unrolled witness replays from
its recorded steps, without re-running the sampler or the engine.

## Replay rules

| Arm | Replay | Agreement settles |
| --- | --- | --- |
| `witness` | Re-execute each path and evaluate `P` at its last node with the instance bound. | `P` true at every last node: `reproduced-with-evaluated-witness`; the item then settles by [FR-391](FR-391-settle-state-graph-claims.md): `proved` after a completed exploration with no undefined evaluation, else `inconclusive` with `well-definedness-unchecked` |
| `trap` | Re-execute the stem and evaluate `Q` (`always possible`) or `X` (`unique path`) at its last node; for `possible` the stem is an initial state. Then explore the forward closure of the stem's last node with FR-181 exhaustive exploration from that state, unreduced, under the request's limits, evaluating the target predicate at every node. | Exploration completes with no target node: `reproduced-with-evaluated-witness`, item `refuted` |
| `path-pair` | Re-execute the stem and evaluate `X` at its last node; re-execute `first` and `second` from that node; check that each ends at a node where `Y` holds, that `Y` is false at every earlier node of each, and that the two step sequences differ. | Every check holds: `reproduced-with-evaluated-witness`, item `refuted` |
| `undefined` | Re-execute the stem, check that its last state's key is the carried `where.state_key`, and evaluate at that node, with the instance bound, the expression the arm's carried `undefined` value names: the claim predicate its `where.predicate` names, the guard of the transition its `where.transition` names, or else the `terminal when` predicate or state constraint at its `where.locus`. | The undefined value with the carried locus and catalogued reason: `reproduced-with-evaluated-witness`, item `refuted` with that `undefined-evaluation` |
| `witness` with `loop_entry` (single-existential claim, [FR-396](FR-396-classify-hyperproperty-forms.md)) | Re-execute each path as a lasso whose last step closes the loop at `loop_entry`; check the claim's fairness set on the loop as [FR-364](../temporal/FR-364-replay-temporal-counterexamples.md) checks a lasso's fairness; evaluate the body over the lasso with the infinite-trace evaluator. | Every lasso closes, is fair and makes the body true: `reproduced-with-evaluated-witness`; the item then settles by FR-391 as the `witness` row states |
| `trap` of a single-existential claim | The stem is empty. Re-explore the named initial state's part of the product of the subject with the body's Büchi automaton, unreduced, under the request's limits, and search it for an accepting cycle that the claim's fairness filter admits. | Exploration completes with no fair accepting cycle: `reproduced-with-evaluated-witness`, item `refuted` |

- When a step is refused, a predicate evaluates false where the rule requires
  true, a trap's closure holds a target node, the two sequences of a path
  pair are equal, an `undefined` arm's expression is defined or differs from
  the carried value in state, locus or reason, a lasso does not close or is
  unfair or makes the body false, or a single-existential trap's product holds
  a fair accepting cycle, the replay boundary SHALL settle the item `inconclusive`
  with `replay-parity` and SHALL construct no verdict from the evidence.
- When a limit stops a trap's closure exploration or a product re-exploration, the replay boundary SHALL
  settle the item `incomplete` with cause `limit-reached{limit, value, setting}`, naming the limit, its value and the request
  setting that raises it, and a cancelled replay `incomplete` with cause
  `cancelled`.
- A trap's replay re-establishes the unreachability of the target by
  exploring the closure again, so it costs the size of that closure. Under a
  reduction the closure explored is the unreduced one.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-394-AC-1 | A witness whose every path re-executes and ends at a node where `P` holds reproduces, and the item then settles by FR-391: `proved` after a completed exploration with no undefined evaluation, else `inconclusive` with `well-definedness-unchecked`; a witness whose last node fails `P` settles `inconclusive`, `replay-parity`. | Test (TC-331, TC-335) |
| FR-394-AC-2 | A trap whose stem re-executes and whose closure, explored afresh from its last node, holds no target node reproduces and settles the item `refuted`; a trap whose closure reaches the target settles `inconclusive`, `replay-parity`. | Test (TC-331, TC-332, TC-335) |
| FR-394-AC-3 | A path pair whose two sequences each reach `Y` first at their last node and differ reproduces and settles `refuted`; a path pair whose sequences are equal settles `inconclusive`, `replay-parity`. | Test (TC-333, TC-335) |
| FR-394-AC-4 | A step whose transition is not enabled, or whose recorded post-state digest matches no successor, settles `inconclusive`, `replay-parity`. | Test (TC-335) |
| FR-394-AC-5 | A sampled, an explored and an unrolled witness with the same steps replay identically, with no sampler or engine present. | Test (TC-335) |
| FR-394-AC-6 | A trap's closure exploration that a limit stops settles `incomplete` with cause `limit-reached` naming the limit, its value and its setting; a cancelled replay settles `incomplete` with cause `cancelled`. | Test (TC-335) |
| FR-394-AC-7 | An `undefined` arm whose stem re-executes and whose named expression (claim predicate, guard of the named transition, `terminal when` predicate or state constraint) evaluates at the last node to the arm's carried undefined value, with its state key, locus and reason, reproduces and settles `refuted`; a defined value there, or another locus or reason, settles `inconclusive`, `replay-parity`. | Test (TC-349) |
| FR-394-AC-8 | A single-existential witness lasso that closes at its `loop_entry`, is fair under the claim's fairness set and makes the body true reproduces, and the item then settles by FR-391 as AC-1 states; an unfair lasso or one on which the body is false settles `inconclusive`, `replay-parity`. | Test (TC-342) |
| FR-394-AC-9 | A single-existential trap reproduces when the named initial state's product part, re-explored unreduced, holds no fair accepting cycle; a fair accepting cycle found settles `inconclusive`, `replay-parity`; a stopped re-exploration settles `incomplete` with cause `limit-reached`. | Test (TC-342) |

## Dependencies

- [FR-393](../../objects/interfaces/FR-393-state-graph-evidence.md) owns the
  evidence wire.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the transition
  relation and exploration from a given state.
- [FR-391](FR-391-settle-state-graph-claims.md) settles the item from the
  replay.
- [FR-197](../ir-proof/FR-197-replay-backend-counterexamples.md) owns the
  replay boundary.

## References

- STD-135 (this requirement); QSL-369 and its ADR-022 (design).
