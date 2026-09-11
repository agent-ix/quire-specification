---
id: TC-141
title: "Correlate related instances without heuristics"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-111
    type: verifies
---
## Description

Planned OB01 correlation controls. No executable test or completed coverage is claimed.

## Test Procedure

Create concurrent O1/O2 with a shared provider under Producer interface 1.2.0.
Link a refund to O1's effect, offer it for O2, remove/link-conflict the relation,
repeat a delivery receipt, and independently mutate the producer version,
relationship identity, and interface digest.

## Expected Results

O1 is selected only by its declared relation; O2 substitution, unknown/conflicting
relation, repeated delivery, and every producer-selection mismatch cannot
manufacture an effect or a reconstructed relationship.
