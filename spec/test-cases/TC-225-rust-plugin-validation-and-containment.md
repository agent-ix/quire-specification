---
id: TC-225
title: "Rust plugin validation and containment"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-305
    type: verifies
---

# TC-225: Rust plugin validation and containment

## Description

Runs a compile-time provider and a process plugin through registration,
negotiation and result settlement. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| PL-01 | A compile-time provider and a process plugin, each advertising one capability kind; one source compiled with an empty registry and with both registered | Each receives only the items negotiation routes to it and returns results the typed FR-331 reader admits; the two checked packages are byte-equal | FR-305-AC-1 |
| PL-02 | A process plugin returning `proved` for its item | The record is `proved`, names the plugin's backend identity and carries the basis label `trusted`; the text rendering shows `trusted` beside the verdict | FR-305-AC-2 |
| PL-03 | A plugin returning malformed output, an oversized frame, a timeout and a crash, beside a second provider with its own items | Each affected item settles `failed` or `incomplete` with its typed cause and no verdict; the second provider's records are unchanged | FR-305-AC-3 |
| PL-04 | A plugin counterexample that replay reproduces, and one it does not | The first settles `refuted`; the second settles `inconclusive` | FR-305-AC-4 |

## Expected Results

A plugin reaches only its routed items, every result passes the typed reader,
its proofs are labelled `trusted`, and its counterexamples count only after
replay.
