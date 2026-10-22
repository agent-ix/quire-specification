---
id: TC-337
title: "Hyperproperty and step-relation syntax, binding and refusals"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-395
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-397
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-398
    type: verifies
---

# TC-337: Hyperproperty and step-relation syntax, binding and refusals

## Description

Parse and admit hyper clauses over `behaviours` and relation clauses over model operations, and refuse each construct the binding rules exclude.

## Test Procedure

Compile each source vector through the real local Rust boundary under the complete facets and record its parse tree, its instances, or its located refusal. The vault subject: model `V` with object type `Vault`, fields `h: Int[0, 1]` (secret) and `l: Int[0, 1]` (public output), population `vaults` with universe `{v}`, one operation `step(i: Int[0, 1])` with frame `modifies [l]` and no precondition, and initial snapshots `(h, l) = (0, 0)` and `(1, 0)`. The leaky model's post clause is `self.l = (i + self.h) mod 2`; the secure model's is `self.l = i`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| HB-01 | `hyper NonInterference using inf over behaviours (v: V::Vault) forall trace a of V forall trace b of V match { a.step.op = b.step.op and a.step.Vault::step.i = b.step.Vault::step.i } { always holds(v.l @ a = v.l @ b) }` | admitted; two variables over alias `V`; one instance, key `v` |
| HB-02 | a clause over aliases `S` and `N` with parameters `(d: S::Device, e: N::Device)` and universes of 2 and 3 keys, each key admitted by a member record of an initial state | admitted; 6 instances over the product of the admitted keys |
| HB-03 | HB-01 with `forall trace a` (no `of`) | `unsupported_construct`/`expression-form` at the quantifier |
| HB-04 | `hyper H using inf over t in Q bounded 4 forall trace a of V { true }` | `unsupported_construct`/`expression-form` at the quantifier |
| HB-05 | a body read `v.l` with no `@` | `ill_typed` at the read |
| HB-06 | a body read `d.used @ n` where `n` ranges over a model that does not declare `Device::used` of `d`'s model | `ill_typed` at the read |
| HB-07 | a match conjunct `v.l @ a = 0` | `unsupported_construct`/`expression-form` at the read |
| HB-08 | HB-01 under a finite-trace profile | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
| HB-09 | `align skip { V::Vault::mix, V::Vault::mix }`; `align skip { W::T::op }` with `W` unbound; `align skip { }` | each refused `unsupported_construct`/`expression-form` at the entry or list |
| HB-10 | an `align skip` clause whose variable carries `fair weak V::Vault::step;` | `unsupported_construct`/`expression-form` at the constraint |
| HB-11 | `relation Det using v over (x: V::Vault::step, y: V::Vault::step) { (pre(x.self.l) = pre(y.self.l) and x.i = y.i) implies x.self.l = y.self.l }` | admitted; two executions of `V::Vault::step` |
| HB-12 | a relation whose execution bindings name a declared trace population | FR-191's finite reading |
| HB-13 | HB-02 with one key of each universe admitted by no member record of any initial state | admitted; 2 × 2 = 4 instances over the product of the admitted keys; the two unadmitted keys are excluded keys of the item |

## Expected Results

Every admitted vector binds exactly its aliases, variables, instances and fairness sets, and every refused vector reports exactly its located refusal.
