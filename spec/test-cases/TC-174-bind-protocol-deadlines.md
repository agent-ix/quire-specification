---
id: TC-174
title: "Preserve deadline branches and explicit workflow closure"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-049
    type: verifies
---
## Description

Planned source/consumer correspondence checks for the native choreography
surface. No executable parser, replay or monitoring coverage is claimed.

## Test Procedure

Replace the immediate receive in the source fragment with its timestamped await alternative. Compare a matching response exactly at 30 seconds, a later response, no response with complete progress through 30, missing provider completeness, and a bound response with a false constraint, once with closed complete progress and once with open progress. Supply a finish record with and without complete required observations/obligations, keeping EOF and business action execution separate.

## Expected Results

The inclusive response participates before timeout settlement. Missing evidence and false constraints do not select timeout as a fallback; the false constraint with closed complete progress selects no branch and reports `violated` with basis `decisive-counterexample`, and with open progress reports `pending`. Only authorized complete progress establishes the miss; late contradictions retain earlier result identities. A finish payload alone cannot establish global success, and assessment invokes no business operation.
