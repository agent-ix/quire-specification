---
id: SR-20261103145404
title: "QSL-1536 r1 gap analysis of PR #609 (FR-154-AC-42..44, FR-322-AC-380..382, FR-142-AC-27/AC-31, TC-981)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification; PR #609 head cf327420 vs merge-base b1feb484; FR-154-AC-42..44, FR-322-AC-380..382, FR-142-AC-27, FR-142-AC-31, TC-981 RV-01..RV-10, TC-187 U41; tests/checked_package_v2.rs (tc_280_quantity_carrier_vectors_are_consistent), proposals/checked-package-v2/quantity-carrier-vectors.json; cross-checked against QSL FR-505 and FR-497 (quire-spec-language origin/main fc091fb1e)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/TC-981
    type: references
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
---

# QSL-1536 r1 gap analysis of PR #609

## Summary

Ticket: QSL-1536. Plan completion: not assessed.

Each new AC traces to TC-981, and each TC-981 vector row traces to the AC it names:

- RV-01 to FR-154-AC-42
- RV-02 and RV-03 to FR-154-AC-43
- RV-04 and RV-05 to FR-154-AC-44, with RV-04 also tracing to FR-322-AC-381
- RV-06 to FR-322-AC-380
- RV-07 to FR-322-AC-381
- RV-08 to FR-322-AC-382
- RV-09 and RV-10 to FR-142-AC-31

FR-142-AC-27 still traces to TC-187 U41. QSpec holds no evaluator, so TC-981 is prose-only. No machine-readable vector backs it.

Every QSL FR-505 criterion has a QSpec counterpart:

- AC-1 maps to FR-154-AC-42 and FR-322-AC-380.
- AC-2 and AC-3 map to FR-154-AC-43.
- AC-4 maps to FR-154-AC-44 and FR-322-AC-380.
- AC-5 maps to FR-154-AC-44.
- AC-6 is QSL-only.
- AC-7 maps to FR-154-AC-44 and FR-322-AC-381.

## Verdict

**There are gaps.** The ACs cover the main paths of the five semantic choices. Three medium gaps remain, and each can be fixed in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The executable oracle encodes the pre-PR rule. `tc_280_quantity_carrier_vectors_are_consistent` decides `well_formed` with `matches!(carrier form, "integer_range" \| "decimal_range")`, so a `rational_range` carrier with `rounding` `exact`, which FR-322-AC-382 admits, would be judged refused. `quantity-carrier-vectors.json` has no `rational_range` carrier case, so FR-322-AC-382 and FR-142-AC-31 have no machine vector that QSL or the IR consumer can read. The PR body defers this. Fix in this PR: add CQ-W rows (an admitted rational carrier `exact`, a refused `nearest-even` one) and update the predicate. | tests/checked_package_v2.rs:21181-21186; proposals/checked-package-v2/quantity-carrier-vectors.json |
| FND-002 | medium | The first-defect order is stated but only partly tested. FR-154-AC-43 and RV-03 test row 1 against row 3, and row 2 against row 3. They do not test row 1 against row 2, row 3 against row 4, row 4 against row 5 (`maxNumerator` `1.5` with `minNumerator` `8`, the case that decides choice 2's placement), or the order between two bad operands (by keyword, not by constraint position). Add one RV-03 pair for each. | spec/functional/type-model/FR-154-admit-domain-package-model.md (FR-154-AC-43); spec/test-cases/TC-981-rational-value-types.md (RV-03) |
| FND-003 | medium | FR-154 refuses a `unit` member that is not an `ix://quire/unit/<name>` string with `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.unit`, but no AC or TC row exercises it. That is exactly the case QSL FR-505-AC-7's bare `unit` `m` would hit. Add bare `m`, a number and an `ix://quire/native/...` URI to FR-154-AC-44 and RV-04. | spec/functional/type-model/FR-154-admit-domain-package-model.md:496-509; spec/test-cases/TC-981-rational-value-types.md (RV-04) |
| FND-004 | low | These edges are untested: a parameter's own constraints (`Operation.params[i].constraints`, which the text names); a conversion refused below `n1` (`-4 m` into `Rational[-3, 7; 1, 12]`); a `maxDenominator` of `0` or a negative one; the operand `-9223372036854775808` admitted at the i64 edge; a supplied value at a unit-bearing member; and `incomplete` under a work limit at the rational membership check. | FR-154-AC-43; FR-154-AC-44; FR-142-AC-31 |

## Dispositions

| ID | Disposition | Resolution |
| --- | --- | --- |
| FND-001 | fixed | `quantity-carrier-vectors.json` gains node `rat_r` (`rational_range`), `q_rat` and `bad_mode_rational`, plus `CQ-W16`..`CQ-W19` and `CQ-E21`..`CQ-E26`. `tc_280_quantity_carrier_vectors_are_consistent` admits a `rational_range` carrier only with `rounding` `exact`, decides rational evaluations by reduced numerator and denominator against the bounds, and traces TC-981, FR-322-AC-382 and FR-142-AC-31. Not run locally: only 4.8G of disk was free, below the 15G threshold for builds. |
| FND-002 | fixed | FR-154-AC-44 and TC-981 RV-03 test the pairs 1 vs 3, 2 vs 3, 1 vs 2, 3 vs 4, 4 vs 5 and 5 vs 6, and two bad operands (keyword order). |
| FND-003 | fixed | FR-154-AC-46 and TC-981 RV-05 test `unit` `m`, `1` and `ix://quire/native/Rational`. |
| FND-004 | fixed | Parameter constraints (AC-45, RV-04); conversion of `-4 m` below `n1` (AC-31, `CQ-E25`); `maxDenominator` 0 and -1 (AC-43); operand `-9223372036854775808` admitted (AC-42); a value at a unit-bearing member (AC-47, RV-06); `incomplete` under work limit one (AC-31, `CQ-E26`). |
