---
id: SR-14688
title: "gap analysis of STD-153 changed ACs against TC rows (QSpec PR #343)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@726b8cccbedda4546e5882c574464c487fc09fd1; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-340-checked-package-frame-location.md, spec/objects/interfaces/FR-342-checked-package-operation-anchor-body.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-280-model-owned-member-types.md, spec/test-cases/TC-281-model-reference-equality-and-field-projection.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---
# SR-14688: gap analysis of STD-153 (QSpec PR #343)

## Summary

Ticket: STD-153. Plan completion: not assessed. This is a manual mapping of
each changed AC to its TC rows. The PR is spec plus schema. The
`tc_233_diagnostic_schema_pairs_each_selection_and_type_cause_with_its_codes`
assertion for the three new pairings is listed as remaining work for the
code lane. The existing test passes against the changed schema.

- FR-322-AC-38 maps to TC-281 RE-12 (M2 owned by `acme/billing`).
- FR-340-AC-13 maps to TC-280 FF-06 (`(M2, "total")`).
- FR-342-AC-4 maps to TC-280 AN-06 (`(M2, "size")`).
- FR-342-AC-9 keeps its TC-233 and TC-280 rows. Its changed clause is only
  a rewording of the selection case.
- FR-322-AC-47 maps to the new TC-233 procedure sentence: "Require the
  Diagnostic schema to admit each of `declaration-form`, `expression-form`
  and `not-yet-implemented` with `unsupported_construct` and with no other
  code".

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-233 checks FR-322-AC-47 in two other places, and neither is extended. The procedure sentence "Validate a Diagnostic entry carrying each of `type-mismatch`, `missing-selection`, `unsupported-selection` and `wrong-selection-role` with every code of the schema's code enum (FR-322-AC-47)" does not list the three new causes. The Expected Results sentence "The Diagnostic schema admits `type-mismatch` only with ..." still ends at the selection causes. So the new clause has a procedure step but no expected result, and the AC-47 procedure is split across two sentences. Add the three causes to both. | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:190-192 |

## Verdict

Every changed AC has a TC row. The new FR-322-AC-47 clause has no
expected result in TC-233 (FND-001).

## Resolution

FND-001: the TC-233 FR-322-AC-47 procedure sentence lists `declaration-form`, `expression-form` and `not-yet-implemented` with the other causes, the separate sentence is removed, and the Expected Results sentence pairs the three causes only with `unsupported_construct`.
