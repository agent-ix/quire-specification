---
id: TC-296
title: "Canonical counterexample of an explicit-state analysis"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-365
    type: verifies
---

# TC-296: Canonical counterexample of an explicit-state analysis

## Description

Determinism and construction of the canonical lasso: the stem, the greedy-walk
loop and its search count.

## Test Procedure

1. Run an exhaustive explicit-state analysis of the TC-293 step 2 claim ten
   times, with backends registered in different orders and with different
   hash seeds.
2. Inspect the returned lasso of that claim.
3. Instrument the greedy walk and count its breadth-first searches on the
   TC-293 step 2 claim and on the TC-299 `weak each` claim.
4. On a subject whose first passing component holds two acceptance-set states
   and one weak and one strong obligation, inspect the loop.

## Expected Results

1. Ten byte-identical counterexamples (FR-365-AC-1).
2. An empty stem, since the initial product state lies in the first passing
   component, and a loop of three `upd(a)` steps from `(0,0)` back to `(0,0)`
   (FR-365-AC-2, FR-365-AC-3).
3. At most one search per obligation plus one in each run (FR-365-AC-3).
4. The loop visits both acceptance-set states and discharges the weak and the
   strong obligation; the lasso replays as fair (FR-365-AC-2).
