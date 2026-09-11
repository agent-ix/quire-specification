---
id: TC-006
title: "Enforce finite integer and normalized rational admission"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-019
    type: verifies
---
## Description

Exercise the replacement numeric domain through real model intake, checking and
evaluation. Planned; rational syntax/type/runtime correspondence is still open.

## Test Procedure

Admit explicitly declared signed-64 endpoints and reject unproved overflow,
missing endpoints, float and implicit conversion. Attempt integer division and
remainder under native spellings in both evaluated and unreachable expressions.
For an explicitly declared rational numerator range -1..1 and denominator maximum
2, compare the supplied fractions 3/6 and 1/2 after normalization. Try 2/3, a zero
denominator and a possible zero divisor under otherwise valid declarations.
Exercise rational arithmetic with a proved and an unproved intermediate bound.

## Expected Results

Defined bounded arithmetic yields its exact value. 3/6 and 1/2 normalize to the
same admitted value; 2/3 exceeds the denominator bound. Invalid/deferred cases
refuse without floating conversion, widening or a Boolean result. The old native
div/rem success tests do not qualify replacement admission.
