---
id: SR-16060
title: "Spec review — verbatim field, operation and parameter identity segments (FR-154)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; spec/functional/type-model/FR-154-admit-domain-package-model.md, spec/test-cases/TC-235-business-construct-meaning-binding.md at 7b56ed2"
review_set: subset
evaluated_revision: "7b56ed2805778bb133563646e60c265f54c153ab"
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-235
    type: reviews
---

## Summary

Ticket: STD-292. PR quire-specification#378 states that a field, operation
and operation parameter identity segment is the declared `name` verbatim under
the member-name rule, adds a declaration-refusal row for a field or operation
name that fails the rule or differs from its last identity segment, notes that
a field and a state with one name in one owner collide, and adds FR-154-AC-20
with TC-235 vector M35. Checked against QSpec main 5ee0071, QSL main 39823fe
(`qsl-semantics` intake fixtures and the hand-coded TC-235 tests) and
filament-core-data #302 (e020b1e).

## Verdict

PASS with minor findings. The rule matches FCD #302: the same member-name
regex, verbatim segments on both frontends, a slugged constrained-field alias,
and the field/state collision raised at the later member (states after
fields) in both specs. The M35 vectors were checked by hand: `fields[1]`,
`operations[2]`, `params[0]` and `states[3]` match K2 (one `OrderLifecycle`
field, two `OrderStore` operations, three states), the 23 charges match K2's
21 declarations plus one field and one operation (parameters are not charged,
as M34 run 2 shows), run 3's parameter identity nests correctly under the
declared operation identity, and run 4 names the operation identity as
FR-154-AC-14 requires. No existing identity changes: QSpec proposals and TC
vectors, and QSL fixtures (`money_field`, K2 tests), already mint verbatim
member segments (`Money/amount_minor`, `OrderPlaced/occurred_at`). QSL tests
hand-code the TC-235 vectors, so no test-parsed fixture changes and no QSL
conformance run is needed. QSL intake has no field or operation name check
yet, so the new refusal row is new QSL work. The ids FR-154-AC-20 and M35 are
unique across open PRs. #374 conflicts textually in the FR-154 AC table
(append-only rows); #267 already conflicts with main. Gates: `quire validate`
86 failure lines with no FR-154 or TC-235 diagnostics; `cargo test --test
checked_package_v2 --test backend_provider_v1` passes (23 + 119).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-154-AC-20 and M35 only cover the mismatch branch of the new row. No vector sends a field or operation `name` that is absent, not a string, empty, or outside `^[A-Za-z][A-Za-z0-9_]*$` (for example `_tmp` with identity `.../O/_tmp`), so those four conditions of the row are untested. FR-154-AC-14 covers each of them for parameters. | spec/functional/type-model/FR-154-admit-domain-package-model.md:85, spec/functional/type-model/FR-154-admit-domain-package-model.md:346 |
| FND-002 | medium | FR-154-AC-20's final clause (the constrained-field alias of `created_at` on `O` is `ix://test/orders/OCreated-at`) has no input in M35, which declares no constrained field. The clause is verified by Test (TC-235) but no vector can fail it. | spec/functional/type-model/FR-154-admit-domain-package-model.md:346, spec/test-cases/TC-235-business-construct-meaning-binding.md:114 |
| FND-003 | low | The new row says "(checked after that row)", but no row is named. In the row directly above, "that row" refers to the repeated-identity row, which sits further down the table. A reader can take it to mean the parameter row above. The prose at line 265 does name "the repeated-identity check of its owner". | spec/functional/type-model/FR-154-admit-domain-package-model.md:85 |
| FND-004 | low | FR-154-AC-20 says the alias "stays" `ix://test/orders/OCreated-at`. That is change-relative wording in a behaviour statement; it should state the value ("is"). | spec/functional/type-model/FR-154-admit-domain-package-model.md:346 |
