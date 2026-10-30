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
| `identity` | The item's obligation identity, the structural `{node_id, model, ordinal}` (FR-479) binding the proof to the exact subject and claim. |
| `scale` | The common denominator the constants were scaled by. |
| `bounds` | The per-clock lower and upper bounds `L` and `U`, per discrete state, used by the simulation test. |
| `nodes` | A finite list of nodes `(s, q, Z)`: discrete state key, claim-automaton state, canonical DBM. |
| `edges` | For each node and each transition identity enabled from it, the index of a target node. |
| `initials` | Nonempty. The position in `nodes` of the initial node of each initial state of the subject, in the subject's initial-state order (FR-361): the node that covers that state's initial symbolic state. A form with one initial state lists one position. QSL FR-244 reads its single `initial` as `initials[0]`; the wire carries no `initial` member. |
| `admitted` | The ADR-026 TD-6 witness: a start node (in a refinement form, any positioned `initials[i]`, so a time-locked first initial state does not decide it) and a path of certificate edges to a quiescent node or to a node of a component with a cycle of positive total delay (QSL FR-244 `admitted`). |
| `refinement` | Present only for the safety half of a timed refinement (QSL FR-430): the refinement part below. |

QSL FR-244 names `identity`, `bounds` and a node's `automaton` as `obligation`,
`lu_bounds` and `automaton_state`; the wire spellings here are the ones the
FR-331 schema carries and QSL FR-245 writes in its loci. A node is marked
`locked` (QSL FR-244 "Locked nodes") by a `locked` member present, as `true`,
on exactly that node.

