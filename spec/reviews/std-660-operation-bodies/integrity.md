---
id: SR-202611101500010
title: "Integrity review of the STD-660 operation_bodies carrier"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #651 at 6ff1281f: FR-322 Operation-body carrier section, FR-322-AC-397, TC-987, operation-body-vectors.json"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
  - { target: ix://agent-ix/quire-specification/TC-987, type: references }
---

## Summary

Ticket: STD-660. Consistency check of the reader rules against each other and against the
`operation_effects` rules. One rule is redundant: in an admitted package, the cross-member key-set
check can never fail, and its vector assigns a defect in `operation_effects` to `operation_bodies`.

## Verdict

CONDITIONAL. The per-row join order, the duplicate-key check before the order check, and the coverage
check match the `operation_effects` reader exactly. The Field-sources row, the member list and the
section agree on the following: the member is optional, absence means the bodies are unavailable,
and an empty array means no operations.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The rule "when the package also carries `operation_effects`, the two key sets are equal" cannot fail in an admitted package. Each reader's coverage rule already requires exactly one row for each operation declared by an admitted `object_type`, `process` or `state_machine` node in the selected documents. OBR-18 fires only because its `operation_effect_keys` drop `Order.touch`, and the `operation_effects` reader refuses that at `/operation_effects`. The vector puts the same defect at `/operation_bodies`, and FR-322 states no read order between the two members, so two conforming readers can report different loci. Fix: delete the rule and OBR-18, because equality follows from the two coverage rules. QSL FR-576-AC-7 keeps the producer-side property. | spec/objects/interfaces/FR-322-checked-package-artifact.md:3976-3978; proposals/checked-package-v2/operation-body-vectors.json OBR-18 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 498a3593 deletes the `operation_effects` key-set rule from the section and from FR-322-AC-397. It also removes OBR-18, the `operation_effect_keys` fixture member and its branch in the reader. Coverage is now the last check. |
