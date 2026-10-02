---
id: TC-339
title: "∀∃ safety hyperproperty and the witness-set limit"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-395
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-399
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-400
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-401
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-191
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-339: ∀∃ safety hyperproperty and the witness-set limit

## Description

Check `hyper Opaque using inf over behaviours (v: V::Vault) forall trace a of V exists trace b of V match { a.step.Vault::step.i = b.step.Vault::step.i } { holds(v.h @ a != v.h @ b) and always holds(v.l @ a = v.l @ b) }`. The vault subject: model `V` with object type `Vault`, fields `h: Int[0, 1]` (secret) and `l: Int[0, 1]` (public output), population `vaults` with universe `{v}`, one operation `step(i: Int[0, 1])` with frame `modifies [l]` and no precondition, and initial snapshots `(h, l) = (0, 0)` and `(1, 0)`. The leaky model's post clause is `self.l = (i + self.h) mod 2`; the secure model's is `self.l = i`.

## Test Procedure

Run each vector through the real local Rust boundary with the explicit-state engine and replay every counterexample with no engine present. Each refutation is checked by replaying its counterexample, never by comparing it with one expected counterexample; the verdict, basis, cause and form are compared exactly.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| FE-01 | `Opaque` on the leaky model | `refuted`, `decisive-counterexample`; `witness-exhausted` with `position` 1 and one trace for `a`; replay recomputes `X_0` as a singleton and `X_1` empty |
| FE-02 | `Opaque` on the secure model | `proved`, `closed-scope`; every witness set is a singleton; 4 reachable product states |
| FE-03 | `Opaque` on a model with 4 initial secrets and nondeterministic `l`, `max_witness_set` 2 | `incomplete`, cause `limit-reached` naming `max_witness_set` and its value 2 |
| FE-04 | FE-02 with no `max_witness_set` in the request | runs under the provider's published default; same verdict |
| FE-05 | form classification of `Opaque` | HP-3 |

## Expected Results

A `∀∃` refutation names universal traces only, and replay re-establishes the absence of an existential witness by recomputing the witness sets.
