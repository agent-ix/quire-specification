---
id: TC-173
title: "Bind related instances and complete recovery populations"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-049
    type: verifies
---
## Description

Planned source/consumer correspondence checks for the native choreography
surface. No executable parser, replay or monitoring coverage is claimed.

## Test Procedure

Use O1/O2 sharing an external provider, related shipments S1/S2 and explicitly bounded refund populations. Validate a complete population with two distinct partial refunds whose exact sum equals the captured target. Change an endpoint direction, relabel O2's refund as O1's, remove a membership/closure assertion, duplicate a delivery receipt and separately add a second business effect.

## Expected Results

Declared relationships and view identities govern admission. Wrong-order and wrong-direction inputs refuse; unknown membership/completeness remains incomplete. A duplicate receipt does not duplicate an effect; a distinct effect remains an aggregate member and may violate the recovery target. No scalar-label match or provider/trace identity repairs the subject.
