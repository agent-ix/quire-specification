---
id: TC-425
title: "Deep inputs settle on their content at every stage"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-460
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-146
    type: verifies
---

# TC-425: Deep inputs settle on their content at every stage

## Description

Drives inputs 100,000 levels deep through every stage and shows that each one
is admitted or refused on its content, that no stage reaches host stack
exhaustion, and that no stage has a depth limit kind or setting. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| DP-01 | A 100,000-term sum `x + x + ... + x`, under node, token and work limits set to fit it | Parses, checks, lowers to a checked package, reads back and evaluates to the same value as the equivalent `100000 * x` | FR-460-AC-1 |
| DP-02 | A 100,000-deep `else if` chain and a 100,000-deep `let` chain, under limits set to fit them | Each parses, checks, lowers, reads back and evaluates to the value a 3-deep chain of equal meaning gives | FR-460-AC-1 |
| DP-03 | A 100,000-deep JSON array and a 100,000-deep JSON object, under a byte limit set to fit them | Each is read and encoded, and the encoding equals its RFC 8785 text | FR-460-AC-2 |
| DP-04 | A 100,000-long recursive list value | It evaluates, compares equal to a copy of itself and is released | FR-460-AC-2 |
| DP-05 | DP-01 to DP-04, each on a thread with a 256 KiB call stack | Each completes with the result above and no host stack exhaustion | FR-460-AC-3 |
| DP-06 | Every limit outcome, effective-limit record and diagnostic produced by DP-01 to DP-05 and by every stage's limit tests; a `--limit` or replay `stage_limits` entry naming a depth | No outcome, record or diagnostic carries a depth or nesting limit kind; the depth setting refuses before any stage runs | FR-460-AC-4 |

## Expected Results

Every deep input is processed on its content under the resource limits it
was given, whatever its depth, on a small fixed call stack. No stage reports
or accepts a depth limit.
