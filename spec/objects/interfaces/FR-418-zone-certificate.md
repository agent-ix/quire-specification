---
id: FR-418
title: "Zone certificate and its checker"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/FR-416
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-417
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
---

# FR-418: Zone certificate and its checker

## Description

When a zone-based provider proves a timed claim, the deadlock-freedom item or
the time-lock-freedom item over a timed subject, it SHALL emit a zone
certificate with exactly the members below, and the checker SHALL accept it,
recomputing every successor with its own difference-bound-matrix code in
exact arithmetic, before the item settles `proved` with basis `zone-certified`.

## Properties

The value is a reachability certificate, extended to a Büchi emptiness
certificate for liveness forms and the time-lock-freedom item, over zones.

### Zones

A zone is a conjunction of constraints `x_i − x_j ≺ c` over the subject's
clocks and a reference clock fixed at 0, `≺` one of `<` and `<=`, held as a
canonical difference-bound matrix (DBM). Every constant of the model and the
claim is scaled once to an integer by the least common multiple of the
denominators, so each DBM bound is an arbitrary-precision integer with its
strictness, or `∞`.

### Reachability certificate

For a safety form (TT-1, TT-2, TT-3, the deadlock-freedom item):

| Member | Content |
| --- | --- |
| `identity` | The item's obligation identity, the canonical identity digest (FR-201) binding the proof to the exact subject and claim. |
| `scale` | The common denominator the constants were scaled by. |
| `bounds` | The per-clock lower and upper bounds `L` and `U`, per discrete state, used by the simulation test. |
| `nodes` | A finite list of nodes `(s, q, Z)`: discrete state key, claim-automaton state, canonical DBM. |
| `edges` | For each node and each transition identity enabled from it, the index of a target node. |

It states three facts: the initial symbolic state (the zero valuation, let
time elapse unless the initial state is urgent, intersected with its time
invariants) is covered by a node of the initial discrete state; each node's
successor by each identity is covered by its named target under the aLU
simulation for `bounds`; and no node is bad (a rejecting automaton state, or
for the deadlock item a deadlocked valuation).

### Büchi emptiness certificate

For TT-4 and the time-lock-freedom item, the certificate adds:

- a component index per node, every edge going to a node of equal or greater
  index, so every cycle stays inside one component;
- per component, its reason: it lacks a node of some acceptance set (the
  time-divergence set included), or some weak fairness constraint is enabled
  at every valuation of every node in it and taken by no edge inside it, or,
  for a strong fairness constraint, a nested numbering of the component with
  the nodes that enable it removed;
- for the time-lock-freedom item, per node, a reachable time-divergent target:
  a quiescent node, or a component that does not fail for want of the
  divergence set.

## Checker

`check_zone_certificate` recompiles the package, recomputes every successor of
every node with its own DBM operations (canonical form by shortest-path
closure, emptiness as a negative cycle, guard intersection, reset, time
elapse, time-invariant intersection, splitting a disjunctive guard into its
convex parts) in exact arithmetic, checks every coverage by the aLU test,
checks initial coverage and the badness of every node, checks the component
numbering and each component's stated reason, and reads no output of the
search beyond the certificate. It refuses a certificate whose `identity`
differs from the item's with `invalid_runtime_input`/`conflicting-identity`.
A rejected certificate settles `inconclusive`, `certificate-rejected{rule,
at}` (FR-331): `initial-missing` at `{state}`, the initial discrete state,
when no node of it covers the initial symbolic state; `successor-missing` at
`{node}` when a node's successor by an identity is not covered by its named
target; `bad-state` at `{node}` for a bad node; `backward-edge` at `{edge}`
for an edge to a smaller component index; `witness-fails` at `{component}`
for a component whose reason does not hold, and at `{node}` for a node whose
named target is not a reachable time-divergent target. `node` and `edge` are
positions in `nodes` and `edges`, and `component` is a component index. A
checker stopped by a budget settles `incomplete`, `limit-reached` naming the
budget. Its cost is linear in the certificate's edges times the DBM operations.

## Identity and comparison

- **Identity kind** — *value*, compared componentwise; nodes and edges in
  their stated order.
- **Absence** — a missing member rejects the certificate,
  `certificate-rejected{rule, at}` with rule `malformed` at `{member}`, that
  member's name.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-418-AC-1 | The checker accepts the certificate of the `Rpc` `NoLateReply` proof at `T = 4 ms` by recomputing every successor and coverage. | Test (TC-371) |
| FR-418-AC-2 | A certificate with a node whose successor is not covered by its named target settles `inconclusive`, `certificate-rejected` with rule `successor-missing` at that node; a bad node, `bad-state` at that node; an edge that decreases the component index, `backward-edge` at that edge; a component with no valid reason, `witness-fails` at that component; and a Büchi certificate without `reasons`, `malformed` at member `reasons`. | Test (TC-371) |
| FR-418-AC-3 | A certificate whose identity differs from the item's is refused `invalid_runtime_input`/`conflicting-identity`. | Test (TC-371) |
| FR-418-AC-4 | Constants are scaled to integers by the least common multiple of their denominators and DBM bounds are arbitrary-precision integers, so a constant of any size checks exactly. | Test (TC-371) |
| FR-418-AC-5 | A time-lock-freedom certificate states a reachable time-divergent target for every node, and the checker verifies each. | Test (TC-371) |

## Dependencies

- [FR-416](../../functional/temporal/FR-416-evaluate-timed-temporal-claims.md)
  and [FR-417](../../functional/temporal/FR-417-settle-time-lock-freedom.md)
  settle the proofs this certificate carries.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the
  canonical identity digest.
- [FR-331](FR-331-backend-provider-envelope.md) carries the certificate in
  the result.

## References

- QSL ADR-026 EZ-2 to EZ-4, CF-1 to CF-6, OV-12.
- S. Wimmer and J. von Mutius, TACAS 2020; S. Wimmer, F. Herbreteau and
  J. van de Pol, FORMATS 2020; F. Herbreteau, B. Srivathsan and
  I. Walukiewicz, LICS 2012.
- Owner ruling recorded on QSL-373: a native zone engine with zone
  certificates for checkable proofs.
- Ticket: STD-139 (paired with QSL-373).
