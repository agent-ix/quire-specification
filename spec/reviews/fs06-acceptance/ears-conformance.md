---
id: SR-557
title: "EARS review of FS06 output-mapping acceptance"
type: SpecReview
analysis: ears-conformance
scope: "FR-120–125, FR-297–299 and NFR-060/061"
review_set: subset
review_date: "2026-09-14"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-120, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-121, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-122, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-123, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-124, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-125, type: reviews }
---
# EARS review of FS06 output-mapping acceptance

## Summary

PASS. The functional requirements use explicit event-driven subjects and
mandatory outcomes. The NFRs retain measurable system-wide quality obligations.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-021 | medium | Resolved: early FR-121 wording placed several mapper outcomes into one ambiguous sentence. The description and behavior now name the selected mapper for each independent SHALL. | FR-121 |
| FND-022 | medium | Resolved: target generation statements could read as broad language-support claims. Each is conditioned on one exact profile/revision and says the mapper generates only the admitted fragment. | FR-122–124 |
| FND-023 | low | Object descriptions use event-driven `When ..., actor SHALL ...` forms and their invariants define absence rather than relying on prose defaults. | FR-297–299 |
| FND-024 | low | NFR-060/061 define observable zero-defect thresholds and concrete boundary, metamorphic, fault, dependency and rights methods. | NFR-060/061 |

## Statement review

| Surface | Result |
| --- | --- |
| Intake and selection | `When` request admission occurs, the coordinator selects and validates exact identities. |
| Obligation accounting | `When` mapping occurs, the selected mapper returns exactly one explicit record per obligation. |
| Target generation | `When` one exact revision is selected, its mapper emits only the bounded fragment. |
| Package assembly | `When` every record is valid, the assembler emits one deterministic immutable package. |
| Quality constraints | Checked bounds, reproducibility, dependency containment and observer separation have measurable thresholds. |
