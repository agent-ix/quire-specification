---
id: SR-714
title: "Spec review (integrity) — STD-110 catalog 1-draft.8 value-refusal codes"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #162 diff against origin/main 0d53cf2: proposals/quire-v1/definitions/native-diagnostics.md, FR-270, FR-271, FR-272, TC-233, spec/complete-v1/tests.md; cross-checked against FR-044, FR-140, FR-141, FR-142, FR-146, FR-147, FR-148, TC-191, TC-193, value-accounting.md"
review_set: subset
---
# Spec review (integrity) — STD-110 catalog 1-draft.8 value-refusal codes

## Summary

Ticket: STD-110. This is an integrity (consistency and completeness) pass over
the PR #162 diff. The native-diagnostics rows, the FR-272 rows, the FR-271
values rows, the FR-271 revision history, FR-270's revision cell, TC-233 and
the matrix row agree with each other. Each code's causes match in both tables,
and the revision label is used consistently. FR-272-AC-11 is the next free
FR-272 criterion and maps to TC-233.

Two gaps remain in how the new codes trace to the requirements that own them.

## Verdict

**Changes requested.** One medium finding and one low finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The revision paragraph cites an owning FR for eight of the ten codes, but not for `integer_out_of_domain` or `rational_out_of_domain`. `integer_out_of_domain` has an owner: FR-142's integer-target membership refusal (value-accounting.md:271-273). It should be cited. `rational_out_of_domain` has no runtime refusal in QSpec. FR-044-AC-6 refuses an exact rational result bound statically, at checking. The only runtime `Rational[..]` membership refusal is FR-148's, which already has `ieee_rational_out_of_domain`. Its only source is QSL `quire-exact` `Refusal::RationalOutOfDomain`. As written, the claim "none is ... a type error" has no owning requirement for this code. Fix: name the owning FR, or state where a runtime rational result-domain refusal arises. | proposals/quire-v1/definitions/native-diagnostics.md:207-215; spec/functional/foundation/FR-044-check-composed-numeric-definedness.md:49 |
| FND-002 | low | 1-draft.8 closes `ieee_nan_payload_not_representable` to `payload-exceeds-target` and `ieee_rational_out_of_domain` to `outside-domain`. FR-148, FR-146, TC-191, TC-193 and value-accounting.md still write `refused { code: ... }` with no cause. Sibling outcomes, such as `cardinality_out_of_bound` in FR-144, name the cause. Adding it keeps the catalog the single source that the ticket asks for. | spec/functional/expressions/FR-148-evaluate-ieee-floating-profiles.md:120-121,133-134; spec/functional/expressions/FR-146-check-total-pure-functions.md:248; proposals/quire-v1/definitions/value-accounting.md:321 |

## Dispositions

Transcribed verbatim from https://linear.app/agent-ix/issue/STD-110/catalog-1-draft8-codes-for-the-kernel-refusal-causes-qsl-cannot-record#comment-5602d42b

<!-- reviewer-dispositions repo=agent-ix/quire-specification visibility=private quoin=0.24.1 module=spec-artifacts-process@v0.26.0 id=SR-714 date=2026-09-26 -->

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6389506c: the catalog paragraph cites FR-142 for `integer_out_of_domain` (integer-target unit conversion) and FR-044 for both codes. FR-044 Behavior and value-accounting now state an evaluation-time refusal after normalization, before result-retain. That decision point already existed in value-accounting ("decide any FR-044 result-bound membership without a charge"), so the rule is not dangling, and `rational_out_of_domain` now has a real source: evaluating an expression that did not pass the static check (direct kernel evaluation). **But** the chosen outcome contradicts FR-145 and TC-190 Q12, which call the same FR-044 decision a located undefined outcome. See SR-719 FND-001. |
| FND-002 | fixed | 6389506c: FR-148 (x2), FR-146, TC-191 P10, TC-193 F25/F27 and value-accounting now write `cause: payload-exceeds-target` / `cause: outside-domain`. No bare `refused { code: ieee_* }` is left. |

+++ [reviewer data]

```yaml
dispositions:
  - fnd: FND-001
    outcome: fixed
    path: spec/functional/foundation/FR-044-check-composed-numeric-definedness.md
    lines: "40-48"
    after_excerpt: |-
      At evaluation, the evaluator decides a bounded integer or `Rational[..]`
      arithmetic result's membership in its result domain after normalization and
      before retaining it (`quire.value.accounting/v1`). A result outside that domain
      is refused with no value, as
      `refused { code: integer_out_of_domain, cause: outside-domain }` or
      `refused { code: rational_out_of_domain, cause: outside-domain }`,
      respectively. An expression this check admitted has that membership proved,
      so only evaluation of an expression that did not pass this check meets
      either refusal.
    note: "Owner now exists and matches value-accounting.md:209-217. The FR-142 citation is carried by value-accounting.md:276-278 and native-diagnostics.md:211, not by the FR-142 file itself. The outcome conflicts with FR-145:94-97 and TC-190 Q12 (located undefined); tracked as SR-719 FND-001."
  - fnd: FND-002
    outcome: fixed
    path: spec/functional/expressions/FR-148-evaluate-ieee-floating-profiles.md
    lines: "120-121,133-134"
    after_excerpt: |-
      `refused { code:
      ieee_nan_payload_not_representable, cause: payload-exceeds-target }` with no flags, even for a signaling
      ...
      `refused { code:
      ieee_rational_out_of_domain, cause: outside-domain }` after `ieee.exact-intermediate` and before
```

+++