It states three facts: the initial symbolic state (the zero valuation, let
time elapse unless the initial state is urgent, intersected with its time
invariants) is covered by a node of the initial discrete state; each node's
successor by each identity is covered by its named target under the aLU
simulation for `bounds`; and no node is bad unless it is marked `locked` (a rejecting automaton state, or
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

### Refinement part

For the safety half of a refinement whose concrete model is timed, the
certificate is the same value with `refinement` present, the reachability part
read over the refinement product (QSL FR-244 "Refinement form"):

| Member | Content |
| --- | --- |
| `refinement_node` | The refinement node's `node_id` (FR-322). The part itself is `refinement` and an `edges` entry's `node` is an index, so no name stands for two things. |
| `concrete_package`, `abstract_package` | The two packages' identities (FR-322 package `name`). |
| `positions` | Nonempty (an admitted refinement half starts at a positioned initial node, so it certifies at least one position). The certified positions: each `{concrete, history, abstract_states}`, a concrete `StateKey`, the FR-378 history values as FR-181 typed values in history-row order, and the sorted abstract `StateKey`s. |
| `taken` | One entry per element of `edges`, in order: `null` for an edge into or inside the locked region (QSL FR-244), else `{type: "stutter"}` or `{type: "abstract", transitions}` with FR-181 transition identities. |

A node's `automaton` is then an index into `positions`; the locked region's
nodes carry the reserved index `len(positions)`, which names no position, and
the initial node of a time-locked concrete initial state whose `check_initial`
fails is such a `locked` node (QSL FR-245-AC-9).

### Canonical bytes

The bytes of a zone certificate are the RFC 8785 (JCS) serialization, UTF-8,
through ADR-013 §2's one encoder, of the FR-331 `ZoneCertificate` value, with
no byte before or after it. Equal certificates give equal bytes (QSL FR-244-AC-3
and AC-5), and the encoding is a function of the value alone.

- **Member order.** Object members are in ascending UTF-16 code-unit order of
  their names, at every depth: for the certificate, `admitted`, `bounds`,
  `components`, `divergence`, `edges`, `identity`, `initials`, `nodes`,
  `reasons`, `refinement`, `scale`. Array elements keep the order the
  certificate states: `nodes` in discovery order, `edges` in canonical
  transition order, `bounds` by the first node of its state then by clock
  index, `components`, `divergence` and `zone` rows by node or clock index.
- **Number encoding.** An index, a count or a position (`node` and `target` of an `edges` entry,
  `automaton`, `components` entries, `divergence` entries, `admitted`
  entries, `initials`) is a JSON integer from 0 to 2^53 − 1. A scaled constant
  (`scale`, every `bound`) is an integer string `0` or `-?[1-9][0-9]*`,
  arbitrary precision, with no `+`, no leading zero and no `-0`; `strict` is a
  boolean. The one string spelling of an unbounded entry, and of an absent LU
  bound, is `"infinity"`. A DBM is its rows of entries, entry `[i][j]`
  bounding `x_i − x_j`, index 0 the reference clock. No number is a fraction
  or has an exponent.
- **Option encoding.** An absent optional member (`components`, `reasons`,
  `divergence`, `refinement`, `locked`) is omitted, never `null`; `locked` is
  present only as `true`. In an array a position is significant, so the one
  option inside an array, a `taken` entry that is `None`, is `null`.
- **No embedded digests.** The certificate embeds no digest or hash of the
  certificate itself, of a node or zone, of the checker, the producer or a
  manifest, or of any run output, and has no checksum member. What it carries
  that names the checker's inputs is structural: `identity` (the obligation
  identity), each `StateKey` (the discrete state's own identity, FR-181) and,
  in a refinement form, `refinement_node` (a `node_id`), `concrete_package`
  and `abstract_package` (package identities). The checker compares nothing
  against them; it checks the certificate's rules against the packages it is
  given.
- **Vectors.** The worked certificates ZC-10 to ZC-12, with their bytes, are
  published in `proposals/backend-provider-v1/zone-certificate-vectors.json`
  (TC-371). Their state keys are fixed illustrative values, not
  computed from a model.

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
at}` (FR-331): `initial-missing` at the State locus `{key}` of
the initial discrete state, when no node of it covers the initial symbolic state (in a refinement form, of the discrete state of concrete initial state `i`, when `initials[i]` does not cover that state's symbolic state, for each `i`); `successor-missing` at
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
- **Byte form** — the canonical bytes above; two certificates are equal exactly
  when their bytes are equal.
- **Absence** — a missing member rejects the certificate,
  `certificate-rejected{rule, at}` with rule `malformed` at `{member}`, that
  member's name.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-418-AC-1 | The checker accepts the certificate of the `Rpc` `NoLateReply` proof at `T = 4 ms` by recomputing every successor and coverage. | Test (TC-371) |
| FR-418-AC-2 | A certificate with a node whose successor is not covered by its named target settles `inconclusive`, `certificate-rejected` with rule `successor-missing` at that node; a bad node, `bad-state` at that node; an edge that decreases the component index, `backward-edge` at that edge; a component with no valid reason, `witness-fails` at that component; a Büchi certificate without `reasons`, `malformed` at member `reasons`; a certificate in which no node covers the initial symbolic state, `initial-missing` at the State record of the initial discrete state; and a time-lock-freedom certificate whose node names a target that is not a reachable time-divergent target, `witness-fails` at that node. | Test (TC-371) |
| FR-418-AC-3 | A certificate whose identity differs from the item's is refused `invalid_runtime_input`/`conflicting-identity`. | Test (TC-371) |
| FR-418-AC-4 | Constants are scaled to integers by the least common multiple of their denominators and DBM bounds are arbitrary-precision integers, so a constant of any size checks exactly. | Test (TC-371) |
| FR-418-AC-5 | A time-lock-freedom certificate states a reachable time-divergent target for every node, and the checker verifies each. | Test (TC-371) |
| FR-418-AC-6 | The bytes of a zone certificate are the RFC 8785 UTF-8 serialization of its `ZoneCertificate` value with members in ascending code-unit order and arrays in the stated order, and the bytes of ZC-10, ZC-11 and ZC-12 equal the published bytes, byte for byte, across two runs. | Test (TC-371) |
| FR-418-AC-7 | An index is a JSON integer in 0 to 2^53 − 1 and a scale or a bound an integer string; a bound written as a JSON number, with a leading zero, a `+` or `-0`, or a scale `0`, is not a `ZoneCertificate`, and a bound of 27 digits is carried exactly. | Test (TC-371) |
| FR-418-AC-8 | An absent optional member is omitted: `components`, `reasons`, `divergence` or `refinement` as `null` is not a `ZoneCertificate`, `locked` is present only as `true`, and a `taken` entry that is absent is `null`. | Test (TC-371) |
| FR-418-AC-9 | `identity` is the structural obligation identity, and in a refinement form the part's `refinement_node` is a `node_id` and `concrete_package` and `abstract_package` are package identities; no member carries a digest, hash or checksum of the certificate or of another artifact. | Test (TC-371) |
| FR-418-AC-10 | Every certificate lists `initials` (nonempty), and a refinement-form certificate lists, in its part, `positions` (nonempty) and `taken`, so a refinement part with empty `positions` is not a `ZoneCertificate`; ZC-12's first initial node (`initials[0]`) is a `locked` node with `automaton` equal to the number of positions and its `admitted` starts at the positioned `initials[1]`, a refinement form whose `initials[i]` does not cover concrete initial state `i` is `initial-missing` at that state's State record (the locus is the discrete state; two initial states sharing one discrete state share the locus, and the checker reports the lowest failing index first), `admitted` may start at any positioned `initials[i]`, and a certificate without `initials` or `admitted`, or with an `initial` member, is not a `ZoneCertificate`; nor is a refinement part whose node-id member is named `refinement` or `node` rather than `refinement_node`. | Test (TC-371); the checker clauses by QSL TC-700 and TC-371 ZC-14 |

## Dependencies

- [FR-416](../../functional/temporal/FR-416-evaluate-timed-temporal-claims.md)
  and [FR-417](../../functional/temporal/FR-417-settle-time-lock-freedom.md)
  settle the proofs this certificate carries.
- [FR-331](FR-331-backend-provider-envelope.md) carries the certificate in
  the result.

## References

- QSL ADR-026 EZ-2 to EZ-4, CF-1 to CF-6, OV-12.
- QSL FR-244 (the certificate's members), FR-245 (its checker) and FR-430 (the refinement form); Linear STD-522 and QSL STD-516.
- S. Wimmer and J. von Mutius, TACAS 2020; S. Wimmer, F. Herbreteau and
  J. van de Pol, FORMATS 2020; F. Herbreteau, B. Srivathsan and
  I. Walukiewicz, LICS 2012.
- Owner ruling recorded on QSL-373: a native zone engine with zone
  certificates for checkable proofs.
- Ticket: STD-139 (paired with QSL-373).
