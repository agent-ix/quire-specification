---
id: TC-371
title: "Zone certificate checker"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-418
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-416
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-417
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-419
    type: verifies
---

# TC-371: Zone certificate checker

## Description

Accept valid zone certificates and reject or refuse invalid ones, recomputing every successor in exact arithmetic.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. Mutations start from ZC-01's certificate. ZC-10 to ZC-13, ZC-19, ZC-23 and ZC-26 check the certificate's wire and canonical bytes (FR-418 "Canonical bytes") against the `ZoneCertificate` definition of the FR-331 schema and the vectors in `proposals/backend-provider-v1/zone-certificate-vectors.json`; they run in `tests/zone_certificate_wire.rs`. ZC-19 supplies the two-state finite periodic task cycle, ZC-23 supplies completed-job renaming while another task job stays live, and ZC-24..25 and ZC-28 supply schema-valid task-reader refusals. ZC-27 and ZC-29 are malformed no-release wire cases. Tag those tests `#[trace("TC-371", "FR-418-AC-n")]`.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Rpc`**, **`NoLateReply`** and **`Settles`** (from QSL ADR-026 §11). `time Rpc dense unit ms;`. One object `c: Call { phase: Phase, inflight: Boolean, late: Boolean, x: Clock }`, `Phase` one of `Idle`, `Waiting`, `Replied` and `TimedOut`; universe `{c}`; initially `phase = Idle`, `inflight` and `late` false, `x = 0`. `Call::send` has pre `phase = Idle` and post `phase = Waiting`, `inflight`, `not late` and `x = 0`. `Call::reply` has pre `inflight and x >= 1 ms` and post `not inflight`, `late = (pre(phase) = TimedOut)`, and `phase = Replied` when `pre(phase) = Waiting`, unchanged otherwise. `Call::timeout` has pre `phase = Waiting and x >= T` and post `phase = TimedOut`. `Call::reset` has pre `not inflight and (phase = Replied or phase = TimedOut)` and post `phase = Idle`. The time invariants are `x <= 3 ms` while `inflight` and `x <= T` while `phase = Waiting`. `T` is a time constant each vector fixes. `NoLateReply` is `temporal NoLateReply using inf over (k: Rpc::Call) clock "model-steps" on origin { always holds(not k.late) }`. `Settles` is `temporal Settles using timed over (k: Rpc::Call) clock "model-time" on origin { always (holds(k.phase = Waiting) implies eventually[0 ms, 3 ms] holds(k.phase != Waiting)) }`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| ZC-01 | The certificate of `NoLateReply` with `T = 4 ms` | Accepted; the item settles `proved`, `zone-certified` |
| ZC-02 | A node whose successor is not covered by its named target | `inconclusive`, `certificate-rejected` with rule `successor-missing` at `{node}`, that node's position |
| ZC-03 | A node in a rejecting automaton state | `certificate-rejected` with rule `bad-state` at that node |
| ZC-04 | A Büchi certificate with an edge to a smaller component index; a component with no valid reason; a Büchi certificate without `reasons` | `certificate-rejected` with rule `backward-edge` at `{edge}`, that edge's position; `witness-fails` at `{component}`; `malformed` at member `reasons` |
| ZC-05 | ZC-01 with another item's identity | Refused `invalid_runtime_input`/`conflicting-identity` |
| ZC-06 | A model with constant `rational(123456789123456789, 7) ms` | Constants scale by 7 and DBM bounds hold the exact integers |
| ZC-07 | The `Rpc` time-lock-freedom certificate | Every node states a reachable time-divergent target, and the checker verifies each |
| ZC-08 | The certificate of `NoLateReply` with `T = 4 ms`, with the node covering the initial symbolic state removed | `inconclusive`, `certificate-rejected` with rule `initial-missing` at `{key}`, the State record of the initial discrete state |
| ZC-09 | The `Rpc` time-lock-freedom certificate with one node's named target replaced by a node that is not a reachable time-divergent target | `certificate-rejected` with rule `witness-fails` at `{node}`, that node's position |
| ZC-10 | The refinement-form certificate of FR-430-AC-1's `TReq` safety half (two positions, two nodes, two edges) | Valid `ZoneCertificate`; its RFC 8785 bytes equal `ZC-10`'s published `bytes_hex` (2343 bytes) |
| ZC-11 | A time-lock-freedom certificate with a component index per node, two reasons (`missing-acceptance`, `weak-fairness`) and a divergence target per node | Valid; bytes equal `ZC-11`'s published `bytes_hex` (1432 bytes) |
| ZC-12 | A refinement form with two initial states, the first a `locked` node with the reserved index and `admitted` starting at the positioned second | Valid; bytes equal `ZC-12`'s published `bytes_hex` (1617 bytes); `taken` is `[]` |
| ZC-13 | ZC-10 with a zone entry or a bound as a bare JSON number, a bound as the JSON number `-1`, a bound `"01"`, `scale` `"0"`, `components` as `null`, `locked` `false`, `initials` removed or `initial` added | Each is not a `ZoneCertificate` |
| ZC-14 | ZC-12 with `initials[1]` replaced by a node whose zone does not cover concrete initial state 1's symbolic state | `certificate-rejected` with rule `initial-missing` at `{key}`, the State record of that initial state's discrete state (the QSL checker's vector, QSL TC-700 step 8) |
| ZC-15 | A fixed-work task product with one task type with `bcet = wcet = 1` and deadline 2, one live job slot, its service/deadline clocks, the exact `M_x` values, diagonal constraints and subtraction by 1; the certificate covers every reachable refined region | Accepted only when `task_product` matches the recompiled product and every edge is covered by the refined-region relation; aLU-only successor coverage is insufficient |
| ZC-16 | ZC-15 with, in turn, its queue bound, one clock ceiling, one diagonal constraint, or the subtraction guard/constant omitted or changed; one subtraction enabled below `c` or above `M_x`, or yielding a negative value | Each is rejected at the task product/certificate locus; the checker recomputes all values from the admitted task set and product |
| ZC-17 | ZC-15 with a reachable deadline-miss node having a verified positive-delay continuation; then a deadline-miss node with no such continuation and marked `locked`; then a task-product certificate containing a queue-cap overflow node | The first certificate is rejected as `bad-state`; the locked time-lock-only path does not refute the timed claim and remains separately reportable by FR-417; the queue-cap overflow node is not a legal task-product state and the checker refuses the certificate |
| ZC-18 | A fixed-work EDF task-product certificate bound by `identity` to its checked claim, for two equal-deadline jobs whose tasks declare no priorities and whose source declaration order is `b`, `a`; then the same certificate with the first dispatch edge reversed; then a task-product carrying an extra `policy: fixed-priority` member | The reader derives EDF from the bound claim, admits the certificate only when every recomputed successor follows absolute deadline and the FR-419 canonical-task-name/activation/occurrence tie rule (`a` before `b`), independent of source declaration order, and does not synthesize task priorities. The reversed edge is `certificate-rejected` for a missing or incorrect successor; the extra policy member is not a `TaskProductCertificate` |
| ZC-19 | A two-state fixed-work periodic cycle: an activation template creates rank 0/slot 0 in the target, a completion template names that live job in the source, and the task has period 2, demand 1 and deadline 2 | The finite certificate closes after one activation/completion cycle although each replayed traversal has the next exact activation occurrence; concrete replay reconstructs the exact occurrence, generation, jitter, deadline and service values for every step |
| ZC-20 | ZC-19 with its certificate edge replaced by a complete exact `TaskTransitionIdentity` carrying `job` and `generation` | Not a `ZoneCertificate`; finite certificate edges do not carry exact global occurrence or generation |
| ZC-21 | ZC-19 with `job` added to the finite activation template | Not a `ZoneCertificate`; a finite template has a closed shape with no global job occurrence |
| ZC-22 | ZC-19 with its bounded-subtraction `transition` replaced by a complete exact `TaskTransitionIdentity` | Not a `ZoneCertificate`; bounded-subtraction rows use the same finite template identity as task-scheduler edges |
| ZC-23 | Task A completes while task B remains live, then A activates again using rank 0/slot 0 | The finite task certificate preserves B's identity and order across A's completion and reuse. Exact replay emits A's first occurrence and next occurrence with the appropriate slot generations while B remains unchanged |
| ZC-24 | ZC-23 with the activation template's `job_rank` set beyond the recomputed live-job range | `certificate-rejected`, `successor-missing` at node 1 |
| ZC-25 | ZC-23 with the completion template's `slot` changed to a slot not bound to its live job | `certificate-rejected`, `successor-missing` at node 0 |
| ZC-26 | A closed source product with no reachable checked task release and interval demand | Valid `ZoneCertificate`; the reader proves the no-release closure and does not use either demand endpoint |
| ZC-27 | ZC-26 with `live_job_cap` changed to 1 | Not a `ZoneCertificate` |
| ZC-28 | ZC-30 bound to `TC-372/CtlFp`, whose checked `Cell::set` source release is reachable | Schema-valid; `certificate-rejected`, `successor-missing` at node 0 |
| ZC-30 | A schema-valid no-release closure certificate with the exact `TC-372/CtlFp` obligation identity and `Ctl.a` demand row, used as ZC-28’s base | Wire-valid; the strict reader refuses it when it recomputes the reachable source release |
| ZC-29 | ZC-26 with `queue_bounds` empty | Not a `ZoneCertificate` |

## Expected Results

Each certificate is accepted, rejected or refused exactly as stated.

For ZC-19 and ZC-23 the reader recomputes all task successors modulo the
canonical live-job bijection. The periodic cycle does not compare concrete
occurrence or generation counters when deciding finite closure. Replay follows
each concrete source transition and instantiates the template with the exact
next occurrence and slot generation; ZC-23 proves that after completion it may
reuse that rank and slot without changing another live job's identity.
Changing the template's task, source operation, action, live-job rank or slot so
it no longer matches the recomputed successor refuses the certificate.

A refinement part with an empty `positions`, an `initial` member, an `initials` member inside the part, or its node id under the name `refinement` or `node` (the part's member is `refinement_node`) is not a `ZoneCertificate` (FR-418-AC-10).
