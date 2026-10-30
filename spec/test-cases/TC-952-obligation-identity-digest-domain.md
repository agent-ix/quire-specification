---
id: TC-952
title: "Obligation identity is structural (retired name: obligation identity digest domain)"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-479
    type: verifies
---

# TC-952: Obligation identity is structural (retired name: obligation identity digest domain)

## Description

Show that an obligation identity is the structural triple (claim or obligation
`node_id`, owning model identity, ordinal) compared componentwise, so that no
member outside the triple changes it.

## Fixture

`O1`: an obligation of a clause with `node_id` `N`, owned by model `M`, ordinal
0, carried with every member of CG's obligation record, `source_span` and the
run configuration included.

## Test Procedure

Build each vector's obligation through CG's obligation identity boundary and
compare its identity with `O1`'s componentwise.

| Vector | Input | Expected result |
| --- | --- | --- |
| OI-01 | `O1` with its `source_span` moved, then in another source document whose package compiles to the same node ids | Equal identity |
| OI-02 | `O1` with another clause's `node_id` | Different identity |
| OI-03 | `O1` with ordinal 1, the second obligation of a structurally identical clause | Different identity |
| OI-04 | `O1` owned by another model | Different identity |
| OI-05 | `O1` with each of the resource ceilings, an embedded oracle, the harness symbol, the contract symbol, the subject path, the results, the solver, `unwind` and one option changed in turn | Each an equal identity |

## Expected Results

Each vector gives exactly its expected result: only the three components of the
triple change the identity.
