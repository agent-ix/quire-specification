---
id: SR-16283
title: "spec-review of decimal literal value spelling (quire-specification PR 418)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@75f830a6; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/test-cases/TC-233-checked-package-and-scalar-contract-vectors.md; tests/checked_package_v2.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-140
    type: references
---

# Spec review of decimal literal value spelling (quire-specification PR 418)

## Summary

Ticket: STD-424. One spec-review pass over the PR's own delta (`git diff the head of #413...75f830a6`), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across open PR heads, and schema/vector consistency.

Examined, clean:

- The spelling rule (`.5`, `1.`, `+1.5`, `01.5`, `-0.0`, `1e2`, `1/4` refused; `0.00` and `10` admitted) and the oracle `decimal_literal_is_canonical` agree. Keeping the scale (`1.0` and `1.00` are two nodes) matches FR-181's retained coefficient and scale.
- Gates at 75f830a6: quire validate 86; cargo 136 + 24 passed.
- QSL conformance at QSL ef43bab4 with QSPEC_DIR at 75f830a6: checked_v2 47 passed, 0 failed.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | This contradicts FR-140 membership. FR-140 decides membership by value: s* = max(s, smin) and c* = v x 10^s*, and "a pre-normalized coefficient or scale never decides it". By hand: `"5"` under Decimal[-1000,1000;1,2] has s* = 1 and c* = 50, so FR-140 makes it a member, but AC-113 refuses it. `"1000.00"` under Decimal[0,1000;0,2] normalizes to (1000, 0), a member, but the written coefficient 100000 exceeds 1000 and it would be refused. Fix: admit a literal exactly when its value is an FR-140 member (keeping the written scale as the representation), and move `"5"` to the admitted list. | spec/objects/interfaces/FR-322-checked-package-artifact.md | wrong-requirement |

## Verdict

Not mergeable: FND-001 (high) block the PR.
