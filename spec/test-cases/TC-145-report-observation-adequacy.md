---
id: TC-145
title: "Report observation adequacy without promotion"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-115
    type: verifies
---
## Description

Planned OB03 result/adequacy controls. No executable test or completed coverage is claimed.

## Test Procedure

Compare healthy, violating, untriggered, missing-data, and unsupported traces. Drop trigger telemetry, mis-correlate a response, and pair a confirmed injected fault with independently observed mitigation.

## Expected Results

All dispositions remain distinct; dropped/mis-correlated inputs cannot improve adequacy; fault/mitigation lineage retains property, binding, source, and run identities.

