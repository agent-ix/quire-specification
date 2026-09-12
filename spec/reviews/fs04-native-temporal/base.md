---
id: SR-410
title: "Base review of native temporal evaluator semantics"
type: SpecReview
analysis: base
scope: "FR-091, FR-093, FR-094, temporal-common, TC-111, TC-114 and TC-116"
review_set: all
evaluated_revision: "b78fccb"
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-091, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-093, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-094, type: reviews }
---
## Summary

The amendment makes the selected temporal semantics explicit: Boolean
connectors are pointwise, duplicate receipt and guard states are not silently
coerced, and an exhausted obligation cannot erase a settled sibling.

## Verdict

PASS for specification integration. Runtime implementation remains gated on the
reviewed `/2` compiler contract, observation authority, FR-025 predicate
projection and FR-026 correspondence selection.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: an interval-aggregate reading of Boolean connectors could disagree with formula semantics. FR-091 now requires pointwise evaluation and TC-111 distinguishes it. | FR-091; TC-111 |
| FND-002 | high | Resolved: false guards, unavailable guards and conflicting duplicate receipts had no total separation. FR-093 now assigns false/no-obligation, incomplete/refused and typed contradiction respectively. | FR-093; TC-114 |
| FND-003 | high | Resolved: a shared resource ceiling could have discarded unrelated settled outcomes. FR-094 preserves those outcomes and returns a non-Boolean result only for the affected obligation. | FR-094; TC-116 |

