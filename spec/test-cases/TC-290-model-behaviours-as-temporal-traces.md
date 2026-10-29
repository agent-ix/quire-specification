---
id: TC-290
title: "Model behaviours read as temporal traces"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-361
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-090
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-181
    type: verifies
---

# TC-290: Model behaviours read as temporal traces

## Description

Conformance vectors for the model subject: positions and anchors, the clock
binding, the two admitted profiles, one terminated model read under both
profiles with different verdicts, the terminal stutter step, the `over`
binding, and the receiver in the transition identity.

## Test Procedure

The **Counter** subject: object type `Counter` with `value: Int[0, 3]`,
population `counters`, and operation `inc` with frame `modifies [value]`,
precondition `self.value < 3` and postcondition
`self.value = pre(self.value) + 1`. Universe `counters = {c}`, one initial
snapshot with `c.value = 0`, and `terminal Halt using v on Cnt when { … }`
whose predicate holds exactly at `value = 3`. Its one behaviour visits values
0, 1, 2, 3 and then has no enabled transition.

1. Read the behaviour and record each position's state and anchor.
2. Check `always[0,4] holds(c.value >= 0)`, activation `on origin`, under
   `quire.temporal.event-position.false-extension/v1` and under
   `quire.temporal.infinite-trace/v1`.
3. Check the same subject under `quire.temporal.fixed-sample.false-extension/v1`
   and `quire.temporal.timestamped-event.finite-window/v1`.
4. Under infinite-trace, extend the behaviour to a lasso and inspect its loop
   step.
5. Widen the universe to `{c, d}`, both at value 0, and check
   `always holds(x.value <= 3)` and `always holds(x.value <= 2)` with
   `over (x: Cnt::Counter)`; enumerate the transitions out of the initial
   state.
6. Add an operation `drop` that removes `d` from `counters` and check
   `always holds(x.value <= 3)` over `x = d` unguarded, then guarded by `d`'s
   population membership.

7. Starting from step 5's subject (`counters = [c, d]`, both at value 0, with `inc` enabled), add a population `all` of member type `Entity` (a supertype of `Counter`) with universe `[c, e]`, `e` an `Entity` that is not a `Counter`, and a population `specials` of member type `SpecialCounter` (a subtype of `Counter`) with universe `[s]`. With `over (x: Cnt::Counter)`, list the instances; check `always holds(x.value >= 0)` unguarded and guarded by the disjunction of `lookup<Counter>(p, x)` being `some` over `all`, `counters` and `specials`; check the unguarded claim `always holds(x.value <= 0)` with `all = [c, e]` and then with `all = [e, c]`, and read the reported instance and cause of each; with `inc` disabled and `c.value = 1`, `s.value = 1`, `d.value = 0`, check `always holds(x.value >= 1)` guarded by the disjunction over `all`, `counters` and `specials`, and guarded by `lookup<Counter>(all, x)` alone; check an item over `Cnt::Counter` against a subject whose only population is `all` with universe `[e]`; rename `all` so that it sorts after `counters` and read the binding of `c`; add to `counters` a member record for `c` typed `Counter` while `all` holds `c` as `Entity`; give `counters` a universe key `z` that no member record of any initial state admits (and, separately, a subject whose only population is `counters = [z]`); put the conflicting `c` records in two different initial states of a two-initial-state subject, and give `counters` no universe in a copy of that subject.
8. Build the universe provision of `counters` for the lists `[c, d]` and `[d, c]` from their keys.
9. Starting from step 5's subject with `inc` disabled (`counters = [c, d, z]`, `c` and `d` at value 0, no initial member record for `z`), add an operation `make` with frame `creates Counter` that creates a `Counter` with value 1 under `z`. With `over (x: Cnt::Counter)`, list the instances and the excluded keys, and check `always holds(x.value = 0)`.
10. Build a derived item (deadlock freedom) over a subject whose universes name populations of one domain package, then of two domain packages, then of none (an empty set of universes).

## Expected Results

