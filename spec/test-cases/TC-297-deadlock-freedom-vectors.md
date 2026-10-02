---
id: TC-297
title: "Deadlock-freedom vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-366
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-364
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-365
    type: verifies
---

# TC-297: Deadlock-freedom vectors

## Description

The deadlock vectors: a refuted deadlock-freedom item with a prefix that
replays, the same model with `terminal when` covering the deadlocked state, and
with `terminal any`, plus the rules that keep authored claims unchanged.

## Test Procedure

The **Job** subject: object type `Job` with `status` of an enum
`{queued, running, done, stuck}`; operation `start` (`queued` to `running`),
`finish` (`running` to `done`) and `fail` (`running` to `stuck`). Universe
`{j}`, initial `status = queued`. `done` and `stuck` are reachable and
successor-free.

1. Request `always eventually holds(j.status = done)` under
   `quire.temporal.infinite-trace/v1` with no `terminal` declaration.
2. Add `terminal Finished using v on M when { … }` whose predicate holds at
   `done` and at `stuck`, and repeat.
3. Change the predicate to hold at `done` only, and repeat.
4. Replace the declaration with `terminal Halts using v on M any`, and repeat.
5. Add a second temporal item over the same subject, then a third item over a
   subject with universe `{j, k}`.
6. Add a second `terminal` declaration for the same model alias.
7. Change the step 3 predicate to hold at `running` and `done`.
8. Replay a deadlock counterexample whose last state is `running`.

## Expected Results

1. The request holds one deadlock-freedom item, settled `refuted`, basis
   `decisive-counterexample`, with a `kind: deadlock` finite prefix to the first
   deadlocked state in canonical breadth-first order, which replays
   (FR-366-AC-1, FR-364-AC-1, FR-365-AC-5). The authored claim settles
   `refuted` with a terminal stutter lasso at `stuck` (FR-366-AC-5).
2. The deadlock-freedom item settles `proved`, proof basis `exhaustive`; the
   authored claim's verdict is unchanged (FR-366-AC-2, FR-366-AC-5).
3. The item settles `refuted` with a prefix ending at `stuck` (FR-366-AC-2).
4. The request holds no deadlock-freedom item; the authored claim's verdict is
   unchanged (FR-366-AC-3).
5. Two items over one subject share one deadlock-freedom item; the
   two-object subject receives its own, with identity (subject,
   `deadlock-freedom`) (FR-366-AC-4).
6. Refuses `ambiguous_declaration`/`ambiguous-name` naming both declarations
   (FR-366-AC-6).
7. `running` is not terminal and is never reported as intended or as a
   deadlock; the item still settles `refuted` at `stuck` (FR-366-AC-7).
8. `inconclusive`, cause `replay-parity`: `finish` and `fail` are enabled at
   `running` (FR-364-AC-6).
