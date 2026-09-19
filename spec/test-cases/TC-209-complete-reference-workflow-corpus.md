---
id: TC-209
title: "Complete reference workflow corpus"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-180
    type: verifies
---

# TC-209: Complete reference workflow corpus

## Description

Run ConfigVersion, timed-refund and fan-out/join/compensation packages through all selected consumers.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-180. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-180; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.

## Case ownership

Each corpus case family is owned by the capability below; the qualification
ticket owns this case's verification/integration/claim closure and cannot
promote missing execution.

| Case family | Capability | Primary ticket | Qualification ticket |
| --- | --- | --- | --- |
| ConfigVersion | V1-RUN-010 | QSL #121 | QSL #123 |
| Timed-refund | V1-RUN-011 | Protocol #82 | Research #78 |
| Fan-out/join/compensation choreography | V1-RUN-012 | Protocol #82 | Research #78 |
| Cross-workflow aggregates | V1-RUN-008 | QObs #25 | QObs #26 |
| Composed domain execution (IT-071) | — | quire-integration | — |
