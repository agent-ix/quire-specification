---
id: SR-2455
title: "Criterion strength review of FR-331-AC-24 (certificate-rejected locus wire)"
type: SpecReview
analysis: criterion-strength
scope: "agent-ix/quire-specification@cff80d04fa88ecae0831be2ef2fb70adabed1c98 against origin/main a865694e5db4daeb292c83070ecc2f12e0c9656c; spec/objects/interfaces/FR-331-backend-provider-envelope.md (Certification paragraph, FR-331-AC-24); spec/test-cases/TC-431-backend-provider-wire-vectors.md (BP-11)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
---
# Criterion strength review of FR-331-AC-24

## Summary

Ticket: none resolved; no tracker id on the branch or the commit.

This was a manual judgment: Jev was not used. It covers only the new
acceptance criterion and the new SHALL statement.

- **FR-331-AC-24 can fail.**
  - It names exact positive values: the three parts and the indices `"0"`,
    `"41"` and `"18446744073709551615"`.
  - It names exact adverse values: an unknown part, `""`, `{}`, a proof step
    with an unknown part, a missing member or an extra member, and an index
    that is a JSON number, `"-1"`, `"18446744073709551616"` or `"041"`.
  - Its range clause is quantified over a stated sample set ("every digit
    position", "lengths up to 21 digits") and checked against a stated
    oracle.
  - A schema that admitted a leading zero, or `u64::MAX + 1`, or that
    rejected `u64::MAX`, fails it.
- **Adverse coverage of the statement.** Each class the paragraph excludes has
  a vector: an unknown query name, a non-canonical index, an out-of-range
  index, the wrong JSON type, and an object that is not closed.
- **The SHALL sentence** ("a reader SHALL refuse any other `at` before
  consumption") is a ubiquitous EARS statement that can be tested. The refusal
  code is left to FR-331's general "refuse before consumption" rule, as
  AC-18 to AC-23 already do.

Its consistency with FR-331's "for every core checker" and with the
dependent FRs is a separate defect, recorded in SR-2454 FND-001. The AC is
strong, but strong about a contract that contradicts other requirements.

## Verdict

**Clean.** FR-331-AC-24 is specific and can fail, and it has positive and
adverse evidence for each clause.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
