---
id: SR-206
title: "Scope-boundary review of settled truth on an open subject"
type: SpecReview
analysis: scope-boundary
scope: "FR-061 ownership across A/B/E/F, Quoin and Engineering Assurance"
review_set: all
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
---
## Summary

The revision stays within B's result-contract ownership. It consumes E's truth
and closure meaning and F's observation authority without redefining either,
while A retains exact profile adoption and Engineering Assurance/Quoin retain
execution and evidence ownership.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: the result contract now distinguishes evaluation completion from the assessed subject's progress and closure instead of silently owning E/F semantics. | FR-061; FR-091; FR-094 |
| FND-002 | medium | Resolved: the change adds no grammar, temporal evaluator, observation source, evidence store, method registry or production runner. | protocol-contract.md; IT-021 |
| FND-003 | high | Open handoff: A and F must amend their owned definition and consumer artifacts; B must provide exact changed bytes and distinguishing cases without merging edits into those files. | protocol-finite.md; FR-115; TC-145 |

## Responsibility allocation

| Owner | Responsibility |
| --- | --- |
| A | Native syntax, profile candidate revision/digests and adoption. |
| B | Portable orthogonal result dimensions, strict combination relation and decision support. |
| E | Temporal truth, admissible continuations and progress/closure interpretation. |
| F | Observation/completeness authority and preservation through consumer adapters. |
| Engineering Assurance / Quoin | Typed execution/results and retained evidence authority. |
