---
id: SR-16279
title: "spec-review of String, Rational, Float and Quantity collection elements (quire-specification PR 409)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@cab63de6; spec/functional/runtime/FR-181-simulate-finite-executions.md; spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md; spec/objects/interfaces/FR-322-checked-package-artifact.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-144
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
---

# Spec review of String, Rational, Float and Quantity collection elements (quire-specification PR 409)

## Summary

Ticket: STD-396. One spec-review pass over the PR's own delta (`git diff origin/main...cab63de6`), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across open PR heads, and schema/vector consistency.

Examined, clean:

- Float key and order, hand-derived: the totalOrder key (`not(b)` for sign 1, `b xor sign_mask` otherwise) is a bijection on bit patterns, so equal keys mean bit identity. The FR-144-AC-25 visiting order fff0, bff0, 8000, 0000, 3ff0, 7ff0 and the encoded-byte order 0000, 3ff0, 7ff0, 8000, bff0, fff0 are both correct. +0 and -0 are two members; NaN payloads are distinct.
- The String, Rational and Float vectors (C25-C27, FR-181-AC-22/24) are consistent. The earlier TC-280 MT-19 collision with #408 is gone (#408 now uses MT-20 to MT-23).
- Gates at cab63de6: quire validate 86; cargo 134 + 24 passed. The own delta is unchanged since 289272dd (main merges only).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | This contradicts requirements left unchanged in the same file. FR-181 "Canonical element encoding" still canonicalizes "a quantity over a decimal", FR-181-AC-19 encodes the quantity 1.50 metre as `{"coefficient":"150","scale":"2","type":"decimal","unit":...}`, and FR-181-AC-20 encodes it as a set element `15`/`1`. The new text, FR-181-AC-23 and C28 write a rational. FR-142 has one quantity type, so two producers following different rows give different state keys for the same set. Fix: pick one canonical form for a quantity, and rewrite or remove the decimal-quantity rule and FR-181-AC-19/20. | spec/functional/runtime/FR-181-simulate-finite-executions.md | wrong-requirement |

## Verdict

Not mergeable: FND-001 (high) block the PR.
