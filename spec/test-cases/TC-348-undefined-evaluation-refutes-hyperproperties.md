---
id: TC-348
title: "Undefined evaluation refutes a hyperproperty"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-399
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-400
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-401
    type: verifies
---

# TC-348: Undefined evaluation refutes a hyperproperty

## Description

A hyper clause whose body or match, or a guard, `terminal when` predicate or
state constraint of a component's subject,
evaluates undefined at a reachable joint step settles `refuted` with cause `undefined-evaluation`, and its `undefined`
counterexample replays to the same undefined value.

## Test Procedure

Use the secure vault of TC-338 and its clause HB-01 (TC-337), on which HB-01
is proved. `rw` is a `Reference<V::Vault>` parameter valued `w`, which is not
in `vaults = {v}`. Run each vector through the real local Rust boundary with
the explicit-state engine, unreduced, and replay every counterexample with no
engine present. The verdict, basis, cause and counterexample kind are compared
exactly; a counterexample is checked by replaying it.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| UE-01 | HB-01 with body `always holds((lookup<V::Vault>(vaults, rw) absent undefined).l @ a = v.l @ b)` | `refuted`, `decisive-counterexample`, cause `undefined-evaluation`: `where` names (`a`, `b`) over alias `V`, joint position 0, both components' state keys and the locus of the `lookup`; `cause` is `absent-key`. The counterexample has kind `undefined`, two traces of no steps and `trace_position` 0; replay reproduces the undefined value with that locus and reason |
| UE-02 | HB-01 with body `always holds(v.l @ a = 1 implies (lookup<V::Vault>(vaults, rw) absent undefined).l @ b = 0)` | the read is reached only once `l_a = 1`, at joint position 1: `refuted`, `decisive-counterexample`, cause `undefined-evaluation` naming joint position 1, both components' state keys there, the `lookup`'s locus and `absent-key`. The `undefined` counterexample has two traces of one step each, `step(1)` for both, carries that value, and replays to it |
| UE-03 | UE-01's counterexample with `kind` set to `lockstep` and a `loop_entry` of 0 | the body evaluates undefined, not false: `inconclusive`, `replay-parity` |
| UE-04 | UE-01's counterexample serialized and read back | byte-identical under RFC 8785 JCS with exactly the members of kind `undefined` |
| UE-05 | HB-01 with body `always holds(v.h @ a = v.h @ b) and always holds(v.l @ a = 1 implies (lookup<V::Vault>(vaults, rw) absent undefined).l @ b = 0)` | the violation of `v.h @ a = v.h @ b` at joint position 0, on the initial tuple with unequal secrets, precedes the undefined read reached at joint position 1 in canonical breadth-first order over the product: `refuted` with a `lockstep` counterexample, not `undefined-evaluation` |
| UE-06 | UE-01's counterexample with its carried `cause` changed to `precondition-false`, and then with one `state_keys` entry changed | `inconclusive`, `replay-parity` in both cases |
| UE-07 | UE-01's counterexample with its `undefined` member removed | refused `invalid_runtime_input`/`invalid-value` before replay |
| UE-08 | HB-01 unchanged, on the secure vault whose `step` carries the guard `(lookup<V::Vault>(vaults, rw) absent undefined).l = 0` | the guard is undefined at `a`'s initial state, never read as disabled: `refuted`, `decisive-counterexample`, cause `undefined-evaluation` whose `where` names `a` over alias `V` alone, joint position 0, `a`'s state key there, the guard's `lookup` locus and the FR-181 transition identity of the first `step` instance tested there; `cause` is `absent-key`. The counterexample has kind `undefined`, two traces of no steps and `trace_position` 0; replay evaluates that transition's guard in `a` to the same undefined value |
| UE-09 | UE-08's counterexample with `variables` and `state_keys` naming both components while `transition` is present | refused `invalid_runtime_input`/`invalid-value` before replay |
| UE-10 | HB-01 unchanged, on the secure vault whose `step` carries the guard `self.l = 0` and whose `terminal when` predicate is `(lookup<V::Vault>(vaults, rw) absent undefined).l = 0` | at joint position 1 both components, after `step(1)`, have `l = 1` and no enabled transition, so the `terminal when` predicate is evaluated and is undefined, never read as non-terminal: `refuted`, `decisive-counterexample`, cause `undefined-evaluation` whose `where` names `a` over alias `V` alone, joint position 1, `a`'s state key there and the predicate's `lookup` locus, with no `transition`; `cause` is `absent-key`. The counterexample has kind `undefined`, two traces of one step `step(1)` each and `trace_position` 1; replay evaluates the `terminal when` predicate in `a` to the same undefined value |
| UE-11 | HB-01 unchanged, on the secure vault under the state constraint `(lookup<V::Vault>(vaults, rw) absent undefined).l = 0` | the constraint is undefined at `a`'s initial state: `refuted`, `decisive-counterexample`, cause `undefined-evaluation` whose `where` names `a` over alias `V` alone, joint position 0, `a`'s state key there and the constraint's `lookup` locus, with no `transition`; `cause` is `absent-key`. The counterexample has kind `undefined`, two traces of no steps and `trace_position` 0; replay evaluates the state constraint in `a` to the same undefined value |
| UE-12 | TC-339 `Opaque` on the secure vault with the match conjunct `(if present(a.step.Vault::step.i) then 1 / value(a.step.Vault::step.i) else 1) = 1` on the universal variable `a`; then the same with a second universal variable `c` before `b` | the engine finds an `undefined` counterexample, cause `undefined-evaluation` `division-by-zero`, at joint position 1: its trace for `a` takes `step(0)`, its trace for `b` is the first witness-set member's path with no step, one step short of `a`'s, `trace_position` is 1 and the `state_keys` entry for `b` is the key of its initial state; `variables` and `state_keys` hold two entries, and three (`a`, `c`, `b`) with `c`. The paths differ in step count, so replay refuses it (TC-345 HR-12) and no result is settled |
| UE-13 | TC-344 PA-02's `align skip` clause with UE-12's match conjunct and both runs starting mid-operation | the engine finds an `undefined` counterexample, cause `undefined-evaluation` `division-by-zero`: its unmarked paths each hold a skipped `mix` and the visible `step(0)`, and `trace_position` and the `where` position are 1, the visible steps, not the moves; replay refuses it (TC-345 HR-12) and no result is settled, since a refutation settles only after its counterexample replays |

## Expected Results

An undefined body, match, or subject guard, `terminal when` predicate or
state constraint refutes the clause the secure vault
otherwise proves, the refutation carries `where` and `cause`, and it settles
only after its `undefined` counterexample replays to the same undefined value
(FR-399-AC-8, FR-399-AC-10, FR-400-AC-1, FR-400-AC-4, FR-400-AC-5, FR-401-AC-8, FR-401-AC-10).
