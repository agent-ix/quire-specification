---
id: TC-226
title: "AOT JIT native and cache parity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-306
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: verifies
---

# TC-226: AOT JIT native and cache parity

## Description

Compares AOT and JIT runs with the interpreter and exercises the content-keyed
cache. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| EB-01 | Every entry of the conformance corpus and of generated programs through the interpreter, AOT and JIT, including undefined, refused and budget-exhausted runs | Each backend returns the interpreter's outcome, typed cause, location, loss records and `incomplete` charge point, and no outcome names a backend | FR-306-AC-1 |
| EB-02 | A `prove` result cached, then the key recomputed after changing the operation, the `package_id`, a claim, a bound, a deterministic budget, the provider's backend identity and the generation target; then after changing a path, a timestamp, a wall-clock deadline, the application version string and the execution backend | Each change of the first group changes the `quire.cache-key/v1` key and misses; each of the second group leaves the key unchanged and hits | FR-306-AC-2 |
| EB-03 | An entry whose stored preimage digest differs from its name, one whose value fails the typed reader, and attempts to store an internal-failure, cancelled, deadline, time-limited and plugin result | Each read is a miss; none of the results is stored | FR-306-AC-3 |
| EB-04 | `--engine fast`; `--engine jit` on a binary that does not link the JIT; an entry the JIT cannot translate | Exit `20` naming the argument; exit `21` before any stage; that entry settles `unsupported` and the interpreter is not run in its place | FR-306-AC-4 |
| EB-05 | An injected JIT internal error; a JIT preparation over its code-size limit | Exit `30` with no verdict; a typed preparation limit failure naming the limit, its value and its setting | FR-306-AC-5 |
| EB-06 | Two conformers computing the cache key of EB-02's first request; the same digest bytes labelled with another domain | The keys are equal; the relabelled digest compares unequal | FR-201-AC-8 |

## Expected Results

Every backend returns the interpreter's exact outcome, and the cache misses on
every meaning change and on nothing else.
