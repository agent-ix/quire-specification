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
Carry event time, ingestion time, causal-order premise, and clock-uncertainty
premise independently through the real consumer boundary; mutate one field at a
time without replacing it with another field.
For timestamp coverage `[0,30)`, place a native inclusive-deadline witness at
30 and a separate decisive witness at 29. Repeat with an explicitly selected
timestamp range that contains 30, and independently use event-position and
fixed-sample ranges without converting either to UTC timestamps.

## Expected Results

Silence settles only under declared authority. Timestamp order does not establish
causality, uncertainty stays explicit, and late data is retained without silently
changing the selected settled result. Every mismatched assertion or missing
restoration basis remains incomplete/refused and cannot settle or restart the
assessment. Progress includes its selected start and excludes its end, so it
cannot settle or contradict the adjacent window through the shared boundary.
All four timing fields/premises remain distinct at the consumer boundary.
The 30 witness is not covered or relabelled as closure by `[0,30)`; the selected
covering range admits it. The 29 witness may settle from its exact support while
unrelated coverage remains incomplete. Event-position and fixed-sample controls
retain their own selected clock identities and do not manufacture timestamp
coverage.
