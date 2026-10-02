---
id: SR-822
title: "EARS analysis of STD-135/STD-136 requirement statements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@c54d91e1a7e9c69eba77c5f4b1505d192a9cc0e9; git diff origin/main...HEAD (PR #168): Description statements of FR-390..FR-403 and the amended FR-341 (infinite-trace) Description"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-390
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-402
    type: reviews
---
# SR-822: EARS analysis of STD-135/STD-136 requirement statements

## Summary

Ticket: STD-135 (with STD-136). PR agent-ix/quire-specification#168, draft.

Read the Description statement of each of the 14 new FRs and of the amended
FR-341. Twelve are event-driven EARS ("When <trigger>, the <system> SHALL
<response>") with one named actor and a testable response. The two below
are not quite EARS.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-390's response ends "and SHALL give it no truth value on a single trace". That is a negative response, so it states what isn't. State what the trace monitor does with a state-graph item (the Routing section already says it evaluates none), or drop the clause. | FR-390:26 |
| FND-002 | low | FR-402's second response, "the obligation identity of each of its items SHALL bind…", has an artifact as its subject, not a system. Name the actor that computes the identity, for example "the producer SHALL compute each item's obligation identity over…". | FR-402:23-27 |

## Verdict

Mergeable as far as this analysis goes. Both findings are wording only.

## Dispositions

Round 1, reviewed at 2b3fae1 (diff origin/spec/wave-b-q1-temporal...HEAD).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2b3fae1 |
| FND-002 | fixed | 2b3fae1 |
