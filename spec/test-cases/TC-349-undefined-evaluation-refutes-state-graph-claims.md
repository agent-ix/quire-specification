---
id: TC-349
title: "Undefined evaluation refutes a state-graph claim"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-391
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-393
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-394
    type: verifies
---

# TC-349: Undefined evaluation refutes a state-graph claim

## Description

A state-graph claim whose predicate, or a guard, `terminal when` predicate or
state constraint of whose subject, evaluates undefined at an explored node
settles `refuted` with cause `undefined-evaluation`, from `undefined`
evidence that replays to the same undefined value.

## Test Procedure

Use the ConfigVersion subject of TC-331. `rz` is a
`Reference<Config::ConfigVersion>` parameter valued `z`, which is not in
`config_history = {a, b}`. Run each vector through the real local Rust
boundary with the explicit-state engine, then replay every evidence value with
no engine present. The verdict, basis, cause and evidence arm are compared
exactly; evidence is checked by replaying it.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| SG-01 | `possible ReachesZ using v over (c: Config::ConfigVersion) { (lookup<Config::ConfigVersion>(config_history, rz) absent undefined).versionNumber = 2 }`, `witness_samples` 0 | each instance `refuted`, `decisive-counterexample`, cause `undefined-evaluation`: `where` names `P`, the state key of `(0, 0)` and the `lookup`'s locus; `cause` is `absent-key`. Evidence is the `undefined` arm with an empty stem at `(0, 0)`; replay evaluates `P` there to the undefined value with that locus and reason |
| SG-02 | `always possible` with `Q` `c.versionNumber = 1` and SG-01's `P` | each instance `refuted`, `decisive-counterexample`, cause `undefined-evaluation` naming `P`; an `undefined` stem that replays |
| SG-03 | the subject with an added operation `retire` that removes `b` from `config_history`, and SG-01's claim reading `rb`, valued `b`, in place of `rz` | each instance `refuted`, `decisive-counterexample`, cause `undefined-evaluation` naming the node after `retire(b)`; an `undefined` stem of the one step `retire(b)` that replays |
| SG-04 | SG-03's evidence with an empty stem | `P` is defined at `(0, 0)`: `inconclusive`, `replay-parity` |
| SG-05 | SG-01's evidence serialized and read back | byte-identical under RFC 8785 JCS with exactly the members of the `undefined` arm |
| SG-06 | an `always possible` claim on a model whose initial state reaches, at depth 1, a trap node where `Q` holds, and at depth 3 a node where `P` reads `lookup` of an absent key | `refuted`, `closed-scope`, with the `trap`: the trap node comes first in discovery order |
| SG-07 | SG-06 with the undefined node at depth 1, discovered before the trap node at depth 2 | `refuted`, `decisive-counterexample`, cause `undefined-evaluation`, with the `undefined` arm |
| SG-08 | SG-03's claim with `witness_samples` 64, each of which reaches `P` before any `retire` step | `refuted`, cause `undefined-evaluation` naming the node after `retire(b)`: the same verdict as SG-03 with `witness_samples` 0 |
| SG-09 | the subject with an added operation `audit(c)` whose guard is `(lookup<Config::ConfigVersion>(config_history, rz) absent undefined).versionNumber = 0`, and TC-331 PV-01's claim, `witness_samples` 0 | each instance `refuted`, `decisive-counterexample`, cause `undefined-evaluation`: `where` carries the state key of `(0, 0)`, the guard's `lookup` locus and the FR-181 transition identity of `audit(a)`, and no `predicate`; `cause` is `absent-key`. Evidence is the `undefined` arm with an empty stem; replay evaluates that transition's guard at `(0, 0)` to the same undefined value |
| SG-10 | SG-09 with `witness_samples` 64 and a witness for every initial state | the same verdict and evidence as SG-09: the witnesses do not settle the item |
| SG-11 | TC-332's Game subject with a `terminal when` predicate that is defined at `Won` and reads `lookup` of an absent key at `Lost`, and `possible` with `P` the phase is `Won` | `refuted`, `decisive-counterexample`, cause `undefined-evaluation`: `where` carries the state key of `Lost` and the `terminal when` predicate's locus, with neither `predicate` nor `transition`; an `undefined` stem `play`, `lose` that replays |
| SG-12 | the subject under a state constraint `(lookup<Config::ConfigVersion>(config_history, rz) absent undefined).versionNumber < 3`, and TC-331 PV-01's claim | each instance `refuted`, `decisive-counterexample`, cause `undefined-evaluation`: `where` carries the state key of `(0, 0)` and the constraint's locus, with neither `predicate` nor `transition`; an empty stem that replays |
| SG-13 | SG-09's evidence serialized and read back; then with `"predicate":"P"` added to its `where`; then with its `transition` naming a transition of no operation of the subject | byte-identical round trip; each altered value refused `invalid_runtime_input`/`invalid-value` before replay |
| SG-14 | SG-09's evidence with its `transition` changed to that of `upd(a)`, whose guard is defined at `(0, 0)` | `inconclusive`, `replay-parity` |

## Expected Results

An undefined predicate, guard, `terminal when` predicate or state constraint
refutes the claim, ordered against other refuting
evidence by discovery order and whatever witnesses exist, and the refutation settles only after its `undefined` evidence replays to the
same undefined value (FR-391-AC-10, FR-391-AC-11, FR-391-AC-12, FR-391-AC-14, FR-393-AC-1, FR-393-AC-5, FR-394-AC-7).
