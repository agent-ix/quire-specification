---
id: TC-226
title: "AOT JIT native parity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-306
    type: verifies
---

# TC-226: AOT JIT native parity

## Description

Compares AOT and JIT runs with the interpreter. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| EB-01 | Every entry of the conformance corpus and of generated programs through the interpreter, AOT and JIT, including undefined, refused and budget-exhausted runs | Each backend returns the interpreter's outcome, typed cause, location, loss records and `incomplete` charge point, and no outcome names a backend | FR-306-AC-1 |
| EB-04 | `--engine fast`; `--engine jit` on a binary that does not link the JIT; an entry the JIT cannot translate | Exit `20` naming the argument; exit `21` before any stage; that entry settles `unsupported` and the interpreter is not run in its place | FR-306-AC-4 |
| EB-05 | An injected JIT internal error; a JIT preparation over its code-size limit | Exit `30` with no verdict; a typed preparation limit failure naming the limit, its value and its setting | FR-306-AC-5 |

## Expected Results

Every backend returns the interpreter's exact outcome.
