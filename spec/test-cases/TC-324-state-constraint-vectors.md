---
id: TC-324
title: "State constraint vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-382
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-383
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-385
    type: verifies
---

# TC-324: State constraint vectors

## Description

Cut exploration at a state constraint, keep real violations, never report a
lasso or deadlock through boundary states, and settle a constrained pass
`inconclusive`.

## Test Procedure

Subject: TC-323's counters with `v: Int[0, 3]` and precondition
`self.v < 3`, no partial-order selection, and the constraint

```text
constraint Low using v on M::Counter at current { self.v <= 1 }
```

named by the request.

1. Explore and count stored and expanded states.
2. For instance `x = a`, check each claim with and without the constraint:
   `always holds(x.v <= 1)`; `always holds(x.v <= 2)`; `eventually
   holds(x.v = 3)` with an empty fairness set; the derived deadlock-freedom
   item.
3. Change the constraint body to `self.v` (non-Boolean).
4. Submit the same claim with two different constraint clauses and compare
   obligation identities and terminal records.
5. Check `always holds(x.v <= 3)` with a constraint `self.v <= 3` that no
   reachable state violates.
6. Name a constraint `self.v <= 2 div (1 - self.v)`, undefined where some
   counter has `v = 1`, and check `always holds(x.v <= 3)`.

## Expected Results

Case 1 stores 20 states (8 inside the constraint, 12 boundary states with
one counter at 2) and expands 8 (FR-383-AC-6, FR-382-AC-5). In case 2:
`always holds(x.v <= 1)` settles `refuted` in both runs with the prefix
`bump(a), bump(a)` reaching the boundary state `(2,0,0)`, which the monitor
reads (FR-383-AC-6); `always holds(x.v <= 2)` settles `inconclusive`,
`constraint-reached{boundary_states: 12}`, constrained, and `refuted` at
`(3,0,0)` unconstrained; `eventually holds(x.v = 3)` settles `inconclusive`,
`constraint-reached{12}`, constrained, with no lasso reported because
expanded states form no cycle and boundary states are never
stutter-extended, and `proved` unconstrained; the deadlock-freedom item
settles `inconclusive`, `constraint-reached{12}`, constrained, and `refuted`
at `(3,3,3)` unconstrained (FR-385-AC-5). Case 3 refuses
`ill_typed`/`non-boolean-root` (FR-382-AC-5). In case 4 the obligation
identities are equal, the request identities differ, and the terminal
records name different constraints
(FR-382-AC-2). Case 5 reaches no boundary state and settles `proved`, basis
`closed-scope`, proof basis `exhaustive`, with the constraint recorded in
the terminal record's method (FR-385-AC-3).
In case 6 the item settles `refuted`, cause `undefined-evaluation`, at the
first reached state with a counter at 1, with a replaying counterexample
ending there, and that state is not a boundary state (FR-383-AC-6).
