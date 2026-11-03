---
id: SR-20261103145401
title: "QSL-1536 r1 spec review of PR #609 (Rational value types, Rational carrier)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR #609 head cf327420 vs merge-base b1feb484; spec/functional/type-model/FR-154-admit-domain-package-model.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/functional/type-model/FR-142-evaluate-quantities-and-units.md; spec/test-cases/TC-187-quantity-and-unit-conversion.md; spec/test-cases/TC-981-rational-value-types.md; context: FR-149, FR-140, FR-478, FR-271, proposals/quire-v1/shared-grammar.md, model-tuple-union-vectors.json, QSL FR-505 and FR-497 (quire-spec-language origin/main fc091fb1e)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-142
    type: references
---

# QSL-1536 r1 spec review of PR #609

## Summary

Ticket: QSL-1536. The PR is spec-only. It adds FR-154 "Rational value types" with AC-42..44, two FR-322 step-4 rows and the `rational_range` carrier with AC-380..382, and the FR-142 `Rational[..]` carrier with AC-31. It adds TC-981 and edits TC-187 U41.

`quire validate` (quire 0.36.2, engine 0.50.2) over the five touched files gives output identical to the base snapshot b1feb484. The only failure is the pre-existing FR-322 error (missing id, record-invalid), and TC-981 is clean. The PR adds no new findings.

AC ids do not collide. Main and open PRs #606 (FR-322-AC-342..370), #607 (merged, AC-371) and #600 (TC-980) use none of FR-154-AC-42..44, FR-322-AC-380..382, FR-142-AC-31 or TC-981.

GitHub reports the PR as CONFLICTING. Main moved by #607, which appended FR-322-AC-371 at the tail of the AC table. The conflict is textual only and needs a rebase.

## Verdict

**Not mergeable yet.** There are five medium findings, all fixable in this PR's files. The five semantic choices were checked as follows.

1. **Empty constraints mean the unbounded `Rational`: consistent with QSpec, and it conflicts with QSL FR-505 row 3 read literally.** It keeps FR-322-AC-181..183 true for "a value type bound to `Rational`". It also matches FR-505's own statement that an unbounded rational site is the bare native. FR-505 must be amended (FND-007). However, the existing vectors and the Rust oracle model that phrase as a value type *with* bounds (FND-001).
2. **Zero `minDenominator`: sound.** Row 4 checks "JSON integer in i64" and row 5 checks `1 <= d1 <= d2`. This agrees with FR-505-AC-2, the testable claim, and with FR-505's own ordered list. FR-154, FR-322 and TC-981 agree with each other. FR-505's prose sentence that routes an out-of-range denominator to `malformed-declaration` needs a QSL amendment (FND-007).
3. **Unit-bearing type as `Quantity<u, Rational[..]>` via `carried_quantity`, `exact`, with no new arithmetic: consistent with FR-142** (mode `exact`, distinct types, convert-out arithmetic), **with FR-478** (the `rational_out_of_domain` precedent) **and with FR-322's convert mode rule.** It contradicts QSL FR-497-AC-1/AC-2, which refuse `Rational[0, 9; 1, 9]`. That follow-up is correctly named in the PR body. One consequence is not reconciled: QSL FR-505 expects a unit-bearing type to be "declared and compared", but a carried quantity cannot be an operand of `=` or `<` (FND-005).
4. **`ix://quire/unit/<name>` spelling: consistent** with FR-154's unit `typeRef` form and with FR-505's own "as FR-154 reads it". QSL FR-505-AC-7's bare `m` would hit the new `malformed-declaration` row at `TypeDefinition.unit`, which no QSpec AC tests (gap-analysis FND-003).
5. **`rational_out_of_domain`, not `inexact_conversion`: sound.** A `Rational[..]` carrier has no grid. This matches FR-149 rational narrowing, FR-478-AC-7, FR-044-AC-10 and the FR-322 convert text. The diagnostic catalog wording lags behind (FND-006).

