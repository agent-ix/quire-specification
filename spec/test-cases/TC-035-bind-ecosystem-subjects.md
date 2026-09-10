---
id: TC-035
title: "Retain explicit ecosystem scope identities in assessment requests"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-035
    type: verifies
---
## Description

Planned acceptance controls for [FR-035](../functional/FR-035-bind-ecosystem-subjects.md).
No executable test or completed coverage is claimed by this artifact.

## Test Procedure

Bind O1 and O2 with distinct workflow identities but a shared external provider. Link O1's payment effect to a refund and two shipments. Substitute a delivery or trace identity for the effect. Bind a declared population window, then change its identity and separately make required membership unavailable.

## Expected Results

Declared relationships and workflow identities survive binding. Substitutions refuse, scope changes remain visible in request identity, and unavailable membership cannot produce aggregate success. Concrete producer/observation representations come from D/F's reviewed contracts, not a duplicated test model.

