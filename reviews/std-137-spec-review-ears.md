---
id: SR-851
title: "EARS review of PR #175: probabilistic properties and dense time"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@d7919e0e5510d1e8f2e5830b4acb5138beaad0e5; PR #175 diff against origin/main a18859d; requirement statements of FR-405..FR-421 and the amended statements of FR-252 and FR-255"
review_set: subset
---
# EARS review of PR #175

## Summary

Ticket: STD-137, with STD-139. Each new Description is an event-driven EARS
statement ("When …, the … SHALL …"). Each names one system and one trigger.
The amended FR-252 and FR-255 statements keep the pattern. The ticket ids
appear only in References.

## Verdict

**Minor changes.** Two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-421's statement packs three SHALL clauses into one requirement: read through digital clocks, settle dense-valid verdicts, and settle `unsupported` outside the reading. FR-405, FR-407, FR-412, FR-413, FR-415, FR-418 and FR-419 each join two. Each clause has its own AC, so this is wording, not a gap. | spec/functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md:22-26 |
| FND-002 | low | FR-408 Admission says the provider settles `NotMarkov` "at negotiation" and, in the same item, "found while sampling". The trigger is sampling, not negotiation. | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:49-57 |

## Dispositions

Round 1, reviewed at 9ed6fab8b33842c1fef83d918d683f84d2b4de04 (fix commits 96a98644 and 9ed6fab8).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 96a98644 |
| FND-002 | fixed | 96a98644 |
