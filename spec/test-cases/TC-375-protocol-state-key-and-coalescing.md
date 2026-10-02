---
id: TC-375
title: "Protocol state key and coalescing"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-425
    type: verifies
---

# TC-375: Protocol state key and coalescing

## Description

Explore small protocol subjects and check that the state key holds exactly
the protocol members of FR-425, that equal keys coalesce, that ordinals are
reused, and that the subject's members enter the obligation identity.

## Test Procedure

Build each vector's checked package and protocol subject and explore it
exhaustively through the real local Rust boundary. For each reachable state,
record its state-key bytes and the member entries. Compare the reachable state
set, the coalesced pairs and the obligation identities with the expected
results.

## Vectors

| Vector | Subject | Expected result |
| --- | --- | --- |
| PK-01 | The `Fill` protocol of TC-376 with `TwoPre: self.v <= 1` | The two behaviours `A` then `B` and `B` then `A` reach distinct model states (2 and 3) and so distinct keys; the two finished states differ only in `semantic`; every state's `observations` member is empty, since no node reads `a` or `b` |
| PK-02 | Two branches each attempting `inc` on distinct cells `c1`, `c2` under receiver-scoped frames | The state after `A` then `B` and the state after `B` then `A` have byte-identical keys and are one state |
| PK-03 | A `parallel` `P3` nested three deep (`P1` > `P2` > `P3`) inside `repeat R max 2` with `outstanding continue` on `P1`, whose `any` join settles after its fast branch | While the first `P1` instance's slow branch still runs, the second iteration's fork creates parallel instance ordinal 1 with branch thread paths ending in ordinal 1; once the first instance is removed, a later fork reuses ordinal 0; the reachable state set is finite |
| PK-04 | A branch binder `x` read by a node after an `all` join, and a binder `y` read by no node | `y` is absent from `observations` in the post-state of its own step; `x` is present from its step until the reading node's step and absent after it |
| PK-05 | One protocol checked with a different `over` binding, a different static role binding, `terminal any` added, `scheduling adversarial` added, and a `tso` selection | Each change gives a different obligation identity from the base subject |
| PK-06 | An `on origin` subject with two initial model states | Exactly two initial protocol states, each holding instance 0 with its root settled at its first rest point |
| PK-07 | A replicated role whose population has no universe, and an event node whose record type has an unbounded `Int` field | Each returns `requires-bound` naming the root before any state is expanded |

## Expected Results

Every key holds the members `semantic`, `control`, `queues`, `roles`,
`observations`, `bounds` and `memory`, each map sorted by key bytes, with the
entry shapes FR-425 lists. Coalescing is decided by key bytes alone, ordinals
are the smallest free value, and the members PK-05 changes are part of the
obligation identity.
