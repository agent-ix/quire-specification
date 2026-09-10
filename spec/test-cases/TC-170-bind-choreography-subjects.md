---
id: TC-170
title: "Bind choreography references without identity substitution"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-049
    type: verifies
---
## Description

Planned source/consumer correspondence checks for the native choreography
surface. No executable parser, replay or monitoring coverage is claimed.

## Test Procedure

Place the choreography declaration fragment in a complete unit with exact synthetic model/profile definitions. Independently substitute each role, channel, message, attempt, effect, relationship and referenced state/temporal declaration kind. Supply one send with two delivery receipts and one business effect. Request global replay and unsupported projection separately.

## Expected Results

The accepted source retains every typed declaration and span. Each wrong-kind or wrong-subject mutant refuses its actual binding; two receipts retain one established effect. An unsupported projection retains the admitted native subject and the independent replay request.
