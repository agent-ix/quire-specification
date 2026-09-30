---
id: SR-548
title: "Base review of FS04 temporal and observation acceptance"
type: SpecReview
analysis: base
scope: "FS04 #5; temporal clocks/history/progress, observation population/completeness and portable controls"
review_set: subset
review_date: "2026-09-14"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-090, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-091, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-092, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-093, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-094, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-095, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-110, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-116, type: reviews }
---
# Base review of FS04 temporal and observation acceptance

## Summary

PASS. FS04 accepts the already-authored temporal and observation meanings. It
does not claim that portable test controls have executed, close the remaining
native/TL composition, adopt producer-qualified topology, or define a
target-specific output profile.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: owning definitions and requirements still described the reviewed closure as Draft/Proposed despite merged compiler, observation, qcir and TL-owner boundaries. FS04 now accepts the semantic contract without promoting executable evidence. | FR-090–095; FR-110–116; NFR-040/050–052 |
| FND-002 | high | Resolved: Plan-002 treated completed observation/progress and Boolean-projection foundations as active/blocked. Their exact owner revisions are now recorded as done; qcir #71 is the sole active composition item and the final conformance gate remains pending. | Plan-002 Task-003/004/006/007; QObs PR #19; qcir PRs #67/#68/#77 |
| FND-004 | medium | Resolved: generic observation-result delivery could be confused with target-specific serialization. FR-116 and the mapping contract now reserve concrete oracle/runtime/report profiles to FS06. | FR-116; observation-output-mapping-contract; FS06 #7 |

## Acceptance check

- Event-position, fixed-sample and timestamped-window clocks retain distinct
  identity, progress and history rules; no implicit conversion establishes time.
- Future and past operators use exact inclusive bounds and explicit origin,
  cutoff, closure and unavailable/incomplete outcomes rather than Boolean defaults.
- Activation, immutable capture, duplicate/contradictory receipt and silent
  settlement depend on authenticated identity and progress.
- Population, membership, snapshot and completeness are explicit and bounded;
  EOF, ingestion order and observed count cannot manufacture closure.
- Replay and incremental assessment preserve the same admitted facts and report
  activation, participation, adequacy and result support as independent axes.
- Native/TL correspondence preserves clause, predicate, capture, clock,
  progress and result identity or refuses without a substitute formula.

## Evidence boundary

QSL PR #82 supplies strict temporal admission. QObs PR #19 supplies the declared
observation/progress owner contract. Qcir PRs #67/#68/#77 and the merged TL owner
APIs ground the proposition/correspondence boundaries. Qcir #71, QObs #18/#13,
FS06, TM-004/TM-007 and the integration tests retain their distinct work.
