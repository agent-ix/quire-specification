---
id: IT-003
title: "Qualify verification reference and JCS correspondence"
type: IT
relationships:
  - target: "ix://agent-ix/quire-specification/FR-017"
    type: verifies
  - target: "ix://agent-ix/quire-specification/FR-018"
    type: verifies
---
# IT-003: Qualify verification reference and JCS correspondence

## Objective

Remove the representational gaps blocking B without conflating producer schema
checks, existing JCS parity and future strict-reader/report qualification.

## Target Integration

A's immutable schema/vectors and B's public Rust canonicalization functions,
followed by B's one strict shared-reference reader and report assembler. The
canonicalizer is quire-verification's `contracts::jcs_canonicalize` and
`contracts::jcs_sha256`. A does not change B's workspace or own its result schema.

## Preconditions

Missing inputs fail setup; they do not skip or pass. All work is local.

## Inputs

The complete amendment schema and separately authored JSON vectors. Each file
is read under caller limits on bytes and JSON values, with published defaults
of 1 MiB and 65,536 values, and nesting depth is not a limit; input over a
limit yields incomplete naming the limit, its value and the limit field that
raises it. Fixtures
include six exact roles and wrong-role controls, version/domain/algorithm/digest
mutations, duplicate/reordered features, exact property/subject/binding/closure/run
mismatches, and an explicitly synthetic automated-core correspondence example.

## Test Procedure

1. Validate the closed schema and positive/adverse structure (30 seconds).
   - IT-003-SC-01: Actual validation agrees with each structural expectation and records that role/identity semantics require further checks.
2. Invoke B's existing Rust JCS APIs on the independently authored canonical vectors (30 seconds).
   - IT-003-SC-02: Exact bytes and digest agree, including the non-circular core input and its mutations.
3. Pass the packet through B's adopted strict shared reader and selected report definition (30 seconds).
   - IT-003-SC-03: Exact-role/version/identity controls and actual report correspondence receive the specified outcomes.

## Expected Results

All three criteria require observed evidence for complete integration acceptance.
SC-01/02 can establish a reviewable producer/canonicalizer handoff now. SC-03 stays
planned until B adopts the amendment and implements its consumer. A synthetic
core fixture proves the selected projection rule only; it is never reported as
a valid VP04 report, completed portfolio, or actual human review.

## Metadata

Priority: High. Automation: Rust command/file/API tests. Status: planned.
Qualification logic belongs in Rust and reuses B's canonicalizer and a selected
Rust JSON Schema validator; it does not introduce a production shared reader.

## Dependencies

- [FR-017](../functional/foundation/FR-017-select-verification-artifact-roles.md)
- [FR-018](../functional/foundation/FR-018-select-verification-jcs-domain.md)
- [IT-002](IT-002-independent-reference-consumer.md) retains the broader FS02/FS05 independent-consumer gate.

## References

- VER-54: replace the fixed JSON depth and size caps in `quire-verification-contracts` with caller limits.
