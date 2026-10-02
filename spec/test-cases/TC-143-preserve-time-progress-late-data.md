---
id: TC-143
title: "Preserve time, progress, and late-data authority"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-113
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-296
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-289
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-267
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-268
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-294
    type: verifies
---
## Description

Accepted FS04 portable OB02 timing controls. TM-007 remains Planned; no
executable standard coverage is claimed.

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
carrier `[0,e)` where `e > 30`, including an event between 30 and `e`; require
the adapter to retain that observation while excluding it from `[0,30]`.
Exercise point interval `[30,30]`, reversed bounds, end/watermark equality,
foreign clocks, discrete successor at its maximum value, and non-discrete time
without an explicit greater end. Independently use event-position and
fixed-sample ranges without converting either to UTC timestamps.
Derive authority and cutoff identities independently; mutate every authority
scope, clock, source-set, boundary and restoration component and every cutoff
clock, instant and rule-identity component. Feed exact `on-time`, exact `late`,
unknown, aliased and absent timing labels. Substitute clock uncertainty for the
clock selection and a JCS digest for raw definition bytes.

## Expected Results

Silence settles only under declared authority. Timestamp order does not establish
causality, uncertainty stays explicit, and late data is retained without silently
changing the selected settled result. Every mismatched assertion or missing
restoration basis remains incomplete/refused and cannot settle or restart the
assessment. Progress includes its selected start and excludes its end, so it
cannot settle or contradict the adjacent window through the shared boundary.
All four timing fields/premises remain distinct at the consumer boundary.
The 30 witness is not covered or relabelled as closure by `[0,30)`; the selected
greater carrier admits it and the above-30 carrier member does not participate
in the inclusive assessment. Equality, overflow, reversed bounds, a foreign
clock and absent non-discrete greater end refuse without invented epsilon. The
29 witness may settle from its exact support while
unrelated coverage remains incomplete. Event-position and fixed-sample controls
retain their own selected clock identities and do not manufacture timestamp
coverage.
Every semantic mutation changes only its applicable derived identity. Unknown,
aliased, absent or foreign-domain inputs refuse. Clock uncertainty remains a
value under the selected clock and cannot replace it. Exact `late` is retained
without result mutation or coercion into completeness; exact `on-time` carries
no implication of completeness or closure.
