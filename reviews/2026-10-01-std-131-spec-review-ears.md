---
id: SR-801
title: "EARS analysis of STD-131/STD-132 requirement statements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@ca8033614b06c21ea8362f2ea1b2fa38ad910739; Description statements of spec/functional/temporal/FR-361..FR-369 and spec/objects/interfaces/FR-370, and the added SHALL paragraphs in FR-048, FR-090, FR-091, FR-092 and FR-161"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-361
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-370
    type: reviews
---
# SR-801: EARS analysis of STD-131/STD-132 requirement statements

## Summary

Ticket: STD-131 (with STD-132). PR agent-ix/quire-specification#170.

Each new FR Description is one EARS statement with a named subject before
SHALL. FR-361 to FR-366, FR-368 and FR-369 use the event form ("When ..., the
analyzer/replayer/request writer/producer SHALL ..."). FR-367 uses the optional
form ("Where the infinite-trace profile is selected, the evaluator SHALL ...").
FR-370 uses the event form for the producer and adds a second statement for
the reader. The SHALL paragraphs added to FR-048, FR-091 and FR-092 use the
optional "Where ..." form. `quire validate` reports no EARS warning on the new
files.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The statements conform to EARS.
