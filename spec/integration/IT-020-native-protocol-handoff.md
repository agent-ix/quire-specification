---
id: IT-020
title: "Native compiler hands off an exact linked protocol"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-050
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-052
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-053
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-054
    type: verifies
---
## Objective

Verify that Agent A's native compiler and the PT01 protocol consumer agree on one
typed representation without reparsing source text or changing source,
model, profile, role, message, channel or control identities.

## Target Integration

The producer is the native Quire linker under L6. The consumer is the future
Rust PT01 protocol implementation. The boundary is the reviewed linked-package
protocol representation selected by the composed native profile.

## Preconditions

The composed native profile, protocol representation and strict consumer are
selected by exact identity. The order/payment/split-shipment fixture declares
all required finite bounds and source/model identities.

## Inputs

One valid linked branching/compensation package plus independent mutants for
role identity, source span, message/effect type, channel premise, choice owner,
join edge and iteration maximum.

## Test Procedure

1. Produce and consume the valid linked protocol without source-text reparsing.
   - IT-020-SC-01: the strict consumer retains every exact producer identity and admits the representation.
2. Apply each one-axis mutant at the boundary.
   - IT-020-SC-02: the strict consumer refuses each mutant with its typed field/path and no conformance execution.
3. Repeat with presentation-only permutations of map-like members.
   - IT-020-SC-03: semantic identity and linked meaning remain unchanged.

## Expected Results

The valid representation is consumed exactly once as typed data. Every semantic
mutation refuses before evaluation. Presentation-only permutations do not change
meaning. No second parser, model vocabulary or source authority appears at the
boundary.

## Metadata

- Priority: High
- Target Integration: native linked package to PT01 protocol consumer
- Automation: Planned Rust integration test

## Dependencies

Agent A's composed grammar/linker and [FR-050](../functional/protocol/FR-050-bind-protocol-instances.md)
through [FR-054](../functional/protocol/FR-054-bind-channel-premises.md).

## Notes

This artifact defines a planned integration contract. It does not claim the
compiler or protocol repository already implements it.

## Traceability

The test verifies the protocol representation handoff required by PT01 and L6.
