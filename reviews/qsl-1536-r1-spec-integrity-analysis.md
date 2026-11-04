---
id: SR-20261103145402
title: "QSL-1536 r1 integrity analysis of PR #609 (Rational value types, Rational carrier)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #609 head cf327420 vs merge-base b1feb484; FR-154 (Rational value types, AC-42..44, Dependencies), FR-322 (step 4 rows, step-1 refused-set text, carried_quantity form, reader matrix, convert text, AC-380..382), FR-142 (Carried quantity, AC-27, AC-31), TC-187 U41, TC-981"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/TC-981
    type: references
---

# QSL-1536 r1 integrity analysis of PR #609

## Summary

Ticket: QSL-1536. This analysis checks ids, traces, cross-references and atomicity across the five touched files.

TC-981 `verifies` all seven new ACs, and each AC's Verification names TC-981. FR-154 gains a `references` edge to FR-149. AC-42..44, AC-380..382 and AC-31 are new and unique on main and on the open PRs.

TC-187 U41 still lists five carriers and still traces to FR-142-AC-27. The row count stays at 167, which matches `QUALIFICATION_VECTORS` and the README count. No test text-matches U41.

FR-322's carried_quantity form text, reader matrix row, `RM-CQ-CARRIER` mutation (still an unbounded `rational` node) and convert text agree with FR-142. The `rational_range` binding names `numerator_min`, `numerator_max`, `denominator_min` and `denominator_max` match the existing node in `hybrid-node-vectors.json`.

## Verdict

Integrity is sound apart from one low atomicity finding. The base review records the cross-document contradictions: shared-grammar, the FR-149 reference, and TK-22/TK-24.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-154-AC-44 bundles four independent claims: field and alias restatement, `unit` resolution, `unit` on a non-rational value type, and runtime value admission. Its `unit` clause ("refuses … when FR-322 resolves it") is not observable at FR-154 intake, and FR-322-AC-381 states the same clause again. Split the runtime-value clause into its own AC, and leave the unresolved-unit refusal to FR-322-AC-381 alone. | spec/functional/type-model/FR-154-admit-domain-package-model.md (FR-154-AC-44) |

## Dispositions

| ID | Disposition | Resolution |
| --- | --- | --- |
| FND-001 | fixed | FR-154-AC-44 is split: restating constraints moved to AC-45, `unit` form to AC-46 and runtime values to AC-47, and AC-44 now holds the first-defect order pairs. The unresolved-unit refusal stays only in FR-322-AC-381. |
