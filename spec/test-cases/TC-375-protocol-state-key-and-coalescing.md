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
| PK-08 | A `repeat R max 3` around `P1` with `outstanding continue`; instances 0 and 1 live, then instance 0 ends and is removed while instance 1 runs | The third fork takes ordinal 0, the smallest ordinal no live sibling holds, not 2 |
| PK-09 | PK-03 with `branch b` holding an inner `parallel Q` whose branch `y` is still `running` after instance 0's join settled and `b` completed | The second fork takes ordinal 1, because `y`'s path extends instance 0's branch path; it takes ordinal 0 once `y` completes |
| PK-10 | A template `Refund` with two registrations; the second activates while compensation thread `(0, Refund, 0)` is live, and in a second run after `cend` removed it | The second thread takes ordinal 1; ordinal 0 |
| PK-11 | Inside `repeat` of maximum 2: `parallel P` (`all` join) binding `a` in `left`, then `parallel Q` (`u` reads `a` in a `check`, `v`), `join any … outstanding continue` | After `join(P)` the forking thread's environment holds `a` and `fork(Q)` copies it into `u` and `v`; `P`'s first instance is removed while `Q`'s instance 0 is live; iteration 2's `fork(Q)` takes ordinal 1 with its own `a`; iteration 1's `u` evaluates over its own `a`, no state refuses, and the entry is dropped when `u` completes |
| PK-12 | A binder bound in a branch of an inner `all`-joined `parallel`, read by a node after the outermost join | The entry is held under the inner branch, the enclosing branch and then the outermost joiner's path, and absent after the reading node's step |
| PK-13 | Two protocols of one package, each declaring `channel Pay` | Every `queues` entry and `duplicate` or `lose` identity holds the channel in its own protocol's qualified name |
| PK-14 | A joining thread whose entry is read only by branches of a later `parallel`, and a thread resting at or before the entry's binding node | The joiner keeps the entry through the later fork, which copies it into each branch; the thread at or before the binding node holds no entry for the node |
| PK-15 | The FR-425-AC-11 protocol inside a `repeat`, the reader of `x` after the binding in `x`'s own branch of `Inner`, once with `Inner` joined `any` and once `all` | With `any`, the joiner's environment and `observations` hold no `x` after `join(Outer)`, the back edge notwithstanding; with `all` they hold it while the reach rule keeps it |
| PK-16 | FR-425-AC-12's `Refund` protocol: two registrations `r1`, `r2` distinguished by an `id` capture, activated by two sibling triggers in both orders, each compensation thread's `cattempt` and `cend`, then `commit Committed` and the instance's finish; and its `attempts 2` variant, `RefundAttempt` with field `ok: Boolean`, with the retry relation `not e.ok` and with `l.ok`: each first `cattempt` record, the retry `cattempt` and `cend`, and a step of `Committed` while the thread rests at its retry point | The two orders give distinct keys, each compensation thread's `thread-state` naming its registration; the `commit` entry appears with the commit step and is absent after the instance is removed; in the `attempts 2` variant with retry relation `not e.ok`, the thread's `attempt-record` entry holds the first attempt's record (`ok` false or true) and the two have distinct keys, a retry `cattempt` is enabled after `ok` false and only `cend` after `ok` true, and the entry is absent after the second `cattempt`, after `cend` and after a step of `Committed` taken at the retry point; with retry relation `l.ok` no state holds the entry and the two first `cattempt` steps reach one state (FR-425-AC-12) |
| PK-17 | The `Fill` protocol of TC-376, whose `role w on M` is a role on the whole model of alias `M`: built with no binding for `w`; with a binding supplied for `w`; over the model subject of a second selection `N`; and a variant with `role v on M::Cell` and no binding for `v` | The first builds, and every state's `roles` entry for `w` is `{"type":"model-binding","selection":"<M's identity>"}`; the other three each refuse `invalid_runtime_input`/`wrong-role-mapping` naming the role before any state is expanded (FR-425-AC-13) |

## Expected Results

Every key holds the members `semantic`, `control`, `queues`, `roles`,
`observations`, `bounds` and `memory`, each map sorted by key bytes, with the
entry shapes FR-425 lists. Coalescing is decided by key bytes alone, ordinals
are the smallest free value, and the members PK-05 changes are part of the
obligation identity.
