---
id: TC-390
title: "Store buffering under sc, tso and ra"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-434
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-436
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-437
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-439
    type: verifies
---

# TC-390: Store buffering under sc, tso and ra

## Description

Check one source under three resolved memory models against hand-enumerated
state counts, verdicts, budgets and canonical counterexamples, and check the
fenced, `seq_cst` and message-passing variants.

## Test Procedure

Model: object type `Cell` with `v: Int[0, 1]`, population `cells`, universe
`{x, y}`, both 0. Operation `set()`, receiver-scoped frame on `v`,
postcondition `self.v = 1`. Object type `Core` with `r: Int[0, 2]`, population
`cores`, universe `{c1, c2}`, both 2 ("not yet read"). Operation `read(c:
Cell)`, receiver-scoped frame on `r`, postcondition `self.r = pre(c.v)`.

Protocol `SB` with one `parallel Both memory sc` and two branches: `left`
(role `c1`) runs `Sx: set on x ordering release` then `Ly: read(y) on c1
ordering acquire`; `right` (role `c2`) runs `Sy: set on y ordering release`
then `Lx: read(x) on c2 ordering acquire`; `join all [left, right]`.

Claim: `always holds(not (c1.r = 0 and c2.r = 0))`. Submit three requests: no
`memory` member, a `tso` selection and an `ra` selection, each with no memory
budget member. Explore exhaustively through the real local Rust boundary,
order transition identities `left` before `right` and attempts before memory
steps, and record the state counts, verdict, method and counterexample.

## Vectors

| Vector | Resolved model | Expected result |
| --- | --- | --- |
| SB-01 | `sc` | 13 states in the `parallel` and 3 joined states, 16 in all; `proved`, basis `exhaustive`; no memory budget applies |
| SB-02 | `tso` | 34 states in the `parallel` and 4 joined states, 38 in all; `refuted`; method states `max_store_buffer` 4, not reached; canonical counterexample, the first violation in the canonical breadth-first order (attempts before flushes; an attempt with no argument before one with an argument; equal arguments by node, `left` before `right`; `Lx` before `Ly`), at depth 4: `Sx` (left buffer `[x := 1]`), `Sy` (right buffer `[y := 1]`), `Lx` reads `x` from memory (`c2.r = 0`), `Ly` reads `y` from memory (`c1.r = 0`); memory stays `x = 0`, `y = 0` |
| SB-03 | `ra` | 14 states in the `parallel` and 4 joined states, 18 in all; `refuted`; method states `max_messages` 4, not reached; canonical counterexample, in the same order: `Sx` release inserts `x1` with view `(x1, y0)`, `Sy` release inserts `y1` with view `(x0, y1)`, `Lx` acquire reads depth 1 (`x0`), `Ly` acquire reads depth 1 (`y0`), each store at slot depth 0 |
| SB-04 | `tso` and `ra`, with `fence F ordering seq_cst;` between store and load in each branch | `proved` under both |
| SB-05 | `tso` and `ra`, every store and load `seq_cst`, no fences | `proved` under both; under `ra` the second load's choice of the old message is disabled because it would close `Sx → Ly → Sy → Lx → Sx` in the SC event graph |
| SB-06 | message passing: `data := 1; flag := 1` against `r1 := flag; r2 := data`, claim `not (r1 = 1 and r2 = 0)` | `proved` under `sc` and `tso` with any orderings; `proved` under `ra` with a `release` flag store and `acquire` flag load; `refuted` under `ra` with `relaxed` flag accesses |
| SB-07 | SB-02 lasso check | the fairness set of a liveness claim over the `tso` subject lists one weak `each` `flush` constraint per thread, origin `memory`; a loop that leaves a buffer non-empty without flushing it is rejected |
| SB-08 | atom values under `tso` | at SB-02's position 1, `holds(x.v = 1)` is false: the store is buffered |

## Expected Results

Every count, verdict, method statement and counterexample matches the table,
each request carries its own obligation identity, and every refutation
replays to `reproduced-with-evaluated-witness`.
