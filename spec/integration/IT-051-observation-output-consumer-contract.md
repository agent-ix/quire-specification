---
id: IT-051
title: "Integrate observation result mapping with executable and export consumers"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-115
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-116
    type: verifies
---
## Objective

Verify the planned consumer seam from the B-owned result contract to codegen and export consumers while preserving completeness, provenance, and mapping loss.

## Target Integration

The integration joins a structured per-obligation observation result with the existing codegen/output mapping boundary. It does not alter native clause authority, IR producer types, or a consumer-owned evidence store.

## Preconditions

The selected A/B result and interchange interfaces have named revisions; in
particular, the result input conforms to [B's FR-007](ix://agent-ix/quire-verification/FR-007).
The consumer declares supported capabilities and mapping profile.

## Inputs

One preserved result plus pending, incomplete, unsupported, and lossy obligation controls, each carrying source/binding/progress identities.

## Test Procedure

1. Deliver each structured disposition to the consumer mapping boundary.
   - IT-051-SC-01: identity and per-obligation disposition survive delivery.
2. Exercise conditional, unrepresented, and refused mapping controls.
   - IT-051-SC-02: the consumer emits the corresponding loss classification.
3. Attempt to coerce non-success dispositions into a Boolean pass.
   - IT-051-SC-03: coercion is refused or remains non-success.

## Expected Results

Consumers retain requested subject, capability, provenance, completeness, and loss state. This planned integration does not claim codegen CLI, backend qualification, or external mapping conformance.
