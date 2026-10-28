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
---

# TC-371: Zone certificate checker

## Description

Accept valid zone certificates and reject or refuse invalid ones, recomputing every successor in exact arithmetic.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. Mutations start from ZC-01's certificate. ZC-10 to ZC-13 check the certificate's wire and canonical bytes (FR-418 "Canonical bytes") against the `ZoneCertificate` definition of the FR-331 schema and the vectors in `proposals/backend-provider-v1/zone-certificate-vectors.json`; they run in `tests/zone_certificate_wire.rs`. Tag those tests `#[trace("TC-371", "FR-418-AC-n")]`.

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
| ZC-08 | The certificate of `NoLateReply` with `T = 4 ms`, with the node covering the initial symbolic state removed | `inconclusive`, `certificate-rejected` with rule `initial-missing` at `{domain, digest}`, the State record of the initial discrete state |
| ZC-09 | The `Rpc` time-lock-freedom certificate with one node's named target replaced by a node that is not a reachable time-divergent target | `certificate-rejected` with rule `witness-fails` at `{node}`, that node's position |
| ZC-10 | The refinement-form certificate of FR-430-AC-1's `TReq` safety half (two positions, two nodes, two edges) | Valid `ZoneCertificate`; its RFC 8785 bytes equal `ZC-10`'s published `bytes_hex` (2343 bytes) |
| ZC-11 | A time-lock-freedom certificate with a component index per node, two reasons (`missing-acceptance`, `weak-fairness`) and a divergence target per node | Valid; bytes equal `ZC-11`'s published `bytes_hex` (1432 bytes) |
| ZC-12 | A refinement form with two initial states, the first a `locked` node with the reserved index and `admitted` starting at the positioned second | Valid; bytes equal `ZC-12`'s published `bytes_hex` (1617 bytes); `taken` is `[]` |
| ZC-13 | ZC-10 with a zone entry or a bound as a bare JSON number, a bound as the JSON number `-1`, a bound `"01"`, `scale` `"0"`, `components` as `null`, `locked` `false`, `initials` removed or `initial` added | Each is not a `ZoneCertificate` |
| ZC-14 | ZC-12 with `initials[1]` replaced by a node whose zone does not cover concrete initial state 1's symbolic state | `certificate-rejected` with rule `initial-missing` at `{domain, digest}`, the State record of that initial state's discrete state (the QSL checker's vector, QSL TC-700 step 8) |

## Expected Results

Each certificate is accepted, rejected or refused exactly as stated.

A refinement part with an empty `positions`, an `initial` member, an `initials` member inside the part, or its node id under the name `refinement` or `node` (the part's member is `refinement_node`) is not a `ZoneCertificate` (FR-418-AC-10).
