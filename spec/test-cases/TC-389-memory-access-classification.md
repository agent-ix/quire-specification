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
the one-location rule at check time and at the pre-check, the ordering-mismatch
shape at the pre-check, the shared-guard
rule, the reader rule, access application over the thread observation, and fences.

## Test Procedure

Check each vector's protocol through the real local Rust boundary and record
each attempt's class, each refusal with its code and payload, and the
item's settlement.

## Vectors

| Vector | Protocol | Expected result |
| --- | --- | --- |
| AC-01 | `SB` of TC-390 | `Sx`, `Sy` atomic stores; `Ly`, `Lx` atomic loads; `(x, v)` and `(y, v)` shared; `(c1, r)` and `(c2, r)` branch-local |
| AC-02 | an attempt that touches no location, shared or branch-local, with `ordering relaxed`; an attempt that only writes with `ordering acquire`; an attempt that only reads with `ordering release`; an RMW `incr` with no ordering; a `release` attempt that reads `x.v` and writes the register `r` | the first three refuse `ill_typed`/`operator-ineligible` at check time naming the attempt and ordering; the last two are admitted at check time (FR-435-AC-2) |
| AC-03 | each class with each admitted ordering, and a local attempt with none | admitted |
| AC-04 | `parallel memory tso` with an attempt writing two shared fields of its receiver | refuses `unsupported_construct`/`declaration-form` naming both locations |
| AC-05 | the AC-04 attempt under source `sc`, request selecting `tso` | settles `unsupported`, cause `weak-access-shape` with its site, transition and shape `multi-location` naming both locations |
| AC-06 | the AC-04 attempt under `sc` with no selection | admitted; explored as one step |
| AC-07 | a choice guard reading `x.v` directly, under `memory tso`, under `memory sc`, and under source `sc` with a `tso` selection; then the guard rewritten over the binder of a load attempt of `x.v` | each direct read refuses `ill_typed`/`operator-ineligible` at the read naming `(x, v)`; the rewritten guard is admitted |
| AC-08 | under `tso`, a load `L` with precondition `x.v = 1` after the other thread's buffered store `x := 1` | `L` is not enabled until the store is flushed |
| AC-09 | under `tso`, an RMW `cas(0, 1)` on `x` whose post-state leaves `x` unchanged because `x = 1` | a load with the read part of its ordering: no locked write occurs |
| AC-10 | `fence F ordering relaxed;` | `refused { code: invalid_syntax, cause: unexpected-token }` at `relaxed` |
| AC-11 | `fence F ordering seq_cst;` | a `fence` step with transition identity members `thread`, `node` and `ordering`, uncounted for intervals |
| AC-12 | under a `tso` selection, a branch holding an `event` whose constraint reads `x.v` beside the branch whose attempt stores `x.v`; the same under `sc`; an `event` reading a field of an object that no attempt accesses; a `finish` reading `x.v` after the join | the first settles `unsupported`, cause `weak-access-shape`, shape `multi-location` naming the event and `(x, v)`; the other three are admitted (FR-435-AC-7) |
| AC-13 | under `ra`, a branch storing `x := 1` once and a second branch repeating an attempt that stores `x := 0` with precondition `x.v = 0` and then loads `x` with precondition `x.v = 1` | after the first branch's store, the second branch's store is enabled while its current view points at the message with value 0 and is not enabled once its view points at the message with value 1; its load is enabled for the messages at or after its view whose value is 1 (FR-435-AC-8) |
| AC-14 | under a `tso` selection, then an `ra` selection, then `sc`: an instantiated load with `release`, a load with `acq_rel`, a store with `acquire`, a store with `acq_rel`, an RMW with no ordering, a `release` attempt that writes a branch-local register and touches no shared location, and an `acquire` attempt that reads a branch-local register and writes two shared fields | under `tso` and `ra`, the first six settle `unsupported`, cause `weak-access-shape`, before any state is explored, with the `ordering-mismatch` shapes FR-435-AC-9 spells (`access` `load`, `load`, `store`, `store`, `rmw`, `local`; `ordering` `release`, `acq_rel`, `acquire`, `acq_rel`, `null`, `release`), and the seventh settles `multi-location` naming both fields; under `sc` all seven are admitted (FR-435-AC-9, FR-439-AC-11) |

## Expected Results

Every class, refusal and settlement matches the table.
