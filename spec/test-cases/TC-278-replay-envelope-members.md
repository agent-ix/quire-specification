---
id: TC-278
title: "Replay envelope members"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: verifies
---

# TC-278: Replay envelope members

## Description

Show that an FR-323 replay request's `replay` property carries the
`ReplaySource`, the selected function's `QualifiedName`, arguments keyed by
parameter node id, the semantic profile selections, the failing node's
occurrence key, the proof bounds and declared domains, the trace position
and the `backend` member, and that a non-replay request carries no `replay`
property, per QSL ADR-013 O-25, O-26 and QC-8.

## Preconditions

- FR-197's `ReplaySource` structural separation (`Witness`/`Input`) exists.
- FR-322 owns the `source_map` occurrence key (`node_id`, `role`, `ordinal`).
- QSL ADR-011 E9: the `replay` facade resolves the selected function by
  `QualifiedName` against the recompiled package's declarations.

## Test Procedure

Build an FR-323 replay request whose `replay` property carries every named
member, and the mutations that omit one member or supply a non-replay
request. Execute each vector through the real local Rust boundary and read
back the admitted request or refusal.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| RE-01 | A replay request whose `replay` property carries the `ReplaySource`, `QualifiedName`, parameter-node-id-keyed arguments, semantic profile selections, occurrence key, proof bounds and declared domains, trace position and `backend` member | Admitted; every named member is present | FR-323-AC-6 |
| RE-02 | The same request with its `QualifiedName` member removed | Refused before consumption | FR-323-AC-6 |
| RE-03 | The same request with an argument keyed by ordinal position instead of parameter node id | Refused before consumption | FR-323-AC-6 |
| RE-04 | The same request with its occurrence key replaced by a bare node id | Refused before consumption | FR-323-AC-6 |
| RE-05 | The same request with its `backend` member removed | Refused before consumption | FR-323-AC-6 |
| RE-06 | A non-replay `NativeRuntimeEnvelope` request | Carries no `replay` property | FR-323-AC-6 |

## Expected Results

RE-01 finds every QC-8 replay member present on a well-formed replay
request. RE-02 through RE-05 find each missing or malformed member refused
before consumption. RE-06 finds the `replay` property entirely absent from a
non-replay request.
