---
id: SR-16361
title: "Failure-domain review of state-graph provider form negotiation"
type: SpecReview
analysis: failure-domain
scope: "FR-331-AC-42, FR-290-AC-16, TC-968, provider v1 schema"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/TC-968
    type: references
---

# Failure-domain review of state-graph provider form negotiation

## Summary

Reviewed descriptor omission and malformed-value handling, candidate mismatch,
and cross-kind isolation. No additional failure boundary is left unspecified.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found. Omitted and partial declarations have explicit semantics; malformed declarations refuse at manifest validation, while unsupported exact-form requests settle through the existing empty-candidate rule. | FR-331-AC-42, FR-290-AC-16, TC-968 |

## Verdict

PASS. The schema rejects forms on non-temporal descriptors and negotiation
does not extend the form filter to formulas, hyperproperties or step relations.
