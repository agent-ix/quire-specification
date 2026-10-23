---
id: FR-327
title: "Protocol assessment request and result envelope"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_009
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-175
    type: references
---

# FR-327: Protocol assessment request and result envelope

## Description

When a protocol claim is submitted to an analyzer, the producer SHALL use the immutable ProtocolAssessmentEnvelope contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.protocol.assessment/v1`.
- `request_id` — identity over protocol, claim, premises, domain, provider and limits.
- `claim` — conformance, projection, realizability, composition or refinement with exact strength.
- `subject` — checked protocol plus observations or finite model.
- `premises` — closure, fairness, channel, population and environment assumptions.
- `provider` — selected capability, profile and options.
- `result` — FR-240 canonical result or analyzer-specific non-conclusive result.
- `witness` — strategy, trace set, counterexample or incompatibility core with source mapping.

## Identity and validation

The request and result keep claim kind, strength and premise identities independent; display labels never identify them.
A document whose `contract_version` is not `quire.protocol.assessment/v1`, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-327-AC-1 | Changing quantifier, direction, premise or bound creates a distinct request. | Analysis |
| FR-327-AC-2 | Unsupported and requires-bound dispositions emit no weakened claim. | Analysis |
| FR-327-AC-3 | Every witness validates against the same checked protocol and native semantics. | Analysis |

## Dependencies

- [interface_009](./interface_009-protocol-assessment.md) owns the operation boundary.
- FR-175 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract.
