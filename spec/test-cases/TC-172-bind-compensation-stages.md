---
id: TC-172
title: "Keep compensation registration and activation captures separate"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-049
    type: verifies
---
## Description

Planned source/consumer correspondence checks for the native choreography
surface. No executable parser, replay or monitoring coverage is claimed.

## Test Procedure

Establish one payment effect, duplicate its receipt, and later supply one eligible shipment failure followed by another trigger. Capture a target at registration and a second value at activation, changing later input values. Exercise a failed initial refund followed by a valid retry, a fourth attempt against maximum three, and refunds before the forward effect or after its forbidding commit.

## Expected Results

One template/effect pair registers once and activates once under the admitted trigger order. The two capture stages retain their original anchors and values; premature reads refuse. Attempt, retry and business-effect identities remain distinct. Invalid commit/order/bound controls cannot satisfy the authored recovery relation.
