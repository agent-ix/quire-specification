---
id: IT-021
title: "Observation and conformance results preserve shared meaning"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-059
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-061
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-062
    type: verifies
---
## Objective

Verify the integration from F-owned observation/progress bindings through PT02
finite conformance into the shared result contract consumed by OB03 and
Engineering Assurance.

## Target Integration

The producer boundary is the reviewed observation, correlation, membership,
progress and closure contract. PT02 consumes it for global conformance and emits
the B-owned result contract. OB03 and Engineering Assurance are strict consumers.

## Preconditions

All three contracts and the fixture rights/provenance are pinned to exact
revisions. The assessment runner and evidence retention are supplied by the
shared Engineering Assurance system rather than a test-local substitute.

## Inputs

Healthy, violating, inactive, missing-provider, wrong-correlation, partial-
refund, failed-compensation and unknown-population cases with exact protocol,
binding, observation and progress identities.

## Test Procedure

1. Submit each case through the reviewed Engineering Assurance execution interface.
   - IT-021-SC-01: every case yields one typed result or an owned execution failure; the runner does not throw away the local result.
2. Read every result through the OB03 and Engineering Assurance strict readers.
   - IT-021-SC-02: both readers accept valid results and preserve every orthogonal dimension and identity.
3. Project every result into the exact E02 obligation and assess it through the quire-verification strict reader.
   - IT-021-SC-03: every FR-007 finding is preserved, the derived result retains the canonical protocol-result artifact, and observation adequacy remains distinct from protocol adequacy.
4. Delete or substitute each trigger, role, member, binding and progress dependency independently.
   - IT-021-SC-04: no mutation improves truth, completeness, adequacy or package status.

## Expected Results

All expected states remain distinct and reproducible. Missing or wrong inputs
degrade or refuse their dependent claims. The E02 projection is a distinct,
source-bound compatibility result and cannot replace the canonical protocol
result or conflate either adequacy domain. The integration retains shared
execution/evidence ownership and creates no local runner, transcript parser or
evidence store.

## Metadata

- Priority: High
- Target Integration: OB01/OB02 inputs to PT02 results to OB03/Engineering Assurance
- Automation: Planned Rust integration test

## Dependencies

[FR-059](../functional/FR-059-assess-finite-global-conformance.md),
[FR-061](../functional/FR-061-report-orthogonal-results.md),
[FR-062](../functional/FR-062-project-results-to-portable-verification.md),
OB01/OB02/OB03 and
the accepted Engineering Assurance typed interface.

## Notes

Engineering Assurance's Rust port is in progress. A missing producer capability
is recorded as an external dependency and candidate shared-tool improvement; it
does not authorize bespoke tooling here.

## Traceability

The test verifies the portable shared-result seam used by PT02 and OB03.
