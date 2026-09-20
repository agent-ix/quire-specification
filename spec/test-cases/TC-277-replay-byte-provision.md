---
id: TC-277
title: "Replay byte provision and source digests"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: verifies
---

# TC-277: Replay byte provision and source digests

## Description

Show that an FR-323 replay request's `byte_provision` supplies every source,
definition and domain-package input the replay recompiles, keyed by declared
digest domain and digest, and that the `package` reference's `RawSourceRef`
digests are exactly the digests the executor recomputes and compares, per
QSL ADR-013 O-26 and QC-1.

## Preconditions

- QSL ADR-011 §1: stages S1 to S4 recompile a checked package from
  digest-addressed source.
- FR-322 owns the checked-package reference and its digest domains.

## Test Procedure

Build FR-323 replay requests whose `byte_provision` supplies a complete,
digest-matching set of inputs, and the mutations that omit an input, supply
mismatched bytes, or name a digest domain the input does not carry. Execute
each vector through the real local Rust boundary and read back the
recompilation outcome.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| BP-01 | A `byte_provision` naming every source, definition and domain-package input by matching digest domain and digest | Recompilation proceeds; no missing-input refusal | FR-323-AC-5 |
| BP-02 | A `byte_provision` missing one input the `package` reference's `RawSourceRef` digests name | Refused `stale_dependency`/`byte-digest-mismatch` before recompilation | FR-323-AC-5 |
| BP-03 | A `byte_provision` entry whose bytes do not match its declared digest | Refused `stale_dependency`/`byte-digest-mismatch` | FR-323-AC-5 |
| BP-04 | A `byte_provision` entry under the wrong declared digest domain | Refused `stale_dependency`/`digest-domain-mismatch` | FR-323-AC-5 |
| BP-05 | A `byte_provision` whose bytes recompile to a `package_id` that differs from the `package` reference's declared `package_id` | Refused `stale_dependency`/`DependencyIdentityMismatch` before any result is produced | FR-323-AC-5 |

## Expected Results

BP-01 finds a complete provision admitted. BP-02 through BP-05 find each
incomplete, mismatched or stale provision refused with its named typed cause
before any recompilation result is produced.