1. Position 0 reads value 0 with anchor `initialization`; positions 1 to 3 read
   values 1 to 3, each anchored by its `inc` transition; `holds` atoms return
   the clause evaluator's values at those observations (FR-361-AC-1).
2. Under event-position false-extension the behaviour is a closed execution of
   four positions, position 4 reads every atom false, and the claim settles
   `refuted` with the four-position prefix; under infinite-trace position 4 is
   a terminal stutter position at value 3 and the claim settles `proved`,
   proof basis `exhaustive` (FR-361-AC-3). Both bind the clock role to the
   step sequence, one tick per step (FR-361-AC-2, FR-090-AC-8).
3. Both settle `unsupported`, `unsupported-requested-capability`, at
   negotiation (FR-361-AC-2, FR-090-AC-8).
4. The loop is one step with transition identity `{"type":"terminal-stutter"}`
   from value 3 to value 3; no operation is enabled at it and it takes no
   fairness constraint (FR-361-AC-4).
5. `always holds(x.value <= 3)` is checked once for `c` and once for `d` and
   settles `proved`; `always holds(x.value <= 2)` settles `refuted` naming the
   binding of its counterexample (FR-361-AC-5). The initial state has two
   transition identities for `inc` that differ only in `receiver`, ordered by
   their JCS bytes (FR-181-AC-7).
6. The unguarded read at a reachable position after `drop` settles `refuted`
   with cause `undefined-evaluation`, naming `d`, the position and the read's
   locus, as TC-305 step 7 does; the guarded claim settles `proved`
   (FR-361-AC-6); every use of `x` other than the guard after `drop` is `absent-key` and the wire reference of `d` still carries `Counter` (FR-361-AC-15).
7. The instances are `c`, `e`, `d`, `s` in first-occurrence order over population identity then universe order, `c` being one instance although `all` and `counters` both list it (FR-361-AC-9); the unguarded `>= 0` claim refutes with `undefined-evaluation` naming `e` and the claim guarded by the disjunction holds; the `>= 1` claim guarded by the disjunction is `refuted` at `d`, whose value is 0, while the `all`-only guard leaves the item `proved` (FR-361-AC-11); the `<= 0` claim is `refuted` under both lists, with cause none and instance `c` under `[c, e]`, and cause `undefined-evaluation` and instance `e` under `[e, c]`, the request identities differing (FR-361-AC-10); the binding of `c` has `object_type` `Counter` and equals the reference from `allInstances<Counter>(counters)`, unchanged when `all` is renamed to sort after `counters`, and `s` binds with `SpecialCounter` (FR-361-AC-12); an item over `Counter` against a subject whose only population is `all = [e]` has the instance `e` and is not `no-instance` (FR-361-AC-8); the cross-population conflict refuses `invalid_runtime_input`/`conflicting-identity` before any instance (FR-361-AC-14); `e` binds as (`universe`, `Entity`, `e`) with every unguarded use `absent-key`, and `z` is not an instance, and the subject whose only population is `counters = [z]` is `no-instance` (FR-361-AC-8, FR-361-AC-15); the conflict split across two initial states refuses `conflicting-identity` too, while the copy with no universe for `counters` returns `requires-bound` first (FR-361-AC-14).
8. The universe provisions `counters-c-d` and `counters-d-c` are the RFC 8785 documents and `digest` rows of the published vectors (BLOCKED-BY STD-606 (owner: planner titan:QSL:plan): universe provision `digest` row), differ from each other, and carry in each key exactly the members `universe` and `identity` (FR-361-AC-13).
9. The instances are `c` and `d`, and the excluded keys are exactly [`z`]; `make` is enabled and reaches a state holding `z`'s object at value 1, yet `always holds(x.value = 0)` is checked for `c` and `d` only and settles `proved`, and no counterexample or binding names `z` (FR-361-AC-15).
10. The first is anchored at that package's `model_root`; the second and the third each settle `unsupported`, `unsupported_projection`/`unsupported-requested-capability`, with no request item written (FR-361-AC-17).
