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
| EB-02 | Cache a `prove` result using one admitted FR-331 provider manifest. Recompute the key after changing one input at a time: the operation, the `package_id`, a claim, a bound, a deterministic budget, or the provider's `BackendId` identity string. In separate runs with the same `BackendId`, `package_id` and request, change only the manifest's tool version, then each other non-identity manifest member in turn to another admitted value, recomputing its domain-retaining `ManifestDigest` from the canonical manifest bytes. Separately cache a `generate` artifact and change only its target, then only a target option. For each baseline, change only a path, a timestamp, a wall-clock deadline, the application version string or the execution backend | Each key-input change changes the `quire.cache-key/v1` key and misses, including each manifest-only change with the same `BackendId`; the old provider result is not reused. Each outside-key change leaves the key unchanged and hits. Target and target-option changes are exercised only for `generate` | FR-306-AC-2 |
| EB-03 | An entry whose stored preimage digest differs from its name, one whose value fails the typed reader, and attempts to store an internal-failure, cancelled, deadline, time-limited and plugin result | Each read is a miss; none of the results is stored | FR-306-AC-3 |
| EB-04 | `--engine fast`; `--engine jit` on a binary that does not link the JIT; an entry the JIT cannot translate | Exit `20` naming the argument; exit `21` before any stage; that entry settles `unsupported` and the interpreter is not run in its place | FR-306-AC-4 |
| EB-05 | An injected JIT internal error; a JIT preparation over its code-size limit | Exit `30` with no verdict; a typed preparation limit failure naming the limit, its value and its setting | FR-306-AC-5 |
| EB-06 | Two conformers independently computing the key from the same closed `{operation, package_id, request, backend, manifest, target?}` object for EB-02's `prove` baseline, its same-backend tool-version variant, an `analyze` request and its `generate` baseline: `backend` is the identity string alone, `manifest` is the provider's `ManifestDigest` over the FR-331 canonical manifest bytes retaining `quire.tool-manifest.jcs/v1`, and `target` holds the target and options only for `generate`. Supply each object's members in different orders; compare a key to the same digest bytes labelled with another domain | Each preimage has exactly the five required members and, only for `generate`, `target`; RFC 8785 canonical bytes and keys agree between conformers regardless of supplied member order. The same-backend tool-version variant's key differs from the `prove` baseline's key. The relabelled digest compares unequal | FR-201-AC-8 |

## Expected Results

Every backend returns the interpreter's exact outcome, and the cache misses on
every meaning change and on nothing else.
