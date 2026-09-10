---
id: TC-143
title: "Preserve time, progress, and late-data authority"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-113
    type: verifies
---
## Description

Planned OB02 timing controls. No executable test or completed coverage is claimed.

## Test Procedure

Use a refund deadline with event and ingestion times, an explicit watermark, a
causal-order counterexample, clock uncertainty, and a late refund after
settlement; repeat without watermark authority. Mutate the selected assertion
definition bytes, scope, clock, source set, and restoration basis independently;
attempt continuation without the required predecessor/history.
Repeat with adjacent half-open windows and progress at the shared boundary.

## Expected Results

Silence settles only under declared authority. Timestamp order does not establish
causality, uncertainty stays explicit, and late data is retained without silently
changing the selected settled result. Every mismatched assertion or missing
restoration basis remains incomplete/refused and cannot settle or restart the
assessment. Progress includes its selected start and excludes its end, so it
cannot settle or contradict the adjacent window through the shared boundary.
