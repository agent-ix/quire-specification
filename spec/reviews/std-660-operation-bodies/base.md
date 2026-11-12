---
id: SR-202611101500009
title: "Base review of the STD-660 operation_bodies carrier"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR #651 at 6ff1281f, diff origin/main(b690867b)...HEAD: spec/objects/interfaces/FR-322-checked-package-artifact.md (operation_bodies member, Operation-body carrier section, FR-322-AC-397), spec/test-cases/TC-987-operation-body-carrier.md, proposals/checked-package-v2/{schema.json,operation-body-vectors.json,README.md}"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
  - { target: ix://agent-ix/quire-specification/TC-987, type: references }
---

## Summary

Ticket: STD-660. Base checklist over the FR-322 amendment that admits the optional top-level
`operation_bodies` member for QSL FR-576. The row shape, key, sort order and per-row joins match
QSL FR-576 (quire-spec-language main) and follow the FR-496 `operation_effects` precedent.
TC-987's expected codes match the 13 schema and 18 reader vectors one for one. One structural
defect: the new criterion FR-322-AC-397 sits outside the acceptance-criteria table, so quire does
not mint it.

## Verdict

FAIL on FND-001 only. The `quire validate` claim is measured: both origin/main b690867b and the PR
head 6ff1281f report the same two structural failures,
`spec/functional/analysis/FR-407-state-probabilistic-claims.md` (FR-407-AC-8 row width) and
`spec/objects/interfaces/FR-341-checked-package-state-clause-body.md` (FR-341-AC-17 row width).
The claim is true, but it does not show FND-001: the criterion count is 3194 on both trees.

Examined and clean:
- Row `{document, declaring, operation, body}`, key `(document, declaring, operation)` and sort
  order, as FR-576 Behavior and FR-496 require.
- The `function`/`no_body` tagged union. No row or `body` may be omitted or null.
- The empty-array decision. `operation_effects` keeps `minItems: 1`, and QSL FR-496 omits that member
  rather than emitting it empty. So a package with no operations carries `operation_bodies: []` and
  no `operation_effects`. The key-set rule applies only when both members are carried, so the two
  members do not contradict each other. The extra value is real: `[]` states "no operations",
  where absence states "unavailable".
- No hash, digest, pin or tracking record anywhere in the diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The FR-322-AC-397 row follows a blank line after the FR-322-AC-396 row, so it is a separate paragraph and not a row of the acceptance-criteria table. quire mints no FR-322-AC-397: `quire matrix` has no row for it, the criterion count is 3194 on both main and head, and the trace tag on the new test raises `untracked-id-has-minted-children`. Fix: delete the blank line at 4775 so that the row continues the table. | spec/objects/interfaces/FR-322-checked-package-artifact.md:4775-4776 |
