---
id: SR-13633
title: "Integrity review of STD-155 FR-154 parameter-name precedence (PR #307)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; FR-154, TC-235, TC-280, adverse.json (PR #307 head d586f0d)"
review_set: subset
date: 2026-10-21
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/TC-235
    type: references
---

# Integrity review of STD-155 FR-154 parameter-name precedence (PR #307)

## Summary

Ticket: STD-155. Checked where the new row falls in FR-154's declaration-refusal order and how it fits with FR-154-AC-13 and TC-235.

Units examined:

- `FR-154-AC-14` (examined): spec/functional/type-model/FR-154-admit-domain-package-model.md
- `FR-154` (examined): spec/functional/type-model/FR-154-admit-domain-package-model.md
- `FR-154` (examined): spec/functional/type-model/FR-154-admit-domain-package-model.md
- `FR-154-AC-13` (context_only): spec/functional/type-model/FR-154-admit-domain-package-model.md
- `TC-280` (examined): spec/test-cases/TC-280-model-owned-member-types.md
- `TC-235` (context_only): spec/test-cases/TC-235-business-construct-meaning-binding.md

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Checking names before the repeated-identity row contradicts FR-154-AC-13 and TC-235 M33 run 4. There, `OrderStore/get` has params `left` and `right`, `right` carrying `.../get/left`, and the expected result is exactly one `conflicting-binding` naming `.../get/left` at `TypeDefinition.operations[0].params[1]`. Under the new rule `right` differs from the last segment `left`, so it refuses `malformed-declaration` at `params[1].name` first. AC-13 also says the collision suppresses the identity checks of both colliding members, "whichever of the two ... carries the correctly minted identity". `parameter-name-repeated-after-malformed-name` (param 0 `name` "" and param 1, both identity `.../scaled/n`) is such a collision, yet it expects `malformed-declaration`. Pick one precedence and make AC-13, AC-14, TC-235 M33 and the vectors agree. | spec/functional/type-model/FR-154-admit-domain-package-model.md:84,96-100,269-270 |
| FND-002 | low | "The parameter-name row sits before the frame-entry row of the table, so ... checked ... before any frame entry" derives check order from table position. FR-154's "Declaration refusals" table, unlike "Admission refusals, in order", declares no order, and the variant collision row already overrides earlier rows. State the order directly, not as a consequence of row position. | spec/functional/type-model/FR-154-admit-domain-package-model.md:96-99 |

## Verdict

Not mergeable. Checking names first contradicts the existing parameter-collision rule and vector (FND-001). The table-order argument behind the precedence is not supported by FR-154 (FND-002).

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
