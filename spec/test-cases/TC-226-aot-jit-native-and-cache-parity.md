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
| EB-02 | Separately cache a `prove` terminal record, an `analyze` terminal record and a `generate` artifact, each with a complete admitted FR-331 provider descriptor selected and serialized as FR-306 requires. For each operation, read its baseline entry with the unchanged descriptor. Then, in separate runs/registry snapshots, hold `BackendId`, `package_id`, request and target/options fixed and mutate each admitted non-identity descriptor member in turn: `advertises`, `domains`, `options`, `bounds`, `obligation_forms`, `reductions`, `evidence_kinds` and `fairness_kinds`. Choose fixtures where each single-member mutation preserves admission and conditional field obligations; recompute the manifest identity from the selected descriptor, and attempt to read the old entry through that operation's actual cache consumer. Also change one key input at a time: operation, `package_id`, claim, bound, deterministic budget or backend identity; for `generate` alone change its target, then a target option. For each operation's baseline, change only a path, timestamp, wall-clock deadline, application version string or execution backend | For all three operations, the unchanged descriptor gives the same key and a store/read hit equal to the baseline value. Every admitted manifest-member mutation changes the key and the actual store/read misses without returning the old result, including non-identity mutations with the same `BackendId`; key comparison alone does not satisfy this row. Each other key-input mutation changes the key and misses. Outside-key changes leave the key unchanged and hit. Target and target-option changes are exercised only for `generate` | FR-306-AC-2 |
| EB-03 | An entry whose stored preimage digest differs from its name, one whose value fails the typed reader, and attempts to store an internal-failure, cancelled, deadline, time-limited and plugin result | Each read is a miss; none of the results is stored | FR-306-AC-3 |
| EB-04 | `--engine fast`; `--engine jit` on a binary that does not link the JIT; an entry the JIT cannot translate | Exit `20` naming the argument; exit `21` before any stage; that entry settles `unsupported` and the interpreter is not run in its place | FR-306-AC-4 |
| EB-05 | An injected JIT internal error; a JIT preparation over its code-size limit | Exit `30` with no verdict; a typed preparation limit failure naming the limit, its value and its setting | FR-306-AC-5 |
| EB-06 | For each of EB-02's `prove`, `analyze` and `generate` baselines and admitted same-backend descriptor variants, give two conformers the selected complete admitted FR-331 `BackendDescriptor` value, independently supplied in different object-member orders, and the same remaining inputs. Each conformer independently canonicalizes that descriptor, hashes those bytes in `quire.tool-manifest.jcs/v1`, serializes the resulting manifest identity using FR-306's referenced FR-322 / QSL ADR-013 O-18/C-16 `{domain, algorithm, digest}` carrier, and constructs/canonicalizes/hashes the closed cache object. Give neither conformer a shared precomputed manifest digest. Also vary the supplied cache-object member order and compare a key to equal digest bytes labelled with another domain | Both conformers select only the complete descriptor at `/manifest/0` in the single-provider `ManifestStage`, excluding the array, hello wrapper, registry snapshot, request and accounting from the manifest preimage. Manifest canonical bytes, digest records with explicit domain/algorithm and exact lowercase hex, cache canonical bytes and keys agree despite supplied object-member order. Each cache preimage has exactly the five required members and, only for `generate`, `target`. Every admitted same-backend descriptor variant has a key different from its baseline. The relabelled digest compares unequal | FR-201-AC-8 |

## Expected Results

Every backend returns the interpreter's exact outcome, and the cache misses on
every admitted key-input change and on nothing else. Planned cache consumption
coverage includes store/read hits and manifest-only misses for `prove`,
`analyze` and `generate`.

An external tool-version change with identical admitted provider descriptor
bytes leaves the manifest digest and cache key identical. No tool-version field
is admitted in the selected descriptor, and outcome accounting cannot supply
one. Whole tool-upgrade invalidation is excluded from these procedures until an
owner-admitted provider-manifest content contract supplies that prerequisite;
these planned cases do not claim that scenario implemented or qualified.
