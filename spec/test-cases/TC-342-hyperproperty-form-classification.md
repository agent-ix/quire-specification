---
id: TC-342
title: "Hyperproperty form classification, unsupported forms and single-existential claims"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-396
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-399
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-394
    type: verifies
---

# TC-342: Hyperproperty form classification, unsupported forms and single-existential claims

## Description

Classify clauses into HP-1 to HP-6, settle HP-4 unsupported without running an engine, and route single-existential clauses to the possibility family.

## Test Procedure

Admit each vector through the real local Rust boundary and record its form, its requested items and kinds, and its result; replay every witness and trap. The vault subject: model `V` with object type `Vault`, fields `h: Int[0, 1]` (secret) and `l: Int[0, 1]` (public output), population `vaults` with universe `{v}`, one operation `step(i: Int[0, 1])` with frame `modifies [l]` and no precondition, and initial snapshots `(h, l) = (0, 0)` and `(1, 0)`. The leaky model's post clause is `self.l = (i + self.h) mod 2`; the secure model's is `self.l = i`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| HF-01 | `exists trace b of V forall trace a of V` with a safety body | HP-4; `unsupported`, `unsupported_projection`/`unsupported-requested-capability`; no engine runs |
| HF-02 | `forall trace a of V exists trace b of V` with body `always eventually holds(v.l @ a = v.l @ b)` | HP-4; `unsupported` |
| HF-03 | `Opaque` with `b` carrying `fair weak V::Vault::step;` | HP-4; `unsupported` |
| HF-04 | `Opaque` with `align skip { V::Vault::step }` | HP-4; `unsupported` |
| HF-05 | `hyper E using inf over behaviours (v: V::Vault) exists trace b of V { eventually holds(v.l @ b = 1) }` on the secure model | HP-5; `proved`, `decisive-witness`, with a lasso witness per initial state carrying `loop_entry`; replay checks fairness and evaluates the body true |
| HF-06 | `hyper E2 using inf over behaviours (v: V::Vault) exists trace b of V { always holds(v.l @ b = 1) }` on a model where `l` starts at 0 and `step` is disabled | HP-5; `refuted`, `closed-scope`, with a trap at the initial state and an empty stem; replay re-explores the product part |
| HF-07 | HF-05 with a `match` block | `unsupported_construct`/`expression-form` at the block |
| HF-08 | HB-01, `Opaque`, HB-11 and an `align skip` universal safety clause | HP-2, HP-3, HP-1 and HP-6, each item of kind `temporal-satisfaction` with its form recorded |
| HF-09 | HB-01 against a single registered candidate whose arm discharges HP-1 only | `unsupported`, warned, `unsupported_projection`/`unsupported-requested-capability` naming the HP-2 form |
| HF-10 | HF-05 with `max_states` set so the product exploration stops after a witness lasso was found for every initial state | `inconclusive`, cause `well-definedness-unchecked`, with the witnesses; never `proved` |
| HF-11 | HF-05 with body `eventually holds((lookup<V::Vault>(vaults, rw) absent undefined).l @ b = 1)`, `rw` valued a key not in `vaults` | `refuted`, `decisive-counterexample`, cause `undefined-evaluation` with `absent-key`, although a witness lasso exists for the defined part |
| HF-12 | HF-05's witness with its lasso made unfair under a `fair weak V::Vault::step;` constraint on `b`, and HF-06's trap replayed on a model where `step` is enabled | `inconclusive`, `replay-parity` in both cases |

## Expected Results

Each clause has exactly one form, HP-4 never reaches an engine, and a single-existential claim is settled by the possibility family's rows.