The refusal list is deterministic: rows 1-5 are ordered, and within rows 3 and 4 the order is by keyword. It is not exhaustive (FND-004).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-181/182/183 are unchanged, and their vectors TK-22 and TK-24 (and the TC-280 oracle) model "a package value type bound to `Rational`" as `{native: Rational, lower: "0", upper: "9"}`, a value type *with* bounds, and assert refusal or no type. The PR reinterprets that phrase as "a Rational value type with no constraints" and admits and types a bounded one as a payload, position and member (AC-380). As written, a reader cannot tell which vector shape is the refused unbounded case. Fix: restate AC-181..183 as "a Rational value type with no `constraints`", and drop `lower`/`upper` from TK-22, TK-24 and `Rat`, or say they are numerator-only and so malformed under row 3. | spec/objects/interfaces/FR-322-checked-package-artifact.md:4152-4154; proposals/checked-package-v2/model-tuple-union-vectors.json:996-1080; tests/model_member_types.rs:646 |
| FND-002 | medium | `proposals/quire-v1/shared-grammar.md`, which FR-154 names as the owner of the grammar, still says "the checker restricts `C` to a bounded `Int[..]` or `Decimal[..]` (FR-142 \"Carried quantity\")". That now contradicts FR-142, which admits `Rational[..]`. Fix: add `Rational[..]` in this PR. | proposals/quire-v1/shared-grammar.md:1082 |
| FND-003 | medium | FR-142 says a `Rational[..]` carrier with `n1 > n2` or outside `1 <= d1 <= d2` is refused "as FR-149 refuses its `Rational[..]`". FR-149 states no such well-formedness rule, and no QSpec requirement defines when a source `Rational[n1, n2; d1, d2]` is well formed or what it refuses (FR-140 does this for `Decimal`). FR-142-AC-31 and TC-981 RV-09 rely on it for `Rational[8, 7; 1, 12]`. Fix: state the rule and its code in FR-142, or add it to FR-149. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:580-583 |
| FND-004 | medium | The FR-154 first-defect list is not exhaustive. The statement requires each constraint's `appliesTo` to be the value type itself, but no row refuses a constraint whose `appliesTo` names another node. Nothing says what happens to a constraint declared elsewhere whose `appliesTo` names a Rational value type. A constraint with no `operands.value` or extra `operands` members is not placed explicitly in row 4. Fix: add a row, or fold these cases into rows 1 and 4, in a stated position. | spec/functional/type-model/FR-154-admit-domain-package-model.md:458-490 |
| FND-005 | medium | Choice 3 leaves a unit-bearing Rational member as a carried quantity. Under FR-142-AC-27 and AC-31, such a member is refused as an operand of `=`, `<` and every quantity operation, so `x.r = y.r` over a `Ratio` with `unit` `m` is `ill_typed` and has to be written through `convert<Quantity<m>>`. QSL FR-505 ("a unit-bearing Rational type is declared and compared"), and the legacy rational-with-unit scalar it succeeds, expect direct comparison. Fix: admit `=` and ordering between operands of one identical carried type, or state the convert-out requirement in FR-154 so the QSL side knows it. | spec/functional/type-model/FR-154-admit-domain-package-model.md:496-509; FR-142-AC-27; FR-142-AC-31 |
| FND-006 | low | The `rational_out_of_domain` catalog rows still describe only "an exact rational arithmetic result". They do not cover the new carried-quantity conversion use, or FR-478's existing conversion use. | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:98; proposals/quire-v1/definitions/native-diagnostics.md:111 |
| FND-007 | low | The follow-ups in the PR body name FCD, Contract IR and QSL FR-497. Of the FR-505 changes they name only the AC-7 unit spelling. FR-505 also needs these changes: row 3 must not refuse empty constraints; its "denominator bounds in `1 ..= i64::MAX` … refuses `malformed-declaration`" sentence must go to the bounds row; AC-1's "one `Ratio` type node" must become the structural `rational_range` node; "`Rational[...]` with that declared unit" must become `Quantity<u, Rational[..]>`; and the Dependencies note (refusing arithmetic with `operator-ineligible`) must change. Report these on QSL-1522, and do not edit them here. | QSL spec/functional/FR-505-admit-bounded-rational-value-types-and-refuse-inconsistent-bounds.md |

## Dispositions

| ID | Disposition | Resolution |
| --- | --- | --- |
| FND-001 | fixed | FR-322-AC-181..183 now say "a Rational value type with no `constraints`". TK-22 and TK-24 drop `lower`/`upper`. The TK oracle (`tests/checked_package_v2.rs`, `unmapped`) maps a Rational value type only when it carries all four flat members `numerator_min`, `numerator_max`, `denominator_min` and `denominator_max`. The TC-280 `Rat` fixture (`tests/model_member_types.rs`) is unbounded. New `TK-36` admits a bounded Rational value type as a payload and as a position (FR-322-AC-380). |
| FND-002 | fixed | `proposals/quire-v1/shared-grammar.md` now restricts `C` to `Int[..]`, `Decimal[..]` or `Rational[..]`, and a `Rational[..]` carrier takes no mode (`exact`). |
| FND-003 | fixed | FR-149 adds "Bounded rationals": the type is well formed iff `n1 <= n2` and `1 <= d1 <= d2`, any other declaration is `ill_typed`, and membership is decided by reduced numerator and denominator (FR-149-AC-27, TC-981 RV-10). FR-142 cites that rule, and FR-142-AC-31 and TC-981 RV-11 use `ill_typed` for a malformed carrier and `ill_typed`/`type-mismatch` for the bare `Rational`. |
| FND-004 | fixed | The FR-154 list now has six rows: keyword, `appliesTo`, repeat, missing, operands, bounds. Rows 1-3 scan in constraint order and rows 4-5 in keyword order. Row 5 covers a missing `operands.value` and extra `operands` members. A constraint outside the value type whose `appliesTo` names it is refused at its carrying node and `Constraint.appliesTo`. FR-154-AC-43..45 and TC-981 RV-02..RV-04 cover these. |
| FND-005 | fixed (semantic choice flagged in PR body) | Least-invasive option: no new operator. FR-154 and FR-142 now state that two unit-bearing Rational members are compared through the exact `convert<Quantity<u>>(e)` of each, with the example `convert<Quantity<m>>(x.r) = convert<Quantity<m>>(y.r)` (FR-142-AC-31, TC-981 RV-11). |
| FND-006 | fixed | The `rational_out_of_domain` rows in FR-271 and `native-diagnostics.md` and the prose paragraph now cover conversions into a `Rational[..]` domain, carried-quantity carriers included. |
| FND-007 | fixed | The PR body's follow-up list now names every QSL FR-505 amendment. |
