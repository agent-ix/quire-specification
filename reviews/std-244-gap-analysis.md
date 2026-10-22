---
id: SR-14718
title: "gap analysis of STD-244 changed ACs against TC rows (QSpec PR #345)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@073ee8c1bd971d53c52138ad16f2b50e9c7e5e87 (diff against origin/main 97bfd65); spec/functional/analysis/FR-381-check-symmetric-populations.md, spec/functional/expressions/FR-145-convert-and-query-collections.md, spec/test-cases/TC-320-symmetric-population-authoring.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-381
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-145
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-320
    type: reviews
---
# SR-14718: gap analysis of STD-244 changed ACs against TC rows (QSpec PR #345)

## Summary

Ticket: STD-244. PR: quire-specification#345, head 073ee8c.

Plan completion: not assessed. Manual mapping. FR-381-AC-2 (changed) maps to TC-320 case 2: annotated `identity-observing-form`, unannotated `operator-ineligible`, and the counting `fold<Tally>` admitted in both. FR-381-AC-5 (new) maps to TC-320 case 6 (`config_history` annotated, `drafts` not, each fold reported by its own source). The FR-145 edit adds no AC; its precedence clause is verified through FR-381-AC-2 and AC-5. (The traversal class SR-14716 FND-001 asks to restore will need its own AC and row.)

Units examined:

- `FR-381-AC-2` (examined)
- `FR-381-AC-5` (examined)
- `FR-381` (examined)
- `FR-381-breaking-forms` (examined)
- `FR-381-precedence` (examined)
- `FR-145` (examined)
- `FR-145-convert` (context_only)
- `TC-320` (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean for the ACs as written. Every changed or new AC has a TC row that matches it.
