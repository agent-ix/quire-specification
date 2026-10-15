---
id: SR-11173
title: "code review of quire-specification PR 256 (Decimal type-link operands in FR-323 and TC-278)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@56199f93d1674ccaa468eef27e14832a99b6e082; spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-278-replay-envelope-members.md; oracle quire-exact@d18927e src/numeric_type_equality.rs, quire-semantic-value@38040e55 src/declaration/membership.rs, quire-spec-language@875074389 qsl-semantics/src/value/supplied.rs"
review_set: subset
---

## Summary

Ticket: QSL-772. PR 256 changes two spec files and no code. This review checks
the new text against the code that implements it.

Names: the four new operand names match `SuppliedNumericOperand::as_str` for
`NumericTypeEndpoint::DecimalExpectedLower`, `DecimalExpectedUpper`,
`DecimalActualLower` and `DecimalActualUpper` exactly.

Order: quire-exact `decimal_equal` checks expected lower, expected upper,
actual lower, actual upper against the ceiling before any comparison. This
matches "expected, then actual, lower before upper".

Scale: `decimal_equal` compares `min_scale`, `max_scale` and `rounding` with
plain equality after the endpoint check. They are never measured against the
ceiling, so "a Decimal endpoint's scale is not a bounded operand" is true.

Charge order: the QSV test
`bounded_decimal_type_link_checks_four_endpoints_and_metadata` charges the
`TypeLinkComparison` event first (spent 2, budget 2), then returns
`NumericTypeSize` with the Decimal position. A budget of 1 stops on
`TypeLinkComparison` before any size check.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-323 item 6, FR-323-AC-16 and TC-278 RE-29 still say the bounded type-to-type equality helper and its QSV caller are not built, and RE-29 says the vector is not runnable. Both exist: quire-exact d18927e has `DecimalType::try_eq_bounded` (and the Integer and Rational forms), and QSV 38040e55 `admit_supplied_bounded_numeric` calls them on a charged `TypeLinkComparison` event with tests for all 16 endpoints. The PR edits these rows but keeps the claim. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:231-234, :444; spec/test-cases/TC-278-replay-envelope-members.md:95 |

## Verdict

The operand names, the count of four, the check order and the scale exclusion
all match the code. One stale status claim stays in the edited rows.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 4001c8ea0dd1b81994ed4d031eb04f30623c0986 |
