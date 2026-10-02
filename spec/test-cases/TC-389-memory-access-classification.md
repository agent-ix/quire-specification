---
id: TC-389
title: "Memory access classification, orderings and fences"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-435
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-439
    type: verifies
---

# TC-389: Memory access classification, orderings and fences

## Description

Check shared and branch-local locations, access classes, admitted orderings,
the one-location rule at check time and at the pre-check, the shared-guard
rule, access application over the thread observation, and fences.

## Test Procedure

Check each vector's protocol through the real local Rust boundary and record
each attempt's class, each refusal with its code and payload, and the
item's settlement.

## Vectors

| Vector | Protocol | Expected result |
| --- | --- | --- |
| AC-01 | `SB` of TC-390 | `Sx`, `Sy` atomic stores; `Ly`, `Lx` atomic loads; `(x, v)` and `(y, v)` shared; `(c1, r)` and `(c2, r)` branch-local |
| AC-02 | a store with `ordering acquire`; a load with `ordering release`; an RMW `incr` with no ordering | each refuses `ill_typed`/`operator-ineligible` naming the attempt, class and ordering |
| AC-03 | each class with each admitted ordering, and a local attempt with none | admitted |
| AC-04 | `parallel memory tso` with an attempt writing two shared fields of its receiver | refuses `unsupported_construct`/`declaration-form` naming both locations |
| AC-05 | the AC-04 attempt under source `sc`, request selecting `tso` | settles `unsupported`, cause `weak-access-shape` with its site, transition and shape `multi-location` naming both locations |
| AC-06 | the AC-04 attempt under `sc` with no selection | admitted; explored as one step |
| AC-07 | a choice guard reading `x.v` directly, under `memory tso`, under `memory sc`, and under source `sc` with a `tso` selection; then the guard rewritten over the binder of a load attempt of `x.v` | each direct read refuses `ill_typed`/`operator-ineligible` at the read naming `(x, v)`; the rewritten guard is admitted |
| AC-08 | under `tso`, a load `L` with precondition `x.v = 1` after the other thread's buffered store `x := 1` | `L` is not enabled until the store is flushed |
| AC-09 | under `tso`, an RMW `cas(0, 1)` on `x` whose post-state leaves `x` unchanged because `x = 1` | a load with the read part of its ordering: no locked write occurs |
| AC-10 | `fence F ordering relaxed;` | refuses `ill_typed`/`operator-ineligible` |
| AC-11 | `fence F ordering seq_cst;` | a `fence` step with transition identity members `thread`, `node` and `ordering`, uncounted for intervals |

## Expected Results

Every class, refusal and settlement matches the table.
