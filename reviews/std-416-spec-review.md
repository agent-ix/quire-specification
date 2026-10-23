---
id: SR-16282
title: "spec-review of float32_bits and float64_bits literal spelling (quire-specification PR 413)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@4f21656e; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/test-cases/TC-233-checked-package-and-scalar-contract-vectors.md; tests/checked_package_v2.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# Spec review of float32_bits and float64_bits literal spelling (quire-specification PR 413)

## Summary

Ticket: STD-416. One spec-review pass over the PR's own delta (`git diff the head of #408...4f21656e`), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across open PR heads, and schema/vector consistency.

Examined, clean:

- The spelling matches FR-181's `bits` form, FR-148's bit-pattern semantics and #409/#419's NaN and signed-zero distinctness. The oracle checks the length and the lowercase hex, and the test walks every fixture float literal. FB-04's family-first order matches the other scalar literals.
- Gates at 4f21656e: quire validate 86; cargo 135 + 24 passed. No schema or fixture changed.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | correct-requirement-no-evidence |

## Verdict

Mergeable: no defects found in this PR's own delta; merge in stack order.
