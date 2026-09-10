---
id: TC-146
title: "Map observation results to consumers"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-116
    type: verifies
---
## Description

Planned consumer reconciliation controls for every acceptance criterion in
[FR-116](../functional/FR-116-map-observation-results-to-consumers.md). No
executable test or completed coverage is claimed.

## Test Procedure

Deliver preserved, pending, incomplete, unsupported, conditional, unrepresented, and refused per-obligation results to an executable/output consumer; attempt Boolean-pass coercion; and compare target text that is structurally accepted with a source-bound loss control.

## Expected Results

Subject/capability/provenance survives. Mapping loss remains classified, no non-success disposition is promoted to a pass, and structural target-text acceptance neither changes the mapping outcome nor establishes semantic preservation.
