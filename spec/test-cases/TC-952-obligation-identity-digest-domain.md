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

Each fixture is one obligation of a checked package, carried with every member
of CG's obligation record, `source_span` and the run configuration included.

- `O1`: a clause, its `claim` occurrence key (`node_id`, `claim`, 0), one
  obligation kind and one argument, a parameter node id with the declared
  domain `Int[0, 9]`.
- `A1`: an operation application, its `expression` occurrence key
  (`node_id`, `expression`, 0), one obligation kind and one argument.
- `F1`: a function, its `declaration` occurrence key (function node id,
  `declaration`, 0), one obligation kind and one argument.
- `P1`: a parity claim on the integer addition `p + 1`, where `p` is an
  Integer-typed parameter with declared domain `Int[0, 9]`: the application
  node and its `expression` occurrence key, with operands `p` at position 0
  (`graph_child`) and the integer literal `1` at position 1 (`inline_literal`,
  range (1, 1)). Addition is not an equality against a bounded peer, so the
  literal stays unwrapped.
- `S1`: a per-step protocol record, `subject_kind`
  `protocol-refinement-step` and a canonical `StepSimulationRecord` with two
  hypotheses, carried with an occurrence key.

## Test Procedure

Build each vector's obligation through CG's Kani obligation identity boundary
(the parity form through the QSL recomputation as well), compute its
obligation identity and compare it with its fixture's.

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| OI-01 | `O1` with its `source_span` moved to another region of the same source | Equal digest | FR-201-AC-10 |
| OI-02 | `O1` with its `source_span` in another source document whose package compiles to the same node ids | Equal digest | FR-201-AC-10 |
| OI-03 | `O1` with another clause's node id | Different digest | FR-201-AC-10 |
| OI-04 | `O1` with occurrence `ordinal` 1, the second occurrence of a structurally identical clause | Different digest | FR-201-AC-10 |
| OI-05 | `O1` with another obligation kind | Different digest | FR-201-AC-10 |
| OI-06 | `O1` with another parameter node id; then with the declared domain `Int[0, 8]` | Each a different digest | FR-201-AC-10 |
| OI-07 | `O1`'s digest bytes labelled `quire.checked-semantic-node/v1`, then `sha256-jcs` | Each compares unequal to `O1`'s identity | FR-201-AC-10 |
| OI-08 | `O1` with each of these changed in turn: the resource ceilings, an embedded oracle, the harness symbol, the contract symbol, the subject path, the results, the solver, `unwind`, one option | Each an equal digest | FR-201-AC-10 |
| OI-09 | `A1` with its occurrence `ordinal` 1, the second occurrence of one application node; then with one argument's declared domain changed | Each a different digest | FR-201-AC-10 |
| OI-10 | `F1` with another function node id; then with the occurrence role `claim` in place of `declaration`; then with a changed run configuration member as in OI-08 | The first two a different digest, the last an equal digest | FR-201-AC-10 |
| OI-11 | `P1` with the literal `2` at position 1; then with the operands swapped; then with the domain of position 0 `Int[0, 8]` | Each a different digest | FR-201-AC-10 |
| OI-12 | `S1` with one hypothesis removed; then with another occurrence key; then with its `source_span` moved | The first a different digest, the last two an equal digest | FR-201-AC-10 |

## Expected Results

Each vector gives exactly its expected result: `source_span`, source regions
and the run configuration never change the identity, every preimage member of
the obligation's subject form does, the per-step record form ignores its
occurrence key, and the domain label keeps an equal-bytes digest of another
domain unequal.
