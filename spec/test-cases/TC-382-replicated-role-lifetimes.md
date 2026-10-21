---
id: TC-382
title: "Replicated role spawn, action and lifetimes"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-428
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
| RR-08 | `role w each M::Worker from workers max 1 lifetime until(w.done = 1)` over the `Worker` model with a `done` field; then the same with `until({w.done = 1})` and with `until(z.done = 1)` | the first is admitted and `w` is one instance of type `Reference<M::Worker>`; the brace form refuses `invalid_syntax`/`unexpected-token` at the `{` and the `z` form `missing_declaration`/`missing-name` at `z` (FR-428-AC-8) |

## Expected Results

The live-instance count never exceeds `max n`; each lifetime retires at the
stated step; no retired object is spawned again in its protocol instance; the
`roles` entry is a map from object reference to status, independent of spawn
order.
