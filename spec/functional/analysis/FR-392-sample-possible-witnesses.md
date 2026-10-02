---
id: FR-392
title: "Sample witnesses for possibility claims"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-390
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-391
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-368
    type: references
  - target: ix://agent-ix/quire-specification/FR-461
    type: references
---

# FR-392: Sample witnesses for possibility claims

## Description

When a `possible` item is checked by exhaustive exploration and the request's
`witness_samples` is greater than zero, the analyzer SHALL first draw
`witness_samples` seeded random walks from each initial state and SHALL keep
each walk that reaches a node where `P` holds as that initial state's witness.

## Inputs

The `possible` item and its instance binding, the subject's initial states in
their canonical order, the request's `seed` and `witness_samples`, the
`max_walk_steps` member of its temporal `limits`, and the run's search
horizon.

## Outputs

Zero or more sampled witnesses, each with its seed and trace index.

## Behavior

- **Request members.** `witness_samples` and `seed` are sampling settings of
  the request's `temporal` member
  ([FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)),
  method parameters, and 0 `witness_samples` turns sampling off. `seed` is a
  decimal string as FR-181 sampling reads it. `max_walk_steps` is the limit
  on the steps of one walk, a member of the temporal `limits`
  ([FR-368](../temporal/FR-368-negotiate-temporal-model-check-items.md)) with
  the setting name `model_check.max_walk_steps` by the
  [FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md) naming rule. The routed provider
  publishes the default of each, which applies when the request sets none. The run's `seed`, `witness_samples` and `max_walk_steps` are
  recorded in its FR-331 terminal record.
- **Walks.** Walk `r` from the initial state with index `i`, for
  `0 <= r < witness_samples`, is the FR-181 sampled trace with the request
  seed and trace index `i × witness_samples + r`, drawn from the same
  successor relation as exhaustive exploration. A walk takes at most
  `max_walk_steps` steps and at most the search horizon's number of steps,
  whichever is fewer, so every walk ends, also in a cyclic model with no
  reachable target. The search horizon is the run's maximum depth, a method
  parameter that the result states; it is not a limit. A walk that reaches
  either bound, or a terminal state, ends and decides nothing; when a walk
  ends at `max_walk_steps`, the terminal record names `max_walk_steps`, its
  value and its setting `model_check.max_walk_steps`, rendered as FR-461 states, and exploration
  still runs. A walk stops at the first node where `P` holds and is then a witness
  for its initial state with source `sampled` carrying that seed and trace
  index. The trace index is computed with checked arithmetic; a
  `witness_samples` whose largest trace index overflows is refused before
  sampling as `invalid_runtime_input`/`invalid-value`, naming
  `witness_samples`.
- **Settlement.** Sampling is the fast path for finding witnesses; it does
  not settle the item. Exhaustive exploration always runs after it, keeps
  every sampled witness, finds the witnesses sampling missed, and checks the
  claim's well-definedness at every reachable node (FR-391 "Well-definedness
  is global"). The item settles `proved` from its witnesses only when that
  exploration completes with no open node and no undefined evaluation. A
  sampled witness with an undefined evaluation elsewhere settles `refuted`,
  cause `undefined-evaluation`. A sampled witness for every initial state on a
  run that a limit stops, or that ends with open nodes, before refuting
  evidence is found settles `inconclusive`, cause `well-definedness-unchecked`,
  stating that a witness was found and well-definedness was not checked. A
  walk that does not reach `P` decides nothing.
- **Verdict independence.** Sampling only adds witnesses. A witness proves
  `possible` whichever phase found it, so on a run that exhaustive exploration
  completes, `proved` from a sampled witness is the verdict exploration alone
  gives. The verdict never depends on `witness_samples` or the seed: sampling
  changes which witness is reported and its source, never the label, the basis
  or the cause. It never settles one that exploration refutes, since a witness
  and a trap at the same initial state cannot both exist, and an undefined
  evaluation refutes whichever phase found the witness.
- **Reproducibility.** Walks are FR-181 counter-based samples named by seed
  and trace index, so two runs with the same subject, claim, limits and seed
  draw the same walks.

A sampled witness replays from its recorded steps, without re-running the
sampler.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-392-AC-1 | A request that sets no `witness_samples` or `seed` runs under the routed provider's published defaults, and the terminal record states the values the run used. | Test (TC-336) |
| FR-392-AC-2 | Walk `r` from initial state `i` is the FR-181 sampled trace with the request seed and trace index `i × witness_samples + r`, and stops at the first node where `P` holds. | Test (TC-336) |
| FR-392-AC-3 | When every initial state has a sampled witness and exploration completes with no open node and no undefined evaluation, the item settles `proved`, `decisive-witness`, with each source `sampled` naming its seed and trace index. | Test (TC-331, TC-336) |
| FR-392-AC-4 | With `witness_samples` 0 no walk is drawn and the same item settles the same verdict with sources `explored`. | Test (TC-331, TC-336) |
| FR-392-AC-5 | A model whose `possible` item exploration refutes with a trap settles `refuted` under every seed and every `witness_samples`. | Test (TC-336) |
| FR-392-AC-6 | A model with a sampled witness for every initial state and an undefined predicate at a reachable node off every walk settles `refuted`, cause `undefined-evaluation`, with `witness_samples` 64 and with `witness_samples` 0. | Test (TC-336) |
| FR-392-AC-7 | A model with a sampled witness for every initial state whose exploration a limit stops before refuting evidence is found settles `inconclusive`, cause `well-definedness-unchecked`, with the sampled witnesses and the statement that well-definedness was not checked. | Test (TC-336) |
| FR-392-AC-8 | The terminal record states the search horizon the run used, and a walk that reaches it decides nothing; the horizon is not a member of the temporal limits. | Test (TC-336) |
| FR-392-AC-9 | Every walk ends within `max_walk_steps` steps; a walk on a cyclic model with no reachable target ends at that limit, the terminal record names `model_check.max_walk_steps`, its value and its setting as FR-461 states, and exploration settles the item. | Test (TC-336) |

## Dependencies

- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns sampled
  traces and their counter-based generator.
- [FR-390](FR-390-analyze-state-graph-claims.md) states the `possible` form.
- [FR-391](FR-391-settle-state-graph-claims.md) settles the item and states
  the settlement method.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  carries the request members and the terminal record.
- [FR-368](../temporal/FR-368-negotiate-temporal-model-check-items.md) owns the
  temporal `limits` member that `max_walk_steps` joins.

## References

- STD-135 (this requirement); QSL-369 and its ADR-022 (design).

