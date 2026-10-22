---
id: TC-952
title: "Obligation identity digest domain"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-201
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-201-AC-10
    type: verifies
---

# TC-952: Obligation identity digest domain

## Description

Compute CG obligation identities in the FR-201
`quire.ir-proof.obligation-identity/v1` domain and show which members enter the
preimage and which stay outside it. Scope: FR-201-AC-10.

## Fixture

`O1` is the obligation of one clause of a checked package: the clause node id,
its `claim` occurrence key (`node_id`, `claim`, 0), one obligation kind and one
argument, a parameter node id with the declared domain `Int[0, 9]`, carried as
a `KaniObligationIdentity` with a `source_span`.

## Test Procedure

Build each vector's `KaniObligationIdentity` through CG's real
`kani_obligations` boundary, compute its obligation identity and compare it
with `O1`'s.

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| OI-01 | `O1` with its `source_span` moved to another region of the same source | Equal digest | FR-201-AC-10 |
| OI-02 | `O1` with its `source_span` in another source document whose package compiles to the same node ids | Equal digest | FR-201-AC-10 |
| OI-03 | `O1` with another clause's node id | Different digest | FR-201-AC-10 |
| OI-04 | `O1` with occurrence `ordinal` 1, the second occurrence of a structurally identical clause | Different digest | FR-201-AC-10 |
| OI-05 | `O1` with another obligation kind | Different digest | FR-201-AC-10 |
| OI-06 | `O1` with another parameter node id; then with the declared domain `Int[0, 8]` | Each a different digest | FR-201-AC-10 |
| OI-07 | `O1`'s digest bytes labelled `quire.checked-semantic-node/v1`, then `sha256-jcs` | Each compares unequal to `O1`'s identity | FR-201-AC-10 |

## Expected Results

Each vector gives exactly its expected result: `source_span` and source regions
never change the identity, every content member does, and the domain label
keeps an equal-bytes digest of another domain unequal.
