---
id: SR-714
title: "Spec review (integrity) — STD-110 catalog 1-draft.8 value-refusal codes"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@05f6e31144250e4d2ca5898b2f1284075ced297f; PR #162 diff against origin/main 0d53cf2: proposals/quire-v1/definitions/native-diagnostics.md, FR-270, FR-271, FR-272, TC-233, spec/complete-v1/tests.md; cross-checked against FR-044, FR-140, FR-141, FR-142, FR-146, FR-147, FR-148, TC-191, TC-193, value-accounting.md"
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
