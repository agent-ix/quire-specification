---
id: SR-305
title: "Risk and complexity review of native temporal lifecycle correspondence"
type: SpecReview
analysis: risk-complexity
scope: "Native temporal clocks, history, settlement, bridge correspondence and bounded resource behavior"
review_set: all
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/NFR-040, type: reviews }
---
## Summary

Temporal correctness is high risk because clock/profile substitution, false
closure, incomplete history or lost settlement support can turn missing data
into a false pass or violation. Complexity stays bounded by finite intervals,
explicit formula/population limits and exact profile-specific rules.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: early truth cannot be inferred from a favorable prefix; the selected profile must establish a decisive witness/counterexample over every admitted continuation. | FR-091-AC-4; TC-116 |
| FND-002 | high | Resolved: exact decision support and both closure axes survive TL mapping even when the TL syntax/result lacks those fields. | FR-095-AC-7; TC-117 |
| FND-003 | medium | Resolved: finite exhaustive caps and explicit resource-stop reporting prevent an unbounded assurance promise. | FR-095-AC-1; NFR-040; TC-117/118 |
| FND-004 | medium | Retained risk: implementation must keep event-position, fixed-sample and timestamped-window rules separate and refuse unsupported past/time mappings. | FR-090; FR-095 support table |
| FND-005 | medium | Retained risk: F consumer and A profile-revision gates remain required before composed adoption. | FR-115/116; definitions/README.md |

## Risk controls

| Risk | Control |
| --- | --- |
| Optimistic open-prefix success | Paired decisive/non-decisive cases with exact support and basis mutations. |
| False inactive result | Closed-complete, open and missing-trigger cases differ one axis at a time. |
| Clock/profile confusion | Three profile discriminators plus identity, period, unit, order and boundary mutations. |
| State rollback | Same-binding regression/conflict controls retain old identity and bytes. |
| Assurance overclaim | Closed finite population, truthful resource stop, and all evidence rows remain planned. |
