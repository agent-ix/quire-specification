---
id: TC-382
title: "Replicated role spawn, action and lifetimes"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-428
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-426
    type: verifies
---

# TC-382: Replicated role spawn, action and lifetimes

## Description

Check `spawn`, action `by` a replicated role, the three lifetimes, the rule
that a retired object is never spawned again, the authored `max`, and the
order-independent `roles` key.

## Test Procedure

Model: object type `Worker`, population `workers`, universe `{w1, w2}`; object
type `Job` with `done: Int[0, 2]`, universe `{j}`, initial `j.done = 0`;
operation `work(j: Job)` on `Worker`, postcondition `j.done = pre(j.done) +
1`, frame `modifies j.done`, precondition `j.done < 2`. Build a protocol with
role `r each M::Worker from workers max n lifetime L` and a `repeat` of
`attempt Work by r on M::Worker::work …` with the lifetime and `max` each
vector gives. Explore exhaustively and record each state's `roles` entry and
every `spawn`, `retire` and `by r` step.

## Vectors

| Vector | Role declaration | Expected result |
| --- | --- | --- |
| RR-01 | `max 1 lifetime workflow` | At most one live instance at any state; behaviours spawn `w1` or `w2`, and the spawned instance takes every `Work` step; it retires in the instance's `finish` step |
| RR-02 | `max 1 lifetime until(j.done = 1)` | After the first `Work`, `retire` of the live instance is enabled; after it, `spawn` of the other object is enabled and `spawn` of the retired object never is; the retired object stays in `roles` as `"retired"` |
| RR-03 | `max 2 lifetime scope`, with `Work` inside `sequence S` and a later node outside `S` | Both instances retire in the step after which no live thread can reach `Work`, folded into that step, with no `retire` step |
| RR-04 | `max 2 lifetime workflow` | The state after `spawn(w1)` then `spawn(w2)` and the state after `spawn(w2)` then `spawn(w1)` have byte-identical `roles` entries and coalesce; their traces carry distinct spawn ordinals |
| RR-05 | `max 2 lifetime workflow`, both live | `Work` has two enabled steps, one per instance, each with its `actor` member naming the acting object |
| RR-06 | `max 1 lifetime until(p)` with `p` reading `lookup<T>(p, r) absent undefined` at a key the population does not hold | every item settles `refuted`, cause `undefined-evaluation`, with the prefix to the first state at which `p` is evaluated and `where` naming `p` |
| RR-07 | `max 1 lifetime scope`, `Work` as the only node inside a `repeat` followed by a node outside it; then `Work` nodes in two sibling `sequence`s of one `parallel`; then `Work` directly in the root `run` | The instance is not retired while the `repeat` can iterate and retires in the step that leaves it; the region of the second is the `parallel`; the region of the third is the root (FR-428-AC-9) |
| RR-08 | `role w each M::Worker from workers max 1 lifetime until(w.done)` where `M::Worker` declares a Boolean field `done` (the Worker of this test case's model gains it); then the same with `until({w.done})` and with `until(z.done)` | the first is admitted and `w` is one instance of type `Reference<M::Worker>`; the brace form refuses `invalid_syntax`/`unexpected-token` at the `{`, and the `z` form `missing_declaration`/`missing-name` at `z` (FR-428-AC-8) |
| RR-09 | RR-08's first role with `w` read as a value in a `choice` guard; then in a node constraint; then in a second replicated role's `until` | each refuses `missing_declaration`/`missing-name` at the name `w`; none is read as the acting instance (FR-428-AC-10) |
| RR-10 | `activation on each` with `max 1 lifetime until(p)` and two live protocol instances 0 and 1; take `spawn` of `w1` in each, then `retire` of `w1` in each | the two `spawn` identities differ in the `instance` member and so do the two `retire` identities; each identity holds `step`, `role`, `object` and `instance` and no `thread` (FR-426-AC-15) |
| RR-11 | RR-08's first role with `from workers`; then with `from absent`; then with `from jobs` where `jobs` is a population of another object type | the first is admitted; `absent` refuses `missing_declaration`/`missing-name` at the name; `jobs` refuses `ill_typed`/`type-mismatch` at the name, each retaining the role and the population name (FR-428-AC-11) |
| RR-12 | A channel `from r to c` and a channel `from c to r` with `r` replicated, `max 2 lifetime workflow`, both instances live; then `r`'s universe empty | `send` and `receive` each give two steps with the `actor` member naming the acting object; with the universe empty a thread at the `send` deadlocks `no-role-instance` naming `r` (FR-428-AC-12) |

## Expected Results

The live-instance count never exceeds `max n`; each lifetime retires at the
stated step; no retired object is spawned again in its protocol instance; the
`roles` entry is a map from object reference to status, independent of spawn
order.
